use std::ops::Index;

mod operations;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstVector<K, const SIZE: usize> {
    content: [K; SIZE],
}

impl<K, const SIZE: usize> From<[K; SIZE]> for ConstVector<K, SIZE> {
    fn from(vector: [K; SIZE]) -> Self {
        Self { content: vector }
    }
}

impl<K, const SIZE: usize> Index<usize> for ConstVector<K, SIZE> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}
