use std::{iter::Sum, ops::Mul};

use crate::ConstVector;

impl<K, const SIZE: usize> ConstVector<K, SIZE>
where
    K: Mul<K, Output = K> + Sum,
{
    pub fn dot(self, other: Self) -> K {
        self.content
            .into_iter()
            .zip(other.content)
            .map(|(x, y)| x * y)
            .sum()
    }
}
