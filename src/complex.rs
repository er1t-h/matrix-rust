#[derive(Clone, Copy, PartialEq, Debug, Default, Hash)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

mod operations;
mod utils;
