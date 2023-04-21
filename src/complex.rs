#[derive(Clone, Copy, PartialEq, Debug, Default, Hash)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

mod operations;
mod utils;

macro_rules! cpl {
    ($real: expr, $imag: expr) => {
        crate::Complex::new($real, $imag)
    };
}

pub(crate) use cpl;
