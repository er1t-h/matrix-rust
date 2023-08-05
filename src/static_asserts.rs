use std::marker::PhantomData;

pub struct AssertNonZero<const N: usize>;
pub struct AssertNonZeroSizeType<T>(PhantomData<T>);

impl<const N: usize> AssertNonZero<N> {
    pub const OK: () = assert!(N != 0, "size can't be zero");
}

impl<T> AssertNonZeroSizeType<T> {
    pub const OK: () = assert!(
        std::mem::size_of::<T>() != 0,
        "zero size type not supported"
    );
}
