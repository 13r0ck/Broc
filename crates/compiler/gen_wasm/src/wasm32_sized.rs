use broc_std::{BrocBox, BrocDec, BrocList, BrocOrder, BrocResult, BrocStr, I128, U128};

pub trait Wasm32Sized: Sized {
    const SIZE_OF_WASM: usize;
    const ALIGN_OF_WASM: usize;
    const ACTUAL_WIDTH: usize =
        if (Self::ALIGN_OF_WASM == 0) || (Self::SIZE_OF_WASM % Self::ALIGN_OF_WASM) == 0 {
            Self::SIZE_OF_WASM
        } else {
            Self::SIZE_OF_WASM + (Self::ALIGN_OF_WASM - (Self::SIZE_OF_WASM % Self::ALIGN_OF_WASM))
        };
}

macro_rules! wasm32_sized_primitive {
    ($($type_name:ident ,)+) => {
        $(
            impl Wasm32Sized for $type_name {
                const SIZE_OF_WASM: usize = core::mem::size_of::<$type_name>();
                const ALIGN_OF_WASM: usize = core::mem::align_of::<$type_name>();
            }
        )*
    }
}

wasm32_sized_primitive!(u8, i8, u16, i16, u32, i32, char, u64, i64, u128, i128, f32, f64, bool,);
wasm32_sized_primitive!(BrocDec, BrocOrder, I128, U128,);

impl Wasm32Sized for () {
    const SIZE_OF_WASM: usize = 0;
    const ALIGN_OF_WASM: usize = 0;
}

impl Wasm32Sized for std::convert::Infallible {
    const SIZE_OF_WASM: usize = 0;
    const ALIGN_OF_WASM: usize = 0;
}

impl Wasm32Sized for BrocStr {
    const SIZE_OF_WASM: usize = 12;
    const ALIGN_OF_WASM: usize = 4;
}

impl<T: Wasm32Sized> Wasm32Sized for BrocList<T> {
    const SIZE_OF_WASM: usize = 12;
    const ALIGN_OF_WASM: usize = 4;
}

impl<T: Wasm32Sized> Wasm32Sized for BrocBox<T> {
    const SIZE_OF_WASM: usize = 4;
    const ALIGN_OF_WASM: usize = 4;
}

impl<T: Wasm32Sized, E: Wasm32Sized> Wasm32Sized for BrocResult<T, E> {
    const ALIGN_OF_WASM: usize = max(&[T::ALIGN_OF_WASM, E::ALIGN_OF_WASM]);
    const SIZE_OF_WASM: usize = max(&[T::ACTUAL_WIDTH, E::ACTUAL_WIDTH]) + 1;
}

impl<T: Wasm32Sized> Wasm32Sized for &'_ T {
    const SIZE_OF_WASM: usize = 4;
    const ALIGN_OF_WASM: usize = 4;
}

impl<T: Wasm32Sized, const N: usize> Wasm32Sized for [T; N] {
    const SIZE_OF_WASM: usize = N * T::SIZE_OF_WASM;
    const ALIGN_OF_WASM: usize = T::ALIGN_OF_WASM;
}

impl Wasm32Sized for usize {
    const SIZE_OF_WASM: usize = 4;
    const ALIGN_OF_WASM: usize = 4;
}

impl Wasm32Sized for isize {
    const SIZE_OF_WASM: usize = 4;
    const ALIGN_OF_WASM: usize = 4;
}

impl<T: Wasm32Sized, U: Wasm32Sized> Wasm32Sized for (T, U) {
    const SIZE_OF_WASM: usize = T::SIZE_OF_WASM + U::SIZE_OF_WASM;
    const ALIGN_OF_WASM: usize = max(&[T::ALIGN_OF_WASM, U::ALIGN_OF_WASM]);
}

impl<T: Wasm32Sized, U: Wasm32Sized, V: Wasm32Sized> Wasm32Sized for (T, U, V) {
    const SIZE_OF_WASM: usize = T::SIZE_OF_WASM + U::SIZE_OF_WASM + V::SIZE_OF_WASM;
    const ALIGN_OF_WASM: usize = max(&[T::ALIGN_OF_WASM, U::ALIGN_OF_WASM, V::ALIGN_OF_WASM]);
}

impl<T: Wasm32Sized, U: Wasm32Sized, V: Wasm32Sized, W: Wasm32Sized> Wasm32Sized for (T, U, V, W) {
    const SIZE_OF_WASM: usize =
        T::SIZE_OF_WASM + U::SIZE_OF_WASM + V::SIZE_OF_WASM + W::SIZE_OF_WASM;
    const ALIGN_OF_WASM: usize = max(&[
        T::ALIGN_OF_WASM,
        U::ALIGN_OF_WASM,
        V::ALIGN_OF_WASM,
        W::ALIGN_OF_WASM,
    ]);
}

const fn max(alignments: &[usize]) -> usize {
    assert!(!alignments.is_empty());

    let mut largest = 0;
    let mut i = 0;
    while i < alignments.len() {
        largest = if largest > alignments[i] {
            largest
        } else {
            alignments[i]
        };

        i += 1;
    }

    largest
}
