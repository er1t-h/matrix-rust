use std::{
    iter::Sum,
    ops::{DivAssign, Mul},
};

use crate::{traits::Sqrt, ConstVector};

impl<K, const SIZE: usize> ConstVector<K, SIZE>
where
    for<'a> &'a K: Mul<&'a K, Output = K>,
    for<'a> K: Sum + Sqrt + DivAssign<&'a K>,
{
    #[must_use]
    pub fn normalize(mut self) -> Self {
        let magnitude = self.magnitude();
        self.content.iter_mut().for_each(|x| *x /= &magnitude);
        self
    }
}
