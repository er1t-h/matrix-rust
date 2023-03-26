use std::{
    iter::Sum,
    ops::{AddAssign, DivAssign, Mul, MulAssign, SubAssign},
};

///
/// Trait used to describe a structure that act as a mathematical space.
///
pub trait Space: Sized + PartialEq<Self> + Clone + AddIdentity + MulIdentity
where
    for<'a> Self: AddAssign<&'a Self>
        + SubAssign<&'a Self>
        + MulAssign<&'a Self>
        + DivAssign<&'a Self>
        + Sum<&'a Self>
        + Mul<&'a Self, Output = Self>,
    for<'a, 'b> &'a Self: PartialEq<&'b Self>,
{
}

pub trait AddIdentity {
    fn add_identity() -> Self;
}
pub trait MulIdentity {
    fn mul_identity() -> Self;
}

macro_rules! impl_mul_identity {
    ($value: expr, $current: ident, $($types: ident),+) => {
        impl_mul_identity!($value, $current);
        impl_mul_identity!($value, $($types),+);
    };
    ($value: expr, $current: ident) => {
        impl MulIdentity for $current {
            fn mul_identity() -> Self {
                $value
            }
        }
    };
}

macro_rules! impl_add_identity {
    ($value: expr, $current: ident, $($types: ident),+) => {
        impl_add_identity!($value, $current);
        impl_add_identity!($value, $($types),+);
    };
    ($value: expr, $current: ident) => {
        impl AddIdentity for $current {
            fn add_identity() -> Self {
                $value
            }
        }
    };
}

macro_rules! impl_space {
    ($current: ident, $($types: ident),+) => {
        impl_space!($current);
        impl_space!($($types),+);
    };
    ($current: ident) => {
        impl Space for $current {}
    };
}

impl_mul_identity!(1, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_mul_identity!(1.0, f32, f64);
impl_add_identity!(0, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_add_identity!(0.0, f32, f64);

impl_space!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
