use crate::graphics::colors::Rgba;
use core::ffi::c_void;
use core::mem::{self, ManuallyDrop};
use broc_std::{BrocList, BrocStr};
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

#[repr(transparent)]
#[cfg(target_pointer_width = "64")] // on a 64-bit system, the tag fits in this pointer's spare 3 bits
pub struct BrocElem {
    entry: *mut BrocElemEntry,
}

impl BrocElem {
    #[cfg(target_pointer_width = "64")]
    pub fn tag(&self) -> BrocElemTag {
        // On a 64-bit system, the last 3 bits of the pointer store the tag
        unsafe { mem::transmute::<u8, BrocElemTag>((self.entry as u8) & 0b0000_0111) }
    }

    pub fn entry(&self) -> &BrocElemEntry {
        // On a 64-bit system, the last 3 bits of the pointer store the tag
        let cleared = self.entry as usize & !0b111;

        unsafe { &*(cleared as *const BrocElemEntry) }
    }
}

#[repr(u8)]
#[allow(unused)] // This is actually used, just via a mem::transmute from u8
#[derive(Debug, Clone, Copy)]
pub enum BrocElemTag {
    Button = 0,
    Col,
    Row,
    Text,
}

#[repr(C)]
#[derive(Clone)]
pub struct BrocButton {
    pub child: ManuallyDrop<BrocElem>,
    pub styles: ButtonStyles,
}

#[repr(C)]
#[derive(Clone)]
pub struct BrocRowOrCol {
    pub children: BrocList<BrocElem>,
}

impl Clone for BrocElem {
    fn clone(&self) -> Self {
        unsafe {
            match self.tag() {
                BrocElemTag::Button => Self {
                    entry: &mut BrocElemEntry {
                        button: (*self.entry).button.clone(),
                    },
                },
                BrocElemTag::Text => Self {
                    entry: &mut BrocElemEntry {
                        text: (*self.entry).text.clone(),
                    },
                },
                BrocElemTag::Col | BrocElemTag::Row => Self {
                    entry: &mut BrocElemEntry {
                        row_or_col: (*self.entry).row_or_col.clone(),
                    },
                },
            }
        }
    }
}

impl Drop for BrocElem {
    fn drop(&mut self) {
        unsafe {
            match self.tag() {
                BrocElemTag::Button => mem::drop(ManuallyDrop::take(&mut (*self.entry).button)),
                BrocElemTag::Text => mem::drop(ManuallyDrop::take(&mut (*self.entry).text)),
                BrocElemTag::Col | BrocElemTag::Row => {
                    mem::drop(ManuallyDrop::take(&mut (*self.entry).row_or_col))
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ButtonStyles {
    pub bg_color: Rgba,
    pub border_color: Rgba,
    pub border_width: f32,
    pub text_color: Rgba,
}

#[repr(C)]
pub union BrocElemEntry {
    pub button: ManuallyDrop<BrocButton>,
    pub text: ManuallyDrop<BrocStr>,
    pub row_or_col: ManuallyDrop<BrocRowOrCol>,
}
