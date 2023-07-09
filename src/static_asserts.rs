pub struct AssertNonZero<const N: usize>;

impl<const N: usize> AssertNonZero<N> {
    pub const OK: () = assert!(N != 0, "size can't be zero");
}
