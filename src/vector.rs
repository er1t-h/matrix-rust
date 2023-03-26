mod ex00;

mod ex01;
pub use ex01::{linear_combination, safe_linear_combination};

mod utils;

use crate::traits::Space;

///
/// Implementation of a Vector
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vector<K: Space> {
    content: Vec<K>,
}
