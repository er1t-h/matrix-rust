use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

///
/// Trait used to describe a structure that act as a mathematical space.
///
pub trait Space:
    Sized
    + PartialEq<Self>
    + Clone
where
    for<'a> Self: AddAssign<&'a Self> + SubAssign<&'a Self> + MulAssign<&'a Self> + DivAssign<&'a Self>,
    for<'a, 'b> &'a Self: PartialEq<&'b Self>
{
}

impl Space for f32 {}
impl Space for f64 {}
impl Space for u8 {}
impl Space for u16 {}
impl Space for u32 {}
impl Space for u64 {}
impl Space for u128 {}
impl Space for i8 {}
impl Space for i16 {}
impl Space for i32 {}
impl Space for i64 {}
impl Space for i128 {}
