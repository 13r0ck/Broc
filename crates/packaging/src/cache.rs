#[cfg(not(target_family = "wasm"))]
use {
    crate::https::{self, PackageMetadata, Problem},
    broc_error_macros::internal_error,
    std::fs,
};
#[cfg(not(target_family = "wasm"))]
const MAX_DOWNLOAD_BYTES: u64 = 32 * 1_000_000_000; // GB

use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug)]
pub enum BrocCacheDir<'a> {
    /// Normal scenario: reading from the user's cache dir on disk
    Persistent(&'a Path),
    /// For build.rs and tests where we never want to be downloading anything - yell loudly if we try!
    Disallowed,
    /// For tests only; we don't want to write to the real cache during a test!
    #[cfg(test)]
    Temp(&'a tempfile::TempDir),
}

// Errors in case NixOS users try to use a dynamically linked platform
#[cfg(target_os = "linux")]
fn nixos_error_if_dynamic(url: &str, dest_dir: &Path) {
    let output = std::process::Command::new("uname")
        .arg("-a")
        .output()
        .expect("uname command failed to start");
    let running_nixos = String::from_utf8_lossy(&output.stdout).contains("NixOS");

    if running_nixos {
        // bash -c is used instead of plain ldd because process::Command escapes its arguments
        let ldd_output = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("ldd {}/linux-x86_64.rh*", dest_dir.display()))
            .output()
            .expect("ldd command failed to start");
        let is_dynamic = String::from_utf8_lossy(&ldd_output.stdout).contains("=>");

        if is_dynamic {
            eprintln!("The platform downloaded from the URL {url} is dynamically linked.\n\
                        Dynamically linked platforms can't be used on NixOS.\n\n\
                        You can:\n\n\t\
                            - Download the source of the platform and build it locally, like in this example:\n\t  \
                                https://github.com/roc-lang/broc/blob/main/examples/platform-switching/brocLovesRust.broc.\n\t  \
                                When building your broc application, you can use the flag `--prebuilt-platform=true` to prevent the platform from being rebuilt every time.\n\t  \
                                For some graphical platforms you may need to use https://github.com/guibou/nixGL.\n\n\t\
                            - Contact the author of the platform to ask them to statically link their platform.\n\t  \
                                musl can be used to prevent a dynamic dependency on the systems' libc.\n\t  \
                                If the platform is dynamically linked to GPU drivers, it can not be statically linked practically. Use the previous suggestion to build locally in this case.\n"
            );
            std::process::exit(1);
        }
    }
}

/// Accepts either a path to the Broc cache dir, or else a TempDir. If a TempDir, always download
/// into that dir. If the cache dir on the filesystem, then look into it to see if we already
/// have an entry for the given URL. If we do, return its info. If we don't already have it, then:
///
/// - Download and decompress the compressed tarball from the given URL
/// - Verify its bytes against the hash in the URL
/// - Extract the tarball's contents into the appropriate cache directory
///
/// Returns the path to the installed package (which will be in the cache dir somewhere), as well
/// as the requested root module filename (optionally specified via the URL fragment).
#[cfg(not(target_family = "wasm"))]
pub fn install_package<'a>(
    broc_cache_dir: BrocCacheDir<'_>,
    url: &'a str,
) -> Result<(PathBuf, Option<&'a str>), Problem> {
    let PackageMetadata {
        cache_subdir,
        content_hash,
        root_module_filename,
    } = PackageMetadata::try_from(url).map_err(Problem::InvalidUrl)?;

    match broc_cache_dir {
        BrocCacheDir::Persistent(cache_dir) => {
            // e.g. ~/.cache/broc/example.com/broc-packages/
            let parent_dir = cache_dir.join(cache_subdir);
            // e.g. ~/.cache/broc/example.com/broc-packages/jDRlAFAA3738vu3-vMpLUoyxtA86Z7CaZneoOKrihbE
            let dest_dir = parent_dir.join(content_hash);

            if dest_dir.exists() {
                // If the cache dir exists already, we assume it has the correct contents
                // (it's a cache, after all!) and return without downloading anything.
                //
                #[cfg(target_os = "linux")]
                {
                    nixos_error_if_dynamic(url, &dest_dir);
                }

                Ok((dest_dir, root_module_filename))
            } else {
                // Download into a tempdir; only move it to dest_dir if hash verification passes.
                println!(
                    "Downloading \u{001b}[36m{url}\u{001b}[0m\n    into {}\n",
                    cache_dir.display()
                );
                let tempdir = tempfile::tempdir().map_err(Problem::IoErr)?;
                let tempdir_path = tempdir.path();
                let downloaded_hash =
                    https::download_and_hash(url, tempdir_path, MAX_DOWNLOAD_BYTES)?;

                // Download the tarball into memory and verify it.
                // The tarball name is the hash of its contents.
                if downloaded_hash == content_hash {
                    // Now that we've verified the hash, rename the tempdir to the real dir.

                    // Create the destination dir's parent dir, since it may not exist yet.
                    fs::create_dir_all(parent_dir).map_err(Problem::IoErr)?;

                    // This rename should be super cheap if it succeeds - just an inode change.
                    if fs::rename(tempdir_path, &dest_dir).is_err() {
                        // If the rename failed, try a recursive copy -
                        // it could have failed due to std::io::ErrorKind::CrossesDevices
                        // (e.g. if the source an destination directories are on different disks)
                        // which as of this implementation is nightly-only
                        // https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.CrossesDevices                       match io_err.kind() {
                        // but if that's what happened, this should work!

                        // fs_extra::dir::copy needs the destination directory to exist already.
                        fs::create_dir(&dest_dir).map_err(Problem::IoErr)?;
                        fs_extra::dir::copy(
                            tempdir_path,
                            &dest_dir,
                            &fs_extra::dir::CopyOptions {
                                content_only: true,
                                ..Default::default()
                            },
                        )
                        .map_err(Problem::FsExtraErr)?;
                    }

                    #[cfg(target_os = "linux")]
                    {
                        nixos_error_if_dynamic(url, &dest_dir);
                    }

                    // The package's files are now in the cache. We're done!
                    Ok((dest_dir, root_module_filename))
                } else {
                    Err(Problem::InvalidContentHash {
                        expected: content_hash.to_string(),
                        actual: downloaded_hash,
                    })
                }
            }
        }
        BrocCacheDir::Disallowed => {
            internal_error!(
                "Tried to download a package ({:?}) via BrocCacheDir::Disallowed - which was explicitly used in order to disallow downloading packages in the current context!",
                url
            )
        }
        #[cfg(test)]
        BrocCacheDir::Temp(temp_dir) => Ok((temp_dir.path().to_path_buf(), None)),
    }
}

