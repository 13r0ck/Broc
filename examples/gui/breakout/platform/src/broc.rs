use crate::graphics::colors::Rgba;
use core::alloc::Layout;
use core::ffi::c_void;
use core::mem::{self, ManuallyDrop};
use broc_std::{BrocList, BrocStr};
use std::ffi::CStr;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::time::Duration;
use winit::event::VirtualKeyCode;

extern "C" {
    // program

    // #[link_name = "broc__programForHost_1_exposed_generic"]
    // fn broc_program();

    // #[link_name = "broc__programForHost_1_exposed_size"]
    // fn broc_program_size() -> i64;

    // init

    #[link_name = "broc__programForHost_0_caller"]
    fn call_init(size: *const Bounds, closure_data: *const u8, output: *mut Model);

    #[link_name = "broc__programForHost_0_size"]
    fn init_size() -> i64;

    #[link_name = "broc__programForHost_0_result_size"]
    fn init_result_size() -> i64;

    // update

    #[link_name = "broc__programForHost_1_caller"]
    fn call_update(
        model: *const Model,
        event: *const BrocEvent,
        closure_data: *const u8,
        output: *mut Model,
    );

    #[link_name = "broc__programForHost_1_size"]
    fn update_size() -> i64;

    #[link_name = "broc__programForHost_1_result_size"]
    fn update_result_size() -> i64;

    // render

    #[link_name = "broc__programForHost_2_caller"]
    fn call_render(model: *const Model, closure_data: *const u8, output: *mut BrocList<BrocElem>);

    #[link_name = "broc__programForHost_2_size"]
    fn broc_render_size() -> i64;
}

#[repr(C)]
pub union BrocEventEntry {
    pub key_down: BrocKeyCode,
    pub key_up: BrocKeyCode,
    pub resize: Bounds,
    pub tick: [u8; 16], // u128 is unsupported in repr(C)
}

#[repr(u8)]
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrocEventTag {
    KeyDown = 0,
    KeyUp,
    Resize,
    Tick,
}

#[repr(C)]
#[cfg(target_pointer_width = "64")] // on a 64-bit system, the tag fits in this pointer's spare 3 bits
pub struct BrocEvent {
    entry: BrocEventEntry,
    tag: BrocEventTag,
}

impl Debug for BrocEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BrocEventTag::*;

        match self.tag() {
            KeyDown => unsafe { self.entry().key_down }.fmt(f),
            KeyUp => unsafe { self.entry().key_up }.fmt(f),
            Resize => unsafe { self.entry().resize }.fmt(f),
            Tick => unsafe { self.entry().tick }.fmt(f),
        }
    }
}

impl BrocEvent {
    #[cfg(target_pointer_width = "64")]
    pub fn tag(&self) -> BrocEventTag {
        self.tag
    }

    pub fn entry(&self) -> &BrocEventEntry {
        &self.entry
    }

    #[allow(non_snake_case)]
    pub fn Resize(size: Bounds) -> Self {
        Self {
            tag: BrocEventTag::Resize,
            entry: BrocEventEntry { resize: size },
        }
    }

    #[allow(non_snake_case)]
    pub fn KeyDown(keycode: BrocKeyCode) -> Self {
        Self {
            tag: BrocEventTag::KeyDown,
            entry: BrocEventEntry { key_down: keycode },
        }
    }

    #[allow(non_snake_case)]
    pub fn KeyUp(keycode: BrocKeyCode) -> Self {
        Self {
            tag: BrocEventTag::KeyUp,
            entry: BrocEventEntry { key_up: keycode },
        }
    }

    #[allow(non_snake_case)]
    pub fn Tick(duration: Duration) -> Self {
        Self {
            tag: BrocEventTag::Tick,
            entry: BrocEventEntry {
                tick: duration.as_nanos().to_ne_bytes(),
            },
        }
    }
}

#[repr(u8)]
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrocKeyCode {
    Down = 0,
    Left,
    Other,
    Right,
    Up,
}

impl From<VirtualKeyCode> for BrocKeyCode {
    fn from(keycode: VirtualKeyCode) -> Self {
        use VirtualKeyCode::*;

        match keycode {
            Left => BrocKeyCode::Left,
            Right => BrocKeyCode::Right,
            Up => BrocKeyCode::Up,
            Down => BrocKeyCode::Down,
            _ => BrocKeyCode::Other,
        }
    }
}

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ElemId(*const BrocElemEntry);

#[repr(C)]
pub union BrocElemEntry {
    pub rect: ManuallyDrop<BrocRect>,
    pub text: ManuallyDrop<BrocText>,
}

#[repr(u8)]
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrocElemTag {
    Rect = 0,
    Text = 1,
}

#[repr(C)]
#[cfg(target_pointer_width = "64")] // on a 64-bit system, the tag fits in this pointer's spare 3 bits
pub struct BrocElem {
    entry: BrocElemEntry,
    tag: BrocElemTag,
}

impl Debug for BrocElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BrocElemTag::*;

        match self.tag() {
            Rect => unsafe { &*self.entry().rect }.fmt(f),
            Text => unsafe { &*self.entry().text }.fmt(f),
        }
    }
}

impl BrocElem {
    #[cfg(target_pointer_width = "64")]
    pub fn tag(&self) -> BrocElemTag {
        self.tag
    }

    #[allow(unused)]
    pub fn entry(&self) -> &BrocElemEntry {
        &self.entry
    }

