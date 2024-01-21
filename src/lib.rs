#![deny(unsafe_op_in_unsafe_fn)]
#![warn(
    clippy::all,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    clippy::correctness,
    clippy::cognitive_complexity,
    clippy::suspicious,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::enum_glob_use,
    clippy::unwrap_used
)]
#![allow(
    clippy::inline_always,
    clippy::cargo_common_metadata,
    clippy::float_cmp
)]

pub mod complex;
pub mod const_matrix;
pub mod const_vector;
pub mod error;
pub mod macros;
pub mod matrix;
pub mod traits;
pub mod utils;
pub mod vector;

mod static_asserts;

pub use crate::complex::Complex;
pub use crate::const_matrix::ConstMatrix;
pub use crate::const_vector::ConstVector;
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