#[cfg(windows)]
// e.g. the "Broc" in %APPDATA%\\Broc
const ROC_CACHE_DIR_NAME: &str = "Broc";

#[cfg(not(windows))]
// e.g. the "broc" in ~/.cache/broc
const ROC_CACHE_DIR_NAME: &str = "broc";

/// This looks up environment variables, so it should ideally be called once and then cached!
///
/// Returns a path of the form cache_dir_path.join(ROC_CACHE_DIR_NAME).join("packages")
/// where cache_dir_path is:
/// - The XDG_CACHE_HOME environment varaible, if it's set.
/// - Otherwise, ~/.cache on UNIX and %APPDATA% on Windows.
///
/// ROC_CACHE_DIR_NAME is "broc" on UNIX and "Broc" on Windows.
///
/// So ~/.cache/broc will be typical on UNIX, and %APPDATA%\\Broc will be typical on Windows.
///
/// Returns None if XDG_CACHE_HOME is not set, and also we can't determine the home directory
/// (or if %APPDATA% is missing on Windows) on this system.
#[cfg(not(target_family = "wasm"))]
pub fn broc_cache_dir() -> PathBuf {
    use std::{env, process};

    const PACKAGES_DIR_NAME: &str = "packages";

    // Respect XDG, if the system appears to be using it.
    // https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    match env::var_os("XDG_CACHE_HOME") {
        Some(xdg_cache_home) => Path::new(&xdg_cache_home)
            .join(ROC_CACHE_DIR_NAME)
            .join(PACKAGES_DIR_NAME),
        None => {
            #[cfg(windows)]
            {
                // e.g. %APPDATA%\\Broc
                if let Some(appdata) =
                    // CSIDL_APPDATA is the same as APPDATA, according to:
                    // https://learn.microsoft.com/en-us/windows/deployment/usmt/usmt-recognized-environment-variables
                    env::var_os("APPDATA").or_else(|| env::var_os("CSIDL_APPDATA"))
                {
                    Path::new(&appdata)
                        .join(ROC_CACHE_DIR_NAME)
                        .join(PACKAGES_DIR_NAME)
                } else {
                    eprintln!("broc needs either the %APPDATA% or else the %XDG_CACHE_HOME% environment variables set. Please set one of these environment variables and re-run broc!");
                    process::exit(1);
                }
            }

            #[cfg(unix)]
            {
                // e.g. $HOME/.cache/broc
                if let Some(home) = env::var_os("HOME") {
                    Path::new(&home)
                        .join(".cache")
                        .join(ROC_CACHE_DIR_NAME)
                        .join(PACKAGES_DIR_NAME)
                } else {
                    eprintln!("broc needs either the $HOME or else the $XDG_CACHE_HOME environment variables set. Please set one of these environment variables and re-run broc!");
                    process::exit(1);
                }
            }
        }
    }
}

/// WASI doesn't have a home directory, so just make the cache dir in the current directory
/// https://github.com/WebAssembly/wasi-filesystem/issues/59
#[cfg(target_family = "wasm")]
pub fn broc_cache_dir() -> PathBuf {
    PathBuf::from(".cache").join(ROC_CACHE_DIR_NAME)
}
