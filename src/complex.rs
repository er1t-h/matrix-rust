#[derive(Clone, Copy, PartialEq, Debug, Default, Hash)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

mod operations;
mod utils;

#[cfg(test)]
macro_rules! cpl {
    ($real: expr, $imag: expr) => {
        crate::Complex::new($real, $imag)
    };
}

#[cfg(test)]
pub(crate) use cpl;
