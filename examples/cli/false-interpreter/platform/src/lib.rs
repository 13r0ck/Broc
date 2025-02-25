#![allow(non_snake_case)]

use core::ffi::c_void;
use core::mem::MaybeUninit;
use libc;
use broc_std::{BrocList, BrocStr};
use std::env;
use std::ffi::CStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::raw::c_char;

extern "C" {
    #[link_name = "broc__mainForHost_1_exposed_generic"]
    fn broc_main(output: *mut u8, args: &BrocStr);

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
pub unsafe extern "C" fn broc_panic(c_ptr: *mut c_void, tag_id: u32) {
    match tag_id {
        0 => {
            let slice = CStr::from_ptr(c_ptr as *const c_char);
            let string = slice.to_str().unwrap();
            eprintln!("Broc hit a panic: {}", string);
            std::process::exit(1);
        }
        _ => todo!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn broc_memcpy(dst: *mut c_void, src: *mut c_void, n: usize) -> *mut c_void {
    libc::memcpy(dst, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn broc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
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

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let arg = env::args()
        .nth(1)
        .expect("Please pass a .false file as a command-line argument to the false interpreter!");
    let arg = BrocStr::from(arg.as_str());

    let size = unsafe { broc_main_size() } as usize;

    unsafe {
        let buffer = broc_alloc(size, 1) as *mut u8;

        broc_main(buffer, &arg);

        // arg has been passed to broc now, and it assumes ownership.
        // so we must not touch its refcount now
        std::mem::forget(arg);

        let result = call_the_closure(buffer);

        broc_dealloc(buffer as _, 1);

        result
    };

    // Exit code
    0
}

unsafe fn call_the_closure(closure_data_ptr: *const u8) -> i64 {
    let size = size_Fx_result() as usize;
    let buffer = broc_alloc(size, 1) as *mut u8;

    call_Fx(
        // This flags pointer will never get dereferenced
        MaybeUninit::uninit().as_ptr(),
        closure_data_ptr as *const u8,
        buffer as *mut u8,
    );

    broc_dealloc(buffer as _, 1);
    0
}

#[no_mangle]
pub extern "C" fn broc_fx_getLine() -> BrocStr {
    let stdin = std::io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();

    BrocStr::from(line1.as_str())
}

#[no_mangle]
pub extern "C" fn broc_fx_getChar() -> u8 {
    let mut buffer = [0];

    if let Err(ioerr) = std::io::stdin().lock().read_exact(&mut buffer[..]) {
        if ioerr.kind() == std::io::ErrorKind::UnexpectedEof {
            u8::MAX
        } else {
            panic!("Got an unexpected error while reading char from stdin");
        }
    } else {
        buffer[0]
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_putLine(line: &BrocStr) {
    let string = line.as_str();
    println!("{}", string);
    let _ = std::io::stdout().lock().flush();
}

#[no_mangle]
pub extern "C" fn broc_fx_putRaw(line: &BrocStr) {
    let string = line.as_str();
    print!("{}", string);
    let _ = std::io::stdout().lock().flush();
}

#[no_mangle]
pub extern "C" fn broc_fx_getFileLine(br_ptr: *mut BufReader<File>) -> BrocStr {
    let br = unsafe { &mut *br_ptr };
    let mut line1 = String::default();

    br.read_line(&mut line1)
        .expect("Failed to read line from file");

    BrocStr::from(line1.as_str())
}

#[no_mangle]
pub extern "C" fn broc_fx_getFileBytes(br_ptr: *mut BufReader<File>) -> BrocList<u8> {
    let br = unsafe { &mut *br_ptr };
    let mut buffer = [0; 0x10 /* This is intentionally small to ensure correct implementation */];

    let count = br
        .read(&mut buffer[..])
        .expect("Failed to read bytes from file");

    BrocList::from_slice(&buffer[..count])
}

#[no_mangle]
pub extern "C" fn broc_fx_closeFile(br_ptr: *mut BufReader<File>) {
    unsafe {
        let boxed = Box::from_raw(br_ptr);
        drop(boxed)
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_openFile(name: &BrocStr) -> *mut BufReader<File> {
    let string = name.as_str();
    match File::open(string) {
        Ok(f) => {
            let br = BufReader::new(f);

            Box::into_raw(Box::new(br))
        }
        Err(_) => {
            panic!("unable to open file {:?}", name)
        }
    }
}

#[no_mangle]
pub extern "C" fn broc_fx_withFileOpen(_name: &BrocStr, _buffer: *const u8) {
    // TODO: figure out accepting a closure in an fx and passing data to it.
    // let f = File::open(name.as_str()).expect("Unable to open file");
    // let mut br = BufReader::new(f);

    // unsafe {
    //     let closure_data_ptr = buffer.offset(8);
    //     call_the_closure(closure_data_ptr);
    // }
}
