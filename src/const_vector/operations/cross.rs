use std::ops::{Mul, Sub};

use crate::ConstVector;

impl<K> ConstVector<K, 3>
where
    for<'a> K: Sub<K, Output = K> + Mul<K, Output = K>,
{
    pub fn cross(self, other: Self) -> Self {
        ConstVector::from([
            self.content[1] * other.content[2] - self.content[2] * other.content[1],
            self.content[2] * other.content[0] - self.content[0] * other.content[2],
            self.content[0] * other.content[1] - self.content[1] * other.content[0],
        ])
    }
}
