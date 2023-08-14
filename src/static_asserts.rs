use std::marker::PhantomData;

pub struct AssertNonZero<const N: usize>;
pub struct AssertOperationEqual<const LHS: usize, const RHS: usize, const EXPECT: usize>;
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

#[allow(dead_code)]
impl<const LHS: usize, const RHS: usize, const EXPECT: usize>
    AssertOperationEqual<LHS, RHS, EXPECT>
{
    pub const ADD: () = assert!(LHS + RHS == EXPECT, "LHS + RHS should be equal to EXPECT");
    pub const SUB: () = assert!(LHS - RHS == EXPECT, "LHS - RHS should be equal to EXPECT");
    pub const MUL: () = assert!(LHS * RHS == EXPECT, "LHS * RHS should be equal to EXPECT");
    pub const DIV: () = assert!(LHS / RHS == EXPECT, "LHS / RHS should be equal to EXPECT");
}
