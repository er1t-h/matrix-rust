mod ex00;

mod ex01;
pub use ex01::{linear_combination, linear_combination_unchecked};

mod ex03;
mod ex04;

mod ex05;
mod ex06;

mod utils;

///
/// Implementation of a Vector
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vector<K: Clone> {
    content: Vec<K>,
}
