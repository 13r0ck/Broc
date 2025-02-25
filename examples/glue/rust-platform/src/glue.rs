// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `broc glue` CLI command

#![allow(unused_unsafe)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]

type Op_StderrWrite = broc_std::BrocStr;
type Op_StdoutWrite = broc_std::BrocStr;
type TODO_broc_function_69 = broc_std::BrocStr;
type TODO_broc_function_70 = broc_std::BrocStr;

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum discriminant_Op {
    Done = 0,
    StderrWrite = 1,
    StdoutWrite = 2,
}

impl core::fmt::Debug for discriminant_Op {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Done => f.write_str("discriminant_Op::Done"),
            Self::StderrWrite => f.write_str("discriminant_Op::StderrWrite"),
            Self::StdoutWrite => f.write_str("discriminant_Op::StdoutWrite"),
        }
    }
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(transparent)]
pub struct Op {
    pointer: *mut union_Op,
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(C)]
union union_Op {
    StderrWrite: core::mem::ManuallyDrop<Op_StderrWrite>,
    StdoutWrite: core::mem::ManuallyDrop<Op_StdoutWrite>,
    _sizer: [u8; 8],
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "x86_64"
))]
//TODO HAS CLOSURE 2
#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(C)]
pub struct BrocFunction_66 {
    pub closure_data: broc_std::BrocList<u8>,
}

