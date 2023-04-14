use std::mem::MaybeUninit;

use broc_mono::ir::CrashTag;
use broc_std::BrocStr;

/// This must have the same size as the repr() of BrocCallResult!
pub const ROC_CALL_RESULT_DISCRIMINANT_SIZE: usize = std::mem::size_of::<u64>();

#[repr(C)]
pub struct BrocCallResult<T> {
    tag: u64,
    error_msg: *mut BrocStr,
    value: MaybeUninit<T>,
}

impl<T> BrocCallResult<T> {
    pub fn new(value: T) -> Self {
        Self {
            tag: 0,
            error_msg: std::ptr::null_mut(),
            value: MaybeUninit::new(value),
        }
    }
}

impl<T: Default> Default for BrocCallResult<T> {
    fn default() -> Self {
        Self {
            tag: 0,
            error_msg: std::ptr::null_mut(),
            value: MaybeUninit::new(Default::default()),
        }
    }
}

impl<T: Sized> From<BrocCallResult<T>> for Result<T, (String, CrashTag)> {
    fn from(call_result: BrocCallResult<T>) -> Self {
        match call_result.tag {
            0 => Ok(unsafe { call_result.value.assume_init() }),
            n => Err({
                let msg: &BrocStr = unsafe { &*call_result.error_msg };
                let tag = (n - 1) as u32;
                let tag = tag
                    .try_into()
                    .unwrap_or_else(|_| panic!("received illegal tag: {tag}"));

                (msg.as_str().to_owned(), tag)
            }),
        }
    }
}

#[macro_export]
macro_rules! run_broc_dylib {
    ($lib:expr, $main_fn_name:expr, $argument_type:ty, $return_type:ty) => {{
        use inkwell::context::Context;
        use broc_builtins::bitcode;
        use broc_gen_llvm::run_broc::BrocCallResult;
        use std::mem::MaybeUninit;

        // NOTE: return value is not first argument currently (may/should change in the future)
        type Main = unsafe extern "C" fn($argument_type, *mut BrocCallResult<$return_type>);

        unsafe {
            let main: libloading::Symbol<Main> = $lib
                .get($main_fn_name.as_bytes())
                .ok()
                .ok_or(format!("Unable to JIT compile `{}`", $main_fn_name))
                .expect("errored");

            main
        }
    }};
}

#[macro_export]
macro_rules! try_run_jit_function {
    ($lib: expr, $main_fn_name: expr, $ty:ty, $transform:expr) => {{
        let v: String = String::new();
        try_run_jit_function!($lib, $main_fn_name, $ty, $transform, v)
    }};

    ($lib: expr, $main_fn_name: expr, $ty:ty, $transform:expr) => {{
        try_run_jit_function!($lib, $main_fn_name, $ty, $transform, &[])
    }};
    ($lib: expr, $main_fn_name: expr, $ty:ty, $transform:expr, $expect_failures:expr) => {{
        use inkwell::context::Context;
        use broc_builtins::bitcode;
        use broc_gen_llvm::run_broc::BrocCallResult;
        use std::mem::MaybeUninit;

        unsafe {
            let main: libloading::Symbol<unsafe extern "C" fn(*mut BrocCallResult<$ty>)> = $lib
                .get($main_fn_name.as_bytes())
                .ok()
                .ok_or(format!("Unable to JIT compile `{}`", $main_fn_name))
                .expect("errored");

            let mut main_result = MaybeUninit::uninit();
            main(main_result.as_mut_ptr());

            main_result.assume_init().into()
        }
    }};
}

#[macro_export]
macro_rules! run_jit_function {
    ($lib: expr, $main_fn_name: expr, $ty:ty, $transform:expr) => {{
        let v: String = String::new();
        run_jit_function!($lib, $main_fn_name, $ty, $transform, v)
    }};

    ($lib: expr, $main_fn_name: expr, $ty:ty, $transform:expr, $errors:expr) => {{
        run_jit_function!($lib, $main_fn_name, $ty, $transform, $errors, &[])
    }};
    ($lib: expr, $main_fn_name: expr, $ty:ty, $transform:expr, $errors:expr, $expect_failures:expr) => {{
        let result =
            $crate::try_run_jit_function!($lib, $main_fn_name, $ty, $transform, $expect_failures);

        match result {
            Ok(success) => {
                // only if there are no exceptions thrown, check for errors
                assert!($errors.is_empty(), "Encountered errors:\n{}", $errors);

                $transform(success)
            }
            Err((error_msg, _)) => {
                eprintln!("This Broc code crashed with: \"{error_msg}\"");

                Expr::MalformedClosure
            }
        }
    }};
}

/// In the repl, we don't know the type that is returned; it it's large enough to not fit in 2
/// registers (i.e. size bigger than 16 bytes on 64-bit systems), then we use this macro.
/// It explicitly allocates a buffer that the broc main function can write its result into.
#[macro_export]
macro_rules! run_jit_function_dynamic_type {
    ($lib: expr, $main_fn_name: expr, $bytes:expr, $transform:expr) => {{
        let v: String = String::new();
        run_jit_function_dynamic_type!($lib, $main_fn_name, $bytes, $transform, v)
    }};

    ($lib: expr, $main_fn_name: expr, $bytes:expr, $transform:expr, $errors:expr) => {{
        use inkwell::context::Context;
        use broc_gen_llvm::run_broc::BrocCallResult;

        unsafe {
            let main: libloading::Symbol<unsafe extern "C" fn(*const u8)> = $lib
                .get($main_fn_name.as_bytes())
                .ok()
                .ok_or(format!("Unable to JIT compile `{}`", $main_fn_name))
                .expect("errored");

            let size = std::mem::size_of::<BrocCallResult<()>>() + $bytes;
            let layout = std::alloc::Layout::array::<u8>(size).unwrap();
            let result = std::alloc::alloc(layout);
            main(result);

            let flag = *result;

            if flag == 0 {
                $transform(result.add(std::mem::size_of::<BrocCallResult<()>>()) as usize)
            } else {
                use std::ffi::CString;
                use std::os::raw::c_char;

                // first field is a char pointer (to the error message)
                // read value, and transmute to a pointer
                let ptr_as_int = *(result as *const u64).offset(1);
                let ptr = ptr_as_int as *mut c_char;

                // make CString (null-terminated)
                let raw = CString::from_raw(ptr);

                let result = format!("{:?}", raw);

                // make sure rust doesn't try to free the Broc constant string
                std::mem::forget(raw);

                eprintln!("{}", result);
                panic!("Broc hit an error");
            }
        }
    }};
}
