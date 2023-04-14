#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate pretty_assertions;
extern crate quickcheck;
extern crate broc_std;

use core::ffi::c_void;

const ROC_SMALL_STR_CAPACITY: usize = core::mem::size_of::<broc_std::BrocStr>() - 1;

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

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn broc_panic(c_ptr: *mut c_void, tag_id: u32) {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    match tag_id {
        0 => {
            let c_str = CStr::from_ptr(c_ptr as *const c_char);
            let string = c_str.to_str().unwrap();
            panic!("broc_panic during test: {}", string);
        }
        _ => todo!(),
    }
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn broc_memcpy(dst: *mut c_void, src: *mut c_void, n: usize) -> *mut c_void {
    libc::memcpy(dst, src, n)
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn broc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}

#[cfg(test)]
mod test_broc_std {
    use broc_std::{BrocBox, BrocDec, BrocList, BrocResult, BrocStr, SendSafeBrocList, SendSafeBrocStr};

    fn broc_str_byte_representation(string: &BrocStr) -> [u8; BrocStr::SIZE] {
        unsafe { core::mem::transmute_copy(string) }
    }

    #[test]
    fn broc_str_empty() {
        let actual = broc_str_byte_representation(&BrocStr::empty());

        let mut expected = [0u8; BrocStr::SIZE];
        expected[BrocStr::SIZE - 1] = BrocStr::MASK;

        assert_eq!(actual, expected);
    }

    #[test]
    fn broc_str_single_char() {
        let actual = broc_str_byte_representation(&BrocStr::from("a"));

        let mut expected = [0u8; BrocStr::SIZE];
        expected[0] = b'a';
        expected[BrocStr::SIZE - 1] = BrocStr::MASK | 1;

        assert_eq!(actual, expected);
    }

    #[test]
    fn broc_str_max_small_string() {
        let s = str::repeat("a", BrocStr::SIZE - 1);
        let actual = broc_str_byte_representation(&BrocStr::from(s.as_str()));

        let mut expected = [0u8; BrocStr::SIZE];
        expected[..BrocStr::SIZE - 1].copy_from_slice(s.as_bytes());
        expected[BrocStr::SIZE - 1] = BrocStr::MASK | s.len() as u8;

        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_string_from_str() {
        let a = BrocStr::from("");
        let b = BrocStr::empty();

        assert_eq!(a, b);
    }

    #[test]
    fn empty_string_length() {
        let string = BrocStr::from("");

        assert_eq!(string.len(), 0);
    }

    #[test]
    fn empty_string_capacity() {
        let string = BrocStr::empty();

        assert_eq!(string.capacity(), super::ROC_SMALL_STR_CAPACITY);
    }

    #[test]
    fn reserve_small_str() {
        let mut broc_str = BrocStr::empty();

        broc_str.reserve(42);

        assert_eq!(broc_str.capacity() >= 42, true);
    }

    #[test]
    fn reserve_big_str() {
        let mut broc_str = BrocStr::empty();

        broc_str.reserve(5000);

        assert_eq!(broc_str.capacity() >= 5000, true);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn str_short_serde_roundtrip() {
        let orig = BrocStr::from("x");

        let serialized = serde_json::to_string(&orig).expect("failed to serialize string");
        let deserialized = serde_json::from_str(&serialized).expect("failed to deserialize string");

        assert_eq!(orig, deserialized);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn str_long_serde_roundtrip() {
        // How about a little philosophy to accompany test failures?
        let orig = BrocStr::from("If there's a remedy when trouble strikes, what reason is there for dejection? And if there is no help for it, what use is there in being glum? -- Shantideva, The Way of the Bodhisattva");

        let serialized = serde_json::to_string(&orig).expect("failed to serialize string");
        let deserialized = serde_json::from_str(&serialized).expect("failed to deserialize string");

        assert_eq!(orig, deserialized);
    }

    #[test]
    fn reserve_small_list() {
        let mut broc_list = BrocList::<BrocStr>::empty();

        broc_list.reserve(42);

        assert_eq!(broc_list.capacity(), 42);
    }

    #[test]
    fn reserve_big_list() {
        let mut broc_list = BrocList::<BrocStr>::empty();

        broc_list.reserve(5000);

        assert_eq!(broc_list.capacity(), 5000);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn short_list_roundtrip() {
        let items: [u8; 4] = [1, 3, 3, 7];
        let orig = BrocList::from_slice(&items);

        let serialized = serde_json::to_string(&orig).expect("failed to serialize string");
        let deserialized =
            serde_json::from_str::<BrocList<u8>>(&serialized).expect("failed to deserialize string");

        assert_eq!(orig, deserialized);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn long_list_roundtrip() {
        let orig = BrocList::from_iter(1..100);

        let serialized = serde_json::to_string(&orig).expect("failed to serialize string");
        let deserialized =
            serde_json::from_str::<BrocList<u8>>(&serialized).expect("failed to deserialize string");

        assert_eq!(orig, deserialized);
    }

    #[test]
    fn list_from_iter() {
        let elems: [i64; 5] = [1, 2, 3, 4, 5];
        let from_slice = BrocList::from_slice(&elems);
        let from_iter = BrocList::from_iter(elems);
        assert_eq!(from_iter, from_slice);
        assert_eq!(from_iter.capacity(), from_slice.capacity());
    }

    #[test]
    fn list_from_iter_zero_size() {
        let elems: [(); 5] = [(), (), (), (), ()];
        let from_slice = BrocList::from_slice(&elems);
        let from_iter = BrocList::from_iter(elems);
        assert_eq!(from_iter, from_slice);
    }

    #[test]
    fn list_from_array() {
        let elems: [i64; 5] = [1, 2, 3, 4, 5];
        let from_slice = BrocList::from_slice(&elems);
        let from_array = BrocList::from(elems);
        assert_eq!(from_array, from_slice);
        assert_eq!(from_array.capacity(), from_slice.capacity());
    }

    #[test]
    fn list_from_array_zero_size() {
        let elems: [(); 5] = [(), (), (), (), ()];
        let from_slice = BrocList::from_slice(&elems);
        let from_array = BrocList::from(elems);
        assert_eq!(from_array, from_slice);
        assert_eq!(from_array.capacity(), from_slice.capacity());
    }

    #[test]
    fn broc_result_to_rust_result() {
        let greeting = "Hello, World!";
        let broc_result: BrocResult<String, ()> = BrocResult::ok(greeting.into());

        match broc_result.into() {
            Ok(answer) => {
                assert_eq!(answer.as_str(), greeting);
            }
            Err(()) => {
                panic!("Received an Err when Ok was expected.")
            }
        }
    }

    #[test]
    fn broc_result_is_ok() {
        let greeting = "Hello, World!";
        let broc_result: BrocResult<String, ()> = BrocResult::ok(greeting.into());

        assert!(broc_result.is_ok());
        assert!(!broc_result.is_err());
    }

    #[test]
    fn broc_result_is_err() {
        let greeting = "Hello, World!";
        let broc_result: BrocResult<(), String> = BrocResult::err(greeting.into());

        assert!(!broc_result.is_ok());
        assert!(broc_result.is_err());
    }

    #[test]
    fn create_broc_box() {
        let contents = 42i32;
        let broc_box = BrocBox::new(contents);

        assert_eq!(broc_box.into_inner(), contents)
    }

    #[test]
    fn broc_dec_fmt() {
        assert_eq!(
            format!("{}", BrocDec::MIN),
            "-1701411834604692317316.87303715884105728"
        );

        let half = BrocDec::from_str("0.5").unwrap();
        assert_eq!(format!("{}", half), "0.5");

        let ten = BrocDec::from_str("10").unwrap();
        assert_eq!(format!("{}", ten), "10");

        let example = BrocDec::from_str("1234.5678").unwrap();
        assert_eq!(format!("{}", example), "1234.5678");
    }

    #[test]
    fn safe_send_no_copy() {
        let x = BrocStr::from("This is a long string but still unique. Yay!!!");
        assert_eq!(x.is_unique(), true);

        let safe_x = SendSafeBrocStr::from(x);
        let new_x = BrocStr::from(safe_x);
        assert_eq!(new_x.is_unique(), true);
        assert_eq!(
            new_x.as_str(),
            "This is a long string but still unique. Yay!!!"
        );
    }

    #[test]
    fn safe_send_requires_copy() {
        let x = BrocStr::from("This is a long string but still unique. Yay!!!");
        let y = x.clone();
        let z = y.clone();
        assert_eq!(x.is_unique(), false);
        assert_eq!(y.is_unique(), false);
        assert_eq!(z.is_unique(), false);

        let safe_x = SendSafeBrocStr::from(x);
        let new_x = BrocStr::from(safe_x);
        assert_eq!(new_x.is_unique(), true);
        assert_eq!(y.is_unique(), false);
        assert_eq!(z.is_unique(), false);
        assert_eq!(
            new_x.as_str(),
            "This is a long string but still unique. Yay!!!"
        );
    }

    #[test]
    fn safe_send_small_str() {
        let x = BrocStr::from("short");
        let y = x.clone();
        let z = y.clone();
        assert_eq!(x.is_unique(), true);
        assert_eq!(y.is_unique(), true);
        assert_eq!(z.is_unique(), true);

        let safe_x = SendSafeBrocStr::from(x);
        let new_x = BrocStr::from(safe_x);
        assert_eq!(new_x.is_unique(), true);
        assert_eq!(y.is_unique(), true);
        assert_eq!(z.is_unique(), true);
        assert_eq!(new_x.as_str(), "short");
    }

    #[test]
    fn empty_list_is_unique() {
        let broc_list = BrocList::<BrocStr>::empty();
        assert_eq!(broc_list.is_unique(), true);
    }

    #[test]
    fn readonly_list_is_sendsafe() {
        let x = BrocList::from_slice(&[1, 2, 3, 4, 5]);
        unsafe { x.set_readonly() };
        assert_eq!(x.is_readonly(), true);

        let y = x.clone();
        let z = y.clone();

        let safe_x = SendSafeBrocList::from(x);
        let new_x = BrocList::from(safe_x);
        assert_eq!(new_x.is_readonly(), true);
        assert_eq!(y.is_readonly(), true);
        assert_eq!(z.is_readonly(), true);
        assert_eq!(new_x.as_slice(), &[1, 2, 3, 4, 5]);
    }
}

#[cfg(test)]
mod with_terminator {
    use core::slice;
    use broc_std::BrocStr;
    use std::ffi::CStr;

    fn verify_temp_c(string: &str, excess_capacity: usize) {
        let mut broc_str = BrocStr::from(string);

        println!("-------------1--------------");
        if excess_capacity > 0 {
            broc_str.reserve(excess_capacity);
        }

        // utf8_nul_terminated
        {
            println!("-------------2--------------");
            let answer = broc_str.clone().utf8_nul_terminated(|ptr, len| {
                println!("-------------3--------------");
                let bytes = unsafe { slice::from_raw_parts(ptr, len + 1) };
                println!("-------------4--------------");
                let c_str = CStr::from_bytes_with_nul(bytes).unwrap();
                println!("-------------5--------------");

                assert_eq!(c_str.to_str(), Ok(string));
                println!("-------------6--------------");

                42
            });

            assert_eq!(Ok(42), answer);
        }

        // utf16_nul_terminated
        {
            let answer = broc_str.utf16_nul_terminated(|ptr, len| {
                let bytes: &[u16] = unsafe { slice::from_raw_parts(ptr.cast(), len + 1) };

                // Verify that it's nul-terminated
                assert_eq!(bytes[len], 0);

                let string = String::from_utf16(&bytes[0..len]).unwrap();

                assert_eq!(string.as_str(), string);

                42
            });

            assert_eq!(Ok(42), answer);
        }
    }

    #[test]
    fn empty_string() {
        verify_temp_c("", 0);
    }

    /// e.g. "a" or "abc" or "abcdefg" etc.
    fn string_for_len(len: usize) -> String {
        let first_index: usize = 97; // start with ASCII lowercase "a"
        let bytes: Vec<u8> = (0..len)
            .map(|index| {
                let letter = (index % 26) + first_index;

                letter.try_into().unwrap()
            })
            .collect();

        assert_eq!(bytes.len(), len);

        // The bytes should contain no nul characters.
        assert!(bytes.iter().all(|byte| *byte != 0));

        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn small_strings() {
        for len in 1..=super::ROC_SMALL_STR_CAPACITY {
            verify_temp_c(&string_for_len(len), 0);
        }
    }

    #[test]
    fn no_excess_capacity() {
        // This is small enough that it should be a stack allocation for UTF-8
        verify_temp_c(&string_for_len(33), 0);

        // This is big enough that it should be a heap allocation for UTF-8 and UTF-16
        verify_temp_c(&string_for_len(65), 0);
    }

    #[test]
    fn with_excess_capacity() {
        println!("Start!");
        // We should be able to use the excess capacity for all of these.
        verify_temp_c(&string_for_len(33), 1); // TODO why isn't this unique?! ohh because I CLONED IT
        println!("Success!");
        // verify_temp_c(&string_for_len(33), 33);
        // verify_temp_c(&string_for_len(65), 1);
        // verify_temp_c(&string_for_len(65), 64);
    }
}
