//! Provides a binary that is only used for static build servers.
use clap::{Arg, Command};
use broc_docs::generate_docs_html;
use std::io;
use std::path::PathBuf;

pub const ROC_FILE: &str = "ROC_FILE";
const DEFAULT_ROC_FILENAME: &str = "main.broc";

fn main() -> io::Result<()> {
    let matches = Command::new("broc-docs")
        .about("Generate documentation for a Broc package")
        .arg(
            Arg::new(ROC_FILE)
                .multiple_values(true)
                .help("The package's main .broc file")
                .allow_invalid_utf8(true)
                .required(false)
                .default_value(DEFAULT_ROC_FILENAME),
        )
        .get_matches();

    // Populate broc_files
    generate_docs_html(PathBuf::from(matches.value_of_os(ROC_FILE).unwrap()));

    Ok(())
}

// These functions don't end up in the final Broc binary but Windows linker needs a definition inside the crate.
// On Windows, there seems to be less dead-code-elimination than on Linux or MacOS, or maybe it's done later.
#[cfg(windows)]
#[allow(unused_imports)]
use windows_broc_platform_functions::*;

#[cfg(windows)]
mod windows_broc_platform_functions {
    use core::ffi::c_void;

    /// # Safety
    /// The Broc application needs this.
    #[no_mangle]
    pub unsafe fn broc_alloc(size: usize, _alignment: u32) -> *mut c_void {
        libc::malloc(size)
    }

    /// # Safety
    /// The Broc application needs this.
    #[no_mangle]
    pub unsafe fn broc_realloc(
        c_ptr: *mut c_void,
        new_size: usize,
        _old_size: usize,
        _alignment: u32,
    ) -> *mut c_void {
        libc::realloc(c_ptr, new_size)
    }

    /// # Safety
    /// The Broc application needs this.
    #[no_mangle]
    pub unsafe fn broc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
        libc::free(c_ptr)
    }
}
