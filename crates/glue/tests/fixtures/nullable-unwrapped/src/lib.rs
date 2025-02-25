mod test_glue;

use indoc::indoc;
use test_glue::StrConsList;

extern "C" {
    #[link_name = "broc__mainForHost_1_exposed_generic"]
    fn broc_main(_: *mut StrConsList);
}

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    use std::cmp::Ordering;
    use std::collections::hash_set::HashSet;

    let tag_union = test_glue::mainForHost(());

    // Verify that it has all the expected traits.

    assert!(tag_union == tag_union); // PartialEq

    assert!(tag_union.clone() == tag_union.clone()); // Clone

    assert!(tag_union.partial_cmp(&tag_union) == Some(Ordering::Equal)); // PartialOrd
    assert!(tag_union.cmp(&tag_union) == Ordering::Equal); // Ord

    print!(
        indoc!(
            r#"
                tag_union was: {:?}
                `Cons "small str" Nil` is: {:?}
                `Nil` is: {:?}
            "#
        ),
        tag_union,
        StrConsList::Cons("small str".into(), StrConsList::Nil()),
        StrConsList::Nil(),
    ); // Debug

    let mut set = HashSet::new();

    set.insert(tag_union.clone()); // Eq, Hash
    set.insert(tag_union);

    assert_eq!(set.len(), 1);

    // Exit code
    0
}

// Externs required by broc_std and by the Broc app

use core::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn broc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn broc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn broc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
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
