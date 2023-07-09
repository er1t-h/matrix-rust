use std::ops::Index;

use crate::static_asserts::AssertNonZero;

mod operations;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstVector<K, const SIZE: usize> {
    content: [K; SIZE],
}

impl<K, const SIZE: usize> From<[K; SIZE]> for ConstVector<K, SIZE> {
    #[allow(clippy::no_effect, path_statements)]
    fn from(vector: [K; SIZE]) -> Self {
        AssertNonZero::<SIZE>::OK;
        Self { content: vector }
    }
}

impl<K, const SIZE: usize> Index<usize> for ConstVector<K, SIZE> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}
