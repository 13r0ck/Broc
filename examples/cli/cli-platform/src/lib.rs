#![allow(non_snake_case)]

mod file_glue;
mod glue;

use core::alloc::Layout;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use glue::Metadata;
use broc_std::{BrocDict, BrocList, BrocResult, BrocStr};
use std::borrow::{Borrow, Cow};
use std::ffi::{ OsStr};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use file_glue::ReadErr;
use file_glue::WriteErr;

extern "C" {
    #[link_name = "broc__mainForHost_1_exposed_generic"]
    fn broc_main(output: *mut u8);

    #[link_name = "broc__mainForHost_1_exposed_size"]
    fn broc_main_size() -> i64;

    #[link_name = "broc__mainForHost_0_caller"]
    fn call_Fx(flags: *const u8, closure_data: *const u8, output: *mut u8);

    #[allow(dead_code)]
    #[link_name = "broc__mainForHost_0_size"]
    fn size_Fx() -> i64;

    #[link_name = "broc__mainForHost_0_result_size"]
    fn size_Fx_result() -> i64;
}

#[no_mangle]
pub unsafe extern "C" fn broc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn broc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    libc::realloc(c_ptr, new_size)
}

#[no_mangle]
pub unsafe extern "C" fn broc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    libc::free(c_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn broc_panic(msg: &BrocStr, tag_id: u32) {
    match tag_id {
        0 => {
            eprintln!("Broc crashed with:\n\n\t{}\n", msg.as_str());

            print_backtrace();
            std::process::exit(1);
        }
        1 => {
            eprintln!("The program crashed with:\n\n\t{}\n", msg.as_str());

            print_backtrace();
            std::process::exit(1);
        }
        _ => todo!(),
    }
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn broc_getppid() -> libc::pid_t {
    libc::getppid()
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn broc_mmap(
    addr: *mut libc::c_void,
    len: libc::size_t,
    prot: libc::c_int,
    flags: libc::c_int,
    fd: libc::c_int,
    offset: libc::off_t,
) -> *mut libc::c_void {
    libc::mmap(addr, len, prot, flags, fd, offset)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn broc_shm_open(
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::mode_t,
) -> libc::c_int {
    libc::shm_open(name, oflag, mode as libc::c_uint)
}

fn print_backtrace() {
    eprintln!("Here is the call stack that led to the crash:\n");

    let mut entries = Vec::new();

    #[derive(Default)]
    struct Entry {
        pub fn_name: String,
        pub filename: Option<String>,
        pub line: Option<u32>,
        pub col: Option<u32>,
    }

    backtrace::trace(|frame| {
        backtrace::resolve_frame(frame, |symbol| {
            if let Some(fn_name) = symbol.name() {
                let fn_name = fn_name.to_string();

                if should_show_in_backtrace(&fn_name) {
                    let mut entry: Entry = Default::default();

                    entry.fn_name = format_fn_name(&fn_name);

                    if let Some(path) = symbol.filename() {
                        entry.filename = Some(path.to_string_lossy().into_owned());
                    };

                    entry.line = symbol.lineno();
                    entry.col = symbol.colno();

                    entries.push(entry);
                }
            } else {
                entries.push(Entry {
                    fn_name: "???".to_string(),
                    ..Default::default()
                });
            }
        });

        true // keep going to the next frame
    });

    for entry in entries {
        eprintln!("\t{}", entry.fn_name);

        if let Some(filename) = entry.filename {
            eprintln!("\t\t{filename}");
        }
    }

    eprintln!("\nOptimizations can make this list inaccurate! If it looks wrong, try running without `--optimize` and with `--linker=legacy`\n");
}

fn should_show_in_backtrace(fn_name: &str) -> bool {
    let is_from_rust = fn_name.contains("::");
    let is_host_fn = fn_name.starts_with("broc_panic")
        || fn_name.starts_with("_Effect_effect")
        || fn_name.starts_with("_broc__")
        || fn_name.starts_with("rust_main")
        || fn_name == "_main";

    !is_from_rust && !is_host_fn
}

fn format_fn_name(fn_name: &str) -> String {
    // e.g. convert "_Num_sub_a0c29024d3ec6e3a16e414af99885fbb44fa6182331a70ab4ca0886f93bad5"
    // to ["Num", "sub", "a0c29024d3ec6e3a16e414af99885fbb44fa6182331a70ab4ca0886f93bad5"]
    let mut pieces_iter = fn_name.split("_");

    if let (_, Some(module_name), Some(name)) =
        (pieces_iter.next(), pieces_iter.next(), pieces_iter.next())
    {
        display_broc_fn(module_name, name)
    } else {
        "???".to_string()
    }
}

fn display_broc_fn(module_name: &str, fn_name: &str) -> String {
    let module_name = if module_name == "#UserApp" {
        "app"
    } else {
        module_name
    };

    let fn_name = if fn_name.parse::<u64>().is_ok() {
        "(anonymous function)"
    } else {
        fn_name
    };

    format!("\u{001B}[36m{module_name}\u{001B}[39m.{fn_name}")
}

#[no_mangle]
pub unsafe extern "C" fn broc_memcpy(dst: *mut c_void, src: *mut c_void, n: usize) -> *mut c_void {
    libc::memcpy(dst, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn broc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let size = unsafe { broc_main_size() } as usize;
    let layout = Layout::array::<u8>(size).unwrap();

    unsafe {
        // TODO allocate on the stack if it's under a certain size
        let buffer = std::alloc::alloc(layout);

        broc_main(buffer);

        call_the_closure(buffer);

        std::alloc::dealloc(buffer, layout);
    }
}

unsafe fn call_the_closure(closure_data_ptr: *const u8) -> u8 {
    let size = size_Fx_result() as usize;
    let layout = Layout::array::<u8>(size).unwrap();
    let buffer = std::alloc::alloc(layout) as *mut u8;

    call_Fx(
        // This flags pointer will never get dereferenced
        MaybeUninit::uninit().as_ptr(),
        closure_data_ptr as *const u8,
        buffer as *mut u8,
    );

    std::alloc::dealloc(buffer, layout);

    // TODO return the u8 exit code returned by the Fx closure
    0
}

#[no_mangle]
pub extern "C" fn broc_fx_envDict() -> BrocDict<BrocStr, BrocStr> {
    // TODO: can we be more efficient about reusing the String's memory for BrocStr?
    std::env::vars_os()
        .map(|(key, val)| {
            (
                BrocStr::from(key.to_string_lossy().borrow()),
                BrocStr::from(val.to_string_lossy().borrow()),
            )
        })
        .collect()
}

#[no_mangle]
pub extern "C" fn broc_fx_args() -> BrocList<BrocStr> {
    // TODO: can we be more efficient about reusing the String's memory for BrocStr?
    std::env::args_os()
        .map(|os_str| BrocStr::from(os_str.to_string_lossy().borrow()))
        .collect()
}

#[no_mangle]
pub extern "C" fn broc_fx_envVar(broc_str: &BrocStr) -> BrocResult<BrocStr, ()> {
    // TODO: can we be more efficient about reusing the String's memory for BrocStr?
    match std::env::var_os(broc_str.as_str()) {
        Some(os_str) => BrocResult::ok(BrocStr::from(os_str.to_string_lossy().borrow())),
        None => BrocResult::err(()),
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_setCwd(broc_path: &BrocList<u8>) -> BrocResult<(), ()> {
    match std::env::set_current_dir(path_from_broc_path(broc_path)) {
        Ok(()) => BrocResult::ok(()),
        Err(_) => BrocResult::err(()),
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_processExit(exit_code: u8) {
    std::process::exit(exit_code as i32);
}

#[no_mangle]
pub extern "C" fn broc_fx_exePath(_broc_str: &BrocStr) -> BrocResult<BrocList<u8>, ()> {
    match std::env::current_exe() {
        Ok(path_buf) => BrocResult::ok(os_str_to_broc_path(path_buf.as_path().as_os_str())),
        Err(_) => BrocResult::err(()),
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_stdinLine() -> BrocStr {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();

    BrocStr::from(line1.as_str())
}

#[no_mangle]
pub extern "C" fn broc_fx_stdoutLine(line: &BrocStr) {
    let string = line.as_str();
    println!("{}", string);
}

#[no_mangle]
pub extern "C" fn broc_fx_stdoutWrite(text: &BrocStr) {
    let string = text.as_str();
    print!("{}", string);
    std::io::stdout().flush().unwrap();
}

#[no_mangle]
pub extern "C" fn broc_fx_stderrLine(line: &BrocStr) {
    let string = line.as_str();
    eprintln!("{}", string);
}

#[no_mangle]
pub extern "C" fn broc_fx_stderrWrite(text: &BrocStr) {
    let string = text.as_str();
    eprint!("{}", string);
    std::io::stderr().flush().unwrap();
}

// #[no_mangle]
// pub extern "C" fn broc_fx_fileWriteUtf8(
//     broc_path: &BrocList<u8>,
//     broc_string: &BrocStr,
//     // ) -> BrocResult<(), WriteErr> {
// ) -> (u8, u8) {
//     let _ = write_slice(broc_path, broc_string.as_str().as_bytes());

//     (255, 255)
// }

// #[no_mangle]
// pub extern "C" fn broc_fx_fileWriteUtf8(broc_path: &BrocList<u8>, broc_string: &BrocStr) -> Fail {
//     write_slice2(broc_path, broc_string.as_str().as_bytes())
// }
#[no_mangle]
pub extern "C" fn broc_fx_fileWriteUtf8(
    broc_path: &BrocList<u8>,
    broc_str: &BrocStr,
) -> BrocResult<(), WriteErr> {
    write_slice(broc_path, broc_str.as_str().as_bytes())
}

#[no_mangle]
pub extern "C" fn broc_fx_fileWriteBytes(
    broc_path: &BrocList<u8>,
    broc_bytes: &BrocList<u8>,
) -> BrocResult<(), WriteErr> {
    write_slice(broc_path, broc_bytes.as_slice())
}

fn write_slice(broc_path: &BrocList<u8>, bytes: &[u8]) -> BrocResult<(), WriteErr> {
    match File::create(path_from_broc_path(broc_path)) {
        Ok(mut file) => match file.write_all(bytes) {
            Ok(()) => BrocResult::ok(()),
            Err(_) => {
                todo!("Report a file write error");
            }
        },
        Err(_) => {
            todo!("Report a file open error");
        }
    }
}

#[cfg(target_family = "unix")]
fn path_from_broc_path(bytes: &BrocList<u8>) -> Cow<'_, Path> {
    use std::os::unix::ffi::OsStrExt;
    let os_str = OsStr::from_bytes(bytes.as_slice());
    Cow::Borrowed(Path::new(os_str))
}

#[cfg(target_family = "windows")]
fn path_from_broc_path(bytes: &BrocList<u8>) -> Cow<'_, Path> {
    use std::os::windows::ffi::OsStringExt;

    let bytes = bytes.as_slice();
    assert_eq!(bytes.len() % 2, 0);
    let characters: &[u16] =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len() / 2) };

    let os_string = std::ffi::OsString::from_wide(characters);

    Cow::Owned(std::path::PathBuf::from(os_string))
}

#[no_mangle]
pub extern "C" fn broc_fx_fileReadBytes(broc_path: &BrocList<u8>) -> BrocResult<BrocList<u8>, ReadErr> {
    use std::io::Read;

    let mut bytes = Vec::new();

    match File::open(path_from_broc_path(broc_path)) {
        Ok(mut file) => match file.read_to_end(&mut bytes) {
            Ok(_bytes_read) => BrocResult::ok(BrocList::from(bytes.as_slice())),
            Err(_) => {
                todo!("Report a file write error");
            }
        },
        Err(_) => {
            todo!("Report a file open error");
        }
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_fileDelete(broc_path: &BrocList<u8>) -> BrocResult<(), ReadErr> {
    match std::fs::remove_file(path_from_broc_path(broc_path)) {
        Ok(()) => BrocResult::ok(()),
        Err(_) => {
            todo!("Report a file write error");
        }
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_cwd() -> BrocList<u8> {
    // TODO instead, call getcwd on UNIX and GetCurrentDirectory on Windows
    match std::env::current_dir() {
        Ok(path_buf) => os_str_to_broc_path(path_buf.into_os_string().as_os_str()),
        Err(_) => {
            // Default to empty path
            BrocList::empty()
        }
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_dirList(
    // TODO: this BrocResult should use Dir.WriteErr - but right now it's File.WriteErr
    // because glue doesn't have Dir.WriteErr yet.
    broc_path: &BrocList<u8>,
) -> BrocResult<BrocList<BrocList<u8>>, WriteErr> {
    println!("Dir.list...");
    match std::fs::read_dir(path_from_broc_path(broc_path)) {
        Ok(dir_entries) => BrocResult::ok(
            dir_entries
                .map(|opt_dir_entry| match opt_dir_entry {
                    Ok(entry) => os_str_to_broc_path(entry.path().into_os_string().as_os_str()),
                    Err(_) => {
                        todo!("handle dir_entry path didn't resolve")
                    }
                })
                .collect::<BrocList<BrocList<u8>>>(),
        ),
        Err(_) => {
            todo!("handle Dir.list error");
        }
    }
}

#[cfg(target_family = "unix")]
fn os_str_to_broc_path(os_str: &OsStr) -> BrocList<u8> {
    use std::os::unix::ffi::OsStrExt;

    BrocList::from(os_str.as_bytes())
}

#[cfg(target_family = "windows")]
fn os_str_to_broc_path(os_str: &OsStr) -> BrocList<u8> {
    use std::os::windows::ffi::OsStrExt;

    let bytes: Vec<_> = os_str.encode_wide().flat_map(|c| c.to_be_bytes()).collect();

    BrocList::from(bytes.as_slice())
}

#[no_mangle]
pub extern "C" fn broc_fx_sendRequest(broc_request: &glue::Request) -> glue::Response {
    let mut builder = reqwest::blocking::ClientBuilder::new();

    if broc_request.timeout.discriminant() == glue::discriminant_TimeoutConfig::TimeoutMilliseconds {
        let ms: &u64 = unsafe { broc_request.timeout.as_TimeoutMilliseconds() };
        builder = builder.timeout(Duration::from_millis(*ms));
    }

    let client = match builder.build() {
        Ok(c) => c,
        Err(_) => {
            return glue::Response::NetworkError; // TLS backend cannot be initialized
        }
    };

    let method = match broc_request.method {
        glue::Method::Connect => reqwest::Method::CONNECT,
        glue::Method::Delete => reqwest::Method::DELETE,
        glue::Method::Get => reqwest::Method::GET,
        glue::Method::Head => reqwest::Method::HEAD,
        glue::Method::Options => reqwest::Method::OPTIONS,
        glue::Method::Patch => reqwest::Method::PATCH,
        glue::Method::Post => reqwest::Method::POST,
        glue::Method::Put => reqwest::Method::PUT,
        glue::Method::Trace => reqwest::Method::TRACE,
    };

    let url = broc_request.url.as_str();

    let mut req_builder = client.request(method, url);
    for header in broc_request.headers.iter() {
        let (name, value) = unsafe { header.as_Header() };
        req_builder = req_builder.header(name.as_str(), value.as_str());
    }
    if broc_request.body.discriminant() == glue::discriminant_Body::Body {
        let (mime_type_tag, body_byte_list) = unsafe { broc_request.body.as_Body() };
        let mime_type_str: &BrocStr = unsafe { mime_type_tag.as_MimeType() };

        req_builder = req_builder.header("Content-Type", mime_type_str.as_str());
        req_builder = req_builder.body(body_byte_list.as_slice().to_vec());
    }

    let request = match req_builder.build() {
        Ok(req) => req,
        Err(err) => {
            return glue::Response::BadRequest(BrocStr::from(err.to_string().as_str()));
        }
    };

    match client.execute(request) {
        Ok(response) => {
            let status = response.status();
            let status_str = status.canonical_reason().unwrap_or_else(|| status.as_str());

            let headers_iter = response.headers().iter().map(|(name, value)| {
                glue::Header::Header(
                    BrocStr::from(name.as_str()),
                    BrocStr::from(value.to_str().unwrap_or_default()),
                )
            });

            let metadata = Metadata {
                headers: BrocList::from_iter(headers_iter),
                statusText: BrocStr::from(status_str),
                url: BrocStr::from(url),
                statusCode: status.as_u16(),
            };

            let bytes = response.bytes().unwrap_or_default();
            let body: BrocList<u8> = BrocList::from_iter(bytes.into_iter());

            if status.is_success() {
                glue::Response::GoodStatus(metadata, body)
            } else {
                glue::Response::BadStatus(metadata, body)
            }
        }
        Err(err) => {
            if err.is_timeout() {
                glue::Response::Timeout
            } else if err.is_request() {
                glue::Response::BadRequest(BrocStr::from(err.to_string().as_str()))
            } else {
                glue::Response::NetworkError
            }
        }
    }
}
