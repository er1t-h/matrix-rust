use std::cmp::Ord;

pub trait AddIdentity {
    fn add_identity() -> Self;
}
pub trait MulIdentity {
    fn mul_identity() -> Self;
}

pub trait Abs {
    fn abs(&self) -> Self;
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

pub trait Max {
    fn max(self, other: Self) -> Self;
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

macro_rules! impl_abs {
    ($current: ident, $($types: ident),+) => {
        impl_abs!($current);
        impl_abs!($($types),+);
    };
    ($current: ident) => {
        impl Abs for $current {
            fn abs(&self) -> Self {
                <$current>::abs(*self)
            }
        }
    };
}

macro_rules! impl_sqrt {
    ($current: ident, $($types: ident),+) => {
        impl_sqrt!($current);
        impl_sqrt!($($types),+);
    };
    ($current: ident) => {
        impl Sqrt for $current {
            fn sqrt(&self) -> Self {
                <$current>::sqrt(*self)
            }
        }
    };
}

macro_rules! impl_max {
    ($current: ident, $($types: ident),+) => {
        impl_max!($current);
        impl_max!($($types),+);
    };
    ($current: ident) => {
        impl Max for $current {
            fn max(self, other: Self) -> Self {
                <$current>::max(self, other)
            }
        }
    };
}
macro_rules! impl_max_ord {
    ($current: ident, $($types: ident),+) => {
        impl_max_ord!($current);
        impl_max_ord!($($types),+);
    };
    ($current: ident) => {
        impl Max for $current {
            fn max(self, other: Self) -> Self {
                <$current as Ord>::max(self, other)
            }
        }
    };
}

impl_mul_identity!(1, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_mul_identity!(1.0, f32, f64);
impl_add_identity!(0, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_add_identity!(0.0, f32, f64);

impl_abs!(i8, i16, i32, i64, i128, f32, f64);
impl_sqrt!(f32, f64);
impl_max!(f32, f64);
impl_max_ord!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
