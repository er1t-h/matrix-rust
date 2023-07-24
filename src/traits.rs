use std::cmp::Ord;

pub trait Zero {
    #[must_use]
    fn zero() -> Self;
}
pub trait One {
    #[must_use]
    fn one() -> Self;
}

pub trait Abs {
    #[must_use]
    fn abs(&self) -> Self;
}

pub trait Sqrt {
    #[must_use]
    fn sqrt(&self) -> Self;
}

pub trait Max {
    #[must_use]
    fn max(self, other: Self) -> Self;
}

pub trait Divisor {
    fn can_be_divisor(&self) -> bool;
}

pub trait IsZero {
    fn is_zero(&self) -> bool;
}
pub trait IsOne {
    fn is_one(&self) -> bool;
}

pub trait FMA<Multiple = Self, Add = Self, Output = Self> {
    fn fma(&self, a: &Multiple, b: &Add) -> Output;
}

pub trait SafeAdd<Rhs = Self>: Sized {
    type Error;
    ///
    /// Adds `rhs` into self.
    ///
    /// # Errors
    /// If the two objects can't be added, returns an appropriate error
    ///
    fn safe_add_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;

    ///
    /// Adds `rhs` and `self`.
    ///
    /// # Errors
    /// If the two objects can't be added, returns an appropriate error
    ///
    fn safe_add(mut self, rhs: Rhs) -> Result<Self, Self::Error> {
        self.safe_add_assign(rhs).map(|_| self)
    }
}
pub trait SafeSub<Rhs = Self>: Sized {
    type Error;
    ///
    /// Subs `rhs` from `self`.
    ///
    /// # Errors
    /// If the two objects can't be added, returns an appropriate error
    ///
    fn safe_sub_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
    ///
    /// Subs `rhs` from `self`, taking its value.
    ///
    /// # Errors
    /// If the two objects can't be added, returns an appropriate error
    ///
    fn safe_sub(mut self, rhs: Rhs) -> Result<Self, Self::Error> {
        self.safe_sub_assign(rhs).map(|_| self)
    }
}

macro_rules! impl_mul_identity {
    ($value: expr, $current: ident, $($types: ident),+) => {
        impl_mul_identity!($value, $current);
        impl_mul_identity!($value, $($types),+);
    };
    ($value: expr, $current: ident) => {
        impl One for $current {
            #[inline(always)]
            fn one() -> Self {
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
        impl Zero for $current {
            #[inline(always)]
            fn zero() -> Self {
                $value
            }
        }
    };
}

macro_rules! impl_divisor {
    ($value: expr, $current: ident, $($types: ident),+) => {
        impl_divisor!($value, $current);
        impl_divisor!($value, $($types),+);
    };
    ($value: expr, $current: ident) => {
        impl Divisor for $current {
            #[inline(always)]
            #[allow(clippy::float_cmp)]
            fn can_be_divisor(&self) -> bool {
                self != $value
            }
        }
    };
}

macro_rules! impl_is_zero {
    ($value: expr, $current: ident, $($types: ident),+) => {
        impl_is_zero!($value, $current);
        impl_is_zero!($value, $($types),+);
    };
    ($value: expr, $current: ident) => {
        impl IsZero for $current {
            #[inline(always)]
            #[allow(clippy::float_cmp)]
            fn is_zero(&self) -> bool {
                self == $value
            }
        }

        impl IsZero for &$current {
            #[inline(always)]
            #[allow(clippy::float_cmp)]
            fn is_zero(&self) -> bool {
                self == &$value
            }
        }

        impl IsZero for &mut $current {
            #[inline(always)]
            #[allow(clippy::float_cmp)]
            fn is_zero(&self) -> bool {
                self == &$value
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

macro_rules! impl_fma {
    (float, $current: ident, $($types: ident),+) => {
        impl_fma!(float, $current);
        impl_fma!(float, $($types),+);
    };
    (float, $current: ident) => {
        impl FMA for $current {
            fn fma(&self, a: &Self, b: &Self) -> Self {
                (*self).mul_add(*a, *b)
            }
        }
    };
    (int, $current: ident, $($types: ident),+) => {
        impl_fma!(int, $current);
        impl_fma!(int, $($types),+);
    };
    (int, $current: ident) => {
        impl FMA for $current {
            fn fma(&self, a: &Self, b: &Self) -> Self {
                self * a + b
            }
        }
    };
}

impl_mul_identity!(1, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_mul_identity!(1.0, f32, f64);
impl_add_identity!(0, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_add_identity!(0.0, f32, f64);
impl_divisor!(&0, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_divisor!(&0.0, f32, f64);
impl_is_zero!(&0, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_is_zero!(&0.0, f32, f64);

impl_abs!(i8, i16, i32, i64, i128, f32, f64);
impl_sqrt!(f32, f64);
impl_max!(f32, f64);
impl_max_ord!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_fma!(float, f32, f64);
impl_fma!(int, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