    #[allow(unused)]
    pub fn rect(styles: ButtonStyles) -> BrocElem {
        todo!("restore rect() method")
        // let rect = BrocRect { styles };
        // let entry = BrocElemEntry {
        //     rect: ManuallyDrop::new(rect),
        // };

        // Self::elem_from_tag(entry, BrocElemTag::Rect)
    }

    #[allow(unused)]
    pub fn text<T: Into<BrocStr>>(into_broc_str: T) -> BrocElem {
        todo!("TODO restore text method")
        // let entry = BrocElemEntry {
        //     text: ManuallyDrop::new(into_broc_str.into()),
        // };

        // Self::elem_from_tag(entry, BrocElemTag::Text)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BrocRect {
    pub color: Rgba,

    // These must be in this order for alphabetization!
    pub height: f32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BrocText {
    pub text: BrocStr,
    pub color: Rgba,
    pub left: f32,
    pub size: f32,
    pub top: f32,
}

impl Clone for BrocElem {
    fn clone(&self) -> Self {
        unsafe {
            match self.tag() {
                BrocElemTag::Rect => Self {
                    tag: BrocElemTag::Rect,
                    entry: BrocElemEntry {
                        rect: self.entry.rect.clone(),
                    },
                },
                BrocElemTag::Text => Self {
                    tag: BrocElemTag::Text,
                    entry: BrocElemEntry {
                        text: self.entry.text.clone(),
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
                BrocElemTag::Rect => mem::drop(ManuallyDrop::take(&mut self.entry.rect)),
                BrocElemTag::Text => mem::drop(ManuallyDrop::take(&mut self.entry.text)),
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ButtonStyles {
    pub bg_color: Rgba,
    pub border_color: Rgba,
    pub border_width: f32,
    pub text_color: Rgba,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Bounds {
    pub height: f32,
    pub width: f32,
}

type Model = c_void;

/// Call the app's init function, then render and return that result
pub fn init_and_render(bounds: Bounds) -> (*const Model, BrocList<BrocElem>) {
    let closure_data_buf;
    let closure_layout;

    // Call init to get the initial model
    let model = unsafe {
        let ret_val_layout = Layout::array::<u8>(init_result_size() as usize).unwrap();

        // TODO allocate on the stack if it's under a certain size
        let ret_val_buf = std::alloc::alloc(ret_val_layout) as *mut Model;

        closure_layout = Layout::array::<u8>(init_size() as usize).unwrap();

        // TODO allocate on the stack if it's under a certain size
        closure_data_buf = std::alloc::alloc(closure_layout);

        call_init(&bounds, closure_data_buf, ret_val_buf);

        ret_val_buf
    };

    // Call render passing the model to get the initial Elems
    let elems = unsafe {
        let mut ret_val: MaybeUninit<BrocList<BrocElem>> = MaybeUninit::uninit();

        // Reuse the buffer from the previous closure if possible
        let closure_data_buf =
            std::alloc::realloc(closure_data_buf, closure_layout, broc_render_size() as usize);

        call_render(model, closure_data_buf, ret_val.as_mut_ptr());

        std::alloc::dealloc(closure_data_buf, closure_layout);

        ret_val.assume_init()
    };

    (model, elems)
}

/// Call the app's update function, then render and return that result
pub fn update(model: *const Model, event: BrocEvent) -> *const Model {
    let closure_data_buf;
    let closure_layout;

    // Call update to get the new model
    unsafe {
        let ret_val_layout = Layout::array::<u8>(update_result_size() as usize).unwrap();

        // TODO allocate on the stack if it's under a certain size
        let ret_val_buf = std::alloc::alloc(ret_val_layout) as *mut Model;

        closure_layout = Layout::array::<u8>(update_size() as usize).unwrap();

        // TODO allocate on the stack if it's under a certain size
        closure_data_buf = std::alloc::alloc(closure_layout);

        call_update(model, &event, closure_data_buf, ret_val_buf);

        ret_val_buf
    }
}

/// Call the app's update function, then render and return that result
pub fn update_and_render(model: *const Model, event: BrocEvent) -> (*const Model, BrocList<BrocElem>) {
    let closure_data_buf;
    let closure_layout;

    // Call update to get the new model
    let model = unsafe {
        let ret_val_layout = Layout::array::<u8>(update_result_size() as usize).unwrap();

        // TODO allocate on the stack if it's under a certain size
        let ret_val_buf = std::alloc::alloc(ret_val_layout) as *mut Model;

        closure_layout = Layout::array::<u8>(update_size() as usize).unwrap();

        // TODO allocate on the stack if it's under a certain size
        closure_data_buf = std::alloc::alloc(closure_layout);

        call_update(model, &event, closure_data_buf, ret_val_buf);

        ret_val_buf
    };

    // Call render passing the model to get the initial Elems
    let elems = unsafe {
        let mut ret_val: MaybeUninit<BrocList<BrocElem>> = MaybeUninit::uninit();

        // Reuse the buffer from the previous closure if possible
        let closure_data_buf =
            std::alloc::realloc(closure_data_buf, closure_layout, broc_render_size() as usize);

        call_render(model, closure_data_buf, ret_val.as_mut_ptr());

        std::alloc::dealloc(closure_data_buf, closure_layout);

        ret_val.assume_init()
    };

    (model, elems)
}
