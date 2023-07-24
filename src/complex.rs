#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T: Debug> Debug for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} + {:?}i)", self.re(), self.im())
    }
}

mod operations;
mod utils;

#[cfg(test)]
macro_rules! cpl {
    ($real: expr, $imag: expr) => {
        crate::Complex::new($real, $imag)
    };
    ($real: literal + $imag: literal i) => {
        cpl!($real, $imag)
    };
    (-$real: literal + $imag: literal i) => {
        cpl!(-$real, $imag)
    };
    ($real: literal - $imag: literal i) => {
        cpl!($real, -$imag)
    };
    (-$real: literal - $imag: literal i) => {
        cpl!(-$real, -$imag)
    };
}

use std::fmt::Debug;

#[cfg(test)]
pub(crate) use cpl;
