#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::inline_always)]

pub mod complex;
pub mod error;
pub mod matrix;
pub mod traits;
pub mod utils;
pub mod vector;

pub use crate::complex::Complex;
pub use crate::matrix::Matrix;
pub use crate::vector::Vector;

#[cfg(test)]
macro_rules! assert_eq_float {
    ($lhs: expr, $rhs: expr, $($args: expr),+) => {
        let diff = $lhs - $rhs as f64;
        assert!(diff.abs() < 0.0001, $($args),*)
    };
    ($lhs: expr, $rhs: expr) => {
        let diff = $lhs - $rhs as f64;
        assert!(diff.abs() < 0.0001, "Error at assertion {logfilename}:{loglinenumber}\nlhs: `{}`\nrhs: `{}`", $lhs, $rhs, logfilename=file!(), loglinenumber=line!())
    };
}

#[cfg(test)]
pub(crate) use assert_eq_float;
