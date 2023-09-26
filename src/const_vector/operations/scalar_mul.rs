use std::ops::{Mul, MulAssign};

use crate::const_vector::ConstVector;

impl<K, const SIZE: usize> MulAssign<K> for ConstVector<K, SIZE>
where
    for<'a> K: MulAssign<&'a K>,
{
    fn mul_assign(&mut self, rhs: K) {
        for lhs in &mut self.content {
            *lhs *= &rhs;
        }
    }
}

impl<K, const SIZE: usize> MulAssign<&K> for ConstVector<K, SIZE>
where
    for<'a> K: MulAssign<&'a K>,
{
    fn mul_assign(&mut self, rhs: &K) {
        for lhs in &mut self.content {
            *lhs *= rhs;
        }
    }
}

impl<K, const SIZE: usize> Mul<K> for ConstVector<K, SIZE>
where
    Self: MulAssign<K>,
{
    type Output = Self;
    fn mul(mut self, rhs: K) -> Self {
        self *= rhs;
        self
    }
}

impl<K, const SIZE: usize> Mul<&K> for ConstVector<K, SIZE>
where
    for<'a> Self: MulAssign<&'a K>,
{
    type Output = Self;
    fn mul(mut self, rhs: &K) -> Self {
        self *= rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::const_vector::ConstVector;

    #[test]
    fn mul_assign() {
        let mut vec1 = ConstVector::from([1, 2, 3]);
        vec1 *= 4;
        assert_eq!(vec1, ConstVector::from([4, 8, 12]));
    }

    #[test]
    fn mul_assign_ref() {
        let mut vec1 = ConstVector::from([1, 2, 3]);
        vec1 *= &3;
        assert_eq!(vec1, ConstVector::from([3, 6, 9]));
    }

    #[test]
    fn mul() {
        let vec1 = ConstVector::from([1, 2, 3]);
        assert_eq!(vec1 * 3, ConstVector::from([3, 6, 9]));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn mul_ref() {
        let vec1 = ConstVector::from([1, 2, 3]);
        assert_eq!(vec1 * &3, ConstVector::from([3, 6, 9]));
    }
}
