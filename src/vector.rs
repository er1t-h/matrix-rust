mod ex00;

mod ex01;
pub use ex01::{linear_combination, safe_linear_combination};

mod ex03;
mod ex04;

mod utils;

///
/// Implementation of a Vector
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vector<K: Clone> {
    content: Vec<K>,
}
