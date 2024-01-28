use std::{iter::Sum, ops::Mul};

use crate::{traits::Sqrt, ConstVector};

impl<K, const SIZE: usize> ConstVector<K, SIZE>
where
    for<'a> &'a K: Mul<&'a K, Output = K>,
    K: Sum + Sqrt,
{
    pub fn magnitude(&self) -> K {
        self.content.iter().map(|x| x * x).sum::<K>().sqrt()
    }
}