impl BrocFunction_66 {
    pub fn force_thunk(mut self, arg_0: ()) -> Op {
        extern "C" {
            fn broc__mainForHost_0_caller(arg_0: &(), closure_data: *mut u8, output: *mut Op);
        }

        let mut output = std::mem::MaybeUninit::uninit();
        let ptr = self.closure_data.as_mut_ptr();
        unsafe { broc__mainForHost_0_caller(&arg_0, ptr, output.as_mut_ptr()) };
        unsafe { output.assume_init() }
    }
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[repr(C)]
pub struct BrocFunction_67 {
    pub closure_data: broc_std::BrocList<u8>,
}

impl BrocFunction_67 {
    pub fn force_thunk(mut self, arg_0: ()) -> Op {
        extern "C" {
            fn broc__mainForHost_1_caller(arg_0: &(), closure_data: *mut u8, output: *mut Op);
        }

        let mut output = std::mem::MaybeUninit::uninit();
        let ptr = self.closure_data.as_mut_ptr();
        unsafe { broc__mainForHost_1_caller(&arg_0, ptr, output.as_mut_ptr()) };
        unsafe { output.assume_init() }
    }
}

impl Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    #[inline(always)]
    fn storage(&self) -> Option<&core::cell::Cell<broc_std::Storage>> {
        let mask = match std::mem::size_of::<usize>() {
            4 => 0b11,
            8 => 0b111,
            _ => unreachable!(),
        };

        // NOTE: pointer provenance is probably lost here
        let unmasked_address = (self.pointer as usize) & !mask;
        let untagged = unmasked_address as *const core::cell::Cell<broc_std::Storage>;

        if untagged.is_null() {
            None
        } else {
            unsafe { Some(&*untagged.sub(1)) }
        }
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Op {
        // The discriminant is stored in the unused bytes at the end of the recursive pointer
        unsafe { core::mem::transmute::<u8, discriminant_Op>((self.pointer as u8) & 0b11) }
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Internal helper
    fn tag_discriminant(pointer: *mut union_Op, discriminant: discriminant_Op) -> *mut union_Op {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        let untagged = (pointer as usize) & (!0b11 as usize);
        let tagged = untagged | (discriminant as usize);

        tagged as *mut union_Op
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Internal helper
    fn union_pointer(&self) -> *mut union_Op {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        ((self.pointer as usize) & (!0b11 as usize)) as *mut union_Op
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// A tag named Done, which has no payload.
    pub const Done: Self = Self {
        pointer: core::ptr::null_mut(),
    };

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StderrWrite` and return its payload at index 0.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
    pub unsafe fn get_StderrWrite_0(&self) -> broc_std::BrocStr {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);

        extern "C" {
            #[link_name = "broc__getter__2_generic"]
            fn getter(_: *mut broc_std::BrocStr, _: *const Op);
        }

        let mut ret = core::mem::MaybeUninit::uninit();
        getter(ret.as_mut_ptr(), self);
        ret.assume_init()
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StderrWrite` and return its payload at index 1.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
    pub unsafe fn get_StderrWrite_1(&self) -> BrocFunction_67 {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);

        extern "C" {
            #[link_name = "broc__getter__3_size"]
            fn size() -> usize;

            #[link_name = "broc__getter__3_generic"]
            fn getter(_: *mut u8, _: *const Op);
        }

        // allocate memory to store this variably-sized value
        // allocates with broc_alloc, but that likely still uses the heap
        let it = std::iter::repeat(0xAAu8).take(size());
        let mut bytes = broc_std::BrocList::from_iter(it);

        getter(bytes.as_mut_ptr(), self);

        BrocFunction_67 {
            closure_data: bytes,
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `StderrWrite`, with the appropriate payload
    pub fn StderrWrite(arg: Op_StderrWrite) -> Self {
        let size = core::mem::size_of::<union_Op>();
        let align = core::mem::align_of::<union_Op>() as u32;

        unsafe {
            let ptr = broc_std::broc_alloc_refcounted::<union_Op>();

            *ptr = union_Op {
                StderrWrite: core::mem::ManuallyDrop::new(arg),
            };

            Self {
                pointer: Self::tag_discriminant(ptr, discriminant_Op::StderrWrite),
            }
        }
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StderrWrite` and convert it to `StderrWrite`'s payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
    pub unsafe fn into_StderrWrite(mut self) -> Op_StderrWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Op;
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut (*ptr).StderrWrite,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StderrWrite` and return its payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
    pub unsafe fn as_StderrWrite(&self) -> &Op_StderrWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Op;

            unsafe { &(*ptr).StderrWrite }
        };

        &payload
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StdoutWrite` and return its payload at index 0.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
    pub unsafe fn get_StdoutWrite_0(&self) -> broc_std::BrocStr {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);

        extern "C" {
            #[link_name = "broc__getter__2_generic"]
            fn getter(_: *mut broc_std::BrocStr, _: *const Op);
        }

        let mut ret = core::mem::MaybeUninit::uninit();
        getter(ret.as_mut_ptr(), self);
        ret.assume_init()
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StdoutWrite` and return its payload at index 1.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
    pub unsafe fn get_StdoutWrite_1(&self) -> BrocFunction_66 {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);

        extern "C" {
            #[link_name = "broc__getter__3_size"]
            fn size() -> usize;

            #[link_name = "broc__getter__3_generic"]
            fn getter(_: *mut u8, _: *const Op);
        }

        // allocate memory to store this variably-sized value
        // allocates with broc_alloc, but that likely still uses the heap
        let it = std::iter::repeat(0xAAu8).take(size());
        let mut bytes = broc_std::BrocList::from_iter(it);

        getter(bytes.as_mut_ptr(), self);

        BrocFunction_66 {
            closure_data: bytes,
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `StdoutWrite`, with the appropriate payload
    pub fn StdoutWrite(arg: Op_StdoutWrite) -> Self {
        let size = core::mem::size_of::<union_Op>();
        let align = core::mem::align_of::<union_Op>() as u32;

        unsafe {
            let ptr = broc_std::broc_alloc_refcounted::<union_Op>();

            *ptr = union_Op {
                StdoutWrite: core::mem::ManuallyDrop::new(arg),
            };

            Self {
                pointer: Self::tag_discriminant(ptr, discriminant_Op::StdoutWrite),
            }
        }
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StdoutWrite` and convert it to `StdoutWrite`'s payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
    pub unsafe fn into_StdoutWrite(mut self) -> Op_StdoutWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Op;
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut (*ptr).StdoutWrite,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(target_arch = "arm", target_arch = "wasm32", target_arch = "x86"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StdoutWrite` and return its payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
    pub unsafe fn as_StdoutWrite(&self) -> &Op_StdoutWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b11) as *mut union_Op;

            unsafe { &(*ptr).StdoutWrite }
        };

        &payload
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Op {
        // The discriminant is stored in the unused bytes at the end of the recursive pointer
        unsafe { core::mem::transmute::<u8, discriminant_Op>((self.pointer as u8) & 0b111) }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Internal helper
    fn tag_discriminant(pointer: *mut union_Op, discriminant: discriminant_Op) -> *mut union_Op {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        let untagged = (pointer as usize) & (!0b111 as usize);
        let tagged = untagged | (discriminant as usize);

        tagged as *mut union_Op
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Internal helper
    fn union_pointer(&self) -> *mut union_Op {
        // The discriminant is stored in the unused bytes at the end of the union pointer
        ((self.pointer as usize) & (!0b111 as usize)) as *mut union_Op
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StderrWrite` and convert it to `StderrWrite`'s payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
    pub unsafe fn into_StderrWrite(mut self) -> Op_StderrWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Op;
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut (*ptr).StderrWrite,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StderrWrite` and return its payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
    pub unsafe fn as_StderrWrite(&self) -> &Op_StderrWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Op;

            unsafe { &(*ptr).StderrWrite }
        };

        &payload
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StdoutWrite` and convert it to `StdoutWrite`'s payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
    pub unsafe fn into_StdoutWrite(mut self) -> Op_StdoutWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Op;
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut (*ptr).StdoutWrite,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    /// Unsafely assume this `Op` has a `.discriminant()` of `StdoutWrite` and return its payload.
    /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
    /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
    pub unsafe fn as_StdoutWrite(&self) -> &Op_StdoutWrite {
        debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);
        let payload = {
            let ptr = (self.pointer as usize & !0b111) as *mut union_Op;

            unsafe { &(*ptr).StdoutWrite }
        };

        &payload
    }
}

impl Drop for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // We only need to do any work if there's actually a heap-allocated payload.
        if let Some(storage) = self.storage() {
            let mut new_storage = storage.get();

            // Decrement the refcount
            let needs_dealloc = !new_storage.is_readonly() && new_storage.decrease();

            if needs_dealloc {
                // Drop the payload first.
                match self.discriminant() {
                    discriminant_Op::Done => {}
                    discriminant_Op::StderrWrite => unsafe {
                        core::mem::ManuallyDrop::drop(&mut (&mut *self.union_pointer()).StderrWrite)
                    },
                    discriminant_Op::StdoutWrite => unsafe {
                        core::mem::ManuallyDrop::drop(&mut (&mut *self.union_pointer()).StdoutWrite)
                    },
                }

                // Dealloc the pointer
                let alignment =
                    core::mem::align_of::<Self>().max(core::mem::align_of::<broc_std::Storage>());

                unsafe {
                    crate::broc_dealloc(storage.as_ptr().cast(), alignment as u32);
                }
            } else {
                // Write the storage back.
                storage.set(new_storage);
            }
        }
    }
}

impl Eq for Op {}

impl PartialEq for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
        if self.discriminant() != other.discriminant() {
            return false;
        }

        unsafe {
            match self.discriminant() {
                discriminant_Op::Done => true,
                discriminant_Op::StderrWrite => {
                    (&*self.union_pointer()).StderrWrite == (&*other.union_pointer()).StderrWrite
                }
                discriminant_Op::StdoutWrite => {
                    (&*self.union_pointer()).StdoutWrite == (&*other.union_pointer()).StdoutWrite
                }
            }
        }
    }
}

impl PartialOrd for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.discriminant().partial_cmp(&other.discriminant()) {
            Some(core::cmp::Ordering::Equal) => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_Op::Done => Some(core::cmp::Ordering::Equal),
                discriminant_Op::StderrWrite => (&*self.union_pointer())
                    .StderrWrite
                    .partial_cmp(&(&*other.union_pointer()).StderrWrite),
                discriminant_Op::StdoutWrite => (&*self.union_pointer())
                    .StdoutWrite
                    .partial_cmp(&(&*other.union_pointer()).StdoutWrite),
            }
        }
    }
}

impl Ord for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.discriminant().cmp(&other.discriminant()) {
            core::cmp::Ordering::Equal => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_Op::Done => core::cmp::Ordering::Equal,
                discriminant_Op::StderrWrite => (&*self.union_pointer())
                    .StderrWrite
                    .cmp(&(&*other.union_pointer()).StderrWrite),
                discriminant_Op::StdoutWrite => (&*self.union_pointer())
                    .StdoutWrite
                    .cmp(&(&*other.union_pointer()).StdoutWrite),
            }
        }
    }
}

impl Clone for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
        if let Some(storage) = self.storage() {
            let mut new_storage = storage.get();
            if !new_storage.is_readonly() {
                new_storage.increment_reference_count();
                storage.set(new_storage);
            }
        }

        Self {
            pointer: self.pointer,
        }
    }
}

impl core::hash::Hash for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self.discriminant() {
            discriminant_Op::Done => discriminant_Op::Done.hash(state),
            discriminant_Op::StderrWrite => unsafe {
                discriminant_Op::StderrWrite.hash(state);
                (&*self.union_pointer()).StderrWrite.hash(state);
            },
            discriminant_Op::StdoutWrite => unsafe {
                discriminant_Op::StdoutWrite.hash(state);
                (&*self.union_pointer()).StdoutWrite.hash(state);
            },
        }
    }
}

impl core::fmt::Debug for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Op::")?;

        unsafe {
            match self.discriminant() {
                discriminant_Op::Done => f.write_str("Done"),
                discriminant_Op::StderrWrite => f
                    .debug_tuple("StderrWrite")
                    // TODO HAS CLOSURE
                    .finish(),
                discriminant_Op::StdoutWrite => f
                    .debug_tuple("StdoutWrite")
                    // TODO HAS CLOSURE
                    .finish(),
            }
        }
    }
}
