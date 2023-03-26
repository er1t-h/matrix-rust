pub mod error;
pub mod matrix;
pub mod traits;
pub mod utils;
pub mod vector;

pub use crate::matrix::Matrix;
pub use crate::vector::Vector;

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

pub(crate) use assert_eq_float;
