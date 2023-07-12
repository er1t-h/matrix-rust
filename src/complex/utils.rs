use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::traits::{Abs, IsZero, One, Sqrt, Zero};

use super::Complex;

impl<T> Complex<T> {
    pub const fn new(real: T, imaginary: T) -> Self {
        Self { real, imaginary }
    }
}

impl<T> From<T> for Complex<T>
where
    T: Default,
{
    fn from(nb: T) -> Self {
        Self {
            real: nb,
            imaginary: T::default(),
        }
    }
}

impl<T: Zero> Zero for Complex<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: One + Zero> One for Complex<T> {
    fn one() -> Self {
        Self::new(T::one(), T::zero())
    }
}

impl<T: IsZero> IsZero for &Complex<T> {
    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }
}

impl<T> Abs for Complex<T>
where
    T: Add<T, Output = T> + Sqrt + Zero,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    fn abs(&self) -> Self {
        let real = (&self.real * &self.real + &self.imaginary * &self.imaginary).sqrt();
        Self {
            real,
            imaginary: T::zero(),
        }
    }
}

impl<T> Complex<T> {
    pub const fn im(&self) -> &T {
        &self.imaginary
    }
    pub const fn re(&self) -> &T {
        &self.real
    }
}

impl<T: Display + IsZero> Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (!self.real.is_zero(), !self.imaginary.is_zero()) {
            (true, true) => write!(f, "({} + {}i)", self.re(), self.im()),
            (_, false) => write!(f, "{}", self.re()),
            (false, true) => write!(f, "{}i", self.im()),
        }
    }
}

impl<T: IsZero> Complex<T> {
    pub fn is_real(&self) -> bool {
        self.imaginary.is_zero()
    }

    pub fn is_imaginary(&self) -> bool {
        self.real.is_zero()
    }
}
