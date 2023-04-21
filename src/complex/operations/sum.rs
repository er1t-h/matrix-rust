use std::{iter::Sum, ops::AddAssign};

use crate::Complex;

impl<T> Sum for Complex<T>
where
    for<'a> T: AddAssign<&'a T> + Default,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Complex::default(), |mut acc, new| {
            acc += new;
            acc
        })
    }
}
