//! Generates code needed for platform hosts to communicate with Broc apps.
//! This tool is not necessary for writing a platform in another language,
//! however, it's a great convenience! Currently supports Rust platforms, and
//! the plan is to support any language via a plugin model.
pub mod enums;
pub mod load;
pub mod broc_type;
pub mod rust_glue;
pub mod structs;
pub mod types;

#[rustfmt::skip]
pub mod glue;

pub use load::generate;

// required because we use broc_std here
mod broc_externs {
    use core::ffi::c_void;

    /// # Safety
    /// This just delegates to libc::malloc, so it's equally safe.
    #[no_mangle]
    pub unsafe extern "C" fn broc_alloc(size: usize, _alignment: u32) -> *mut c_void {
        libc::malloc(size)
    }

    /// # Safety
    /// This just delegates to libc::realloc, so it's equally safe.
    #[no_mangle]
    pub unsafe extern "C" fn broc_realloc(
        c_ptr: *mut c_void,
        new_size: usize,
        _old_size: usize,
        _alignment: u32,
    ) -> *mut c_void {
        libc::realloc(c_ptr, new_size)
    }

    /// # Safety
    /// This just delegates to libc::free, so it's equally safe.
    #[no_mangle]
    pub unsafe extern "C" fn broc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
        libc::free(c_ptr)
    }
}
