mod basic_operations;

mod linear_combination;
pub use linear_combination::{linear_combination, linear_combination_unchecked};

mod dot_product;
mod norms;

mod angle_cosine;
mod cross_product;

mod utils;

///
/// Implementation of a Vector
///
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Vector<K: Clone> {
    content: Vec<K>,
}
