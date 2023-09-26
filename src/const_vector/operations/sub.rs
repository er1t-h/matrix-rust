use std::ops::{Sub, SubAssign};

use crate::const_vector::ConstVector;

impl<K, const SIZE: usize> SubAssign for ConstVector<K, SIZE>
where
    K: SubAssign<K>,
{
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs -= rhs;
        }
    }
}

impl<K, const SIZE: usize> SubAssign<&Self> for ConstVector<K, SIZE>
where
    for<'a> K: SubAssign<&'a K>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs -= rhs;
        }
    }
}

impl<K, const SIZE: usize> Sub for ConstVector<K, SIZE>
where
    Self: SubAssign<Self>,
{
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<K, const SIZE: usize> Sub<&Self> for ConstVector<K, SIZE>
where
    for<'a> Self: SubAssign<&'a Self>,
{
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self {
        self -= rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::const_vector::ConstVector;

    #[test]
    fn sub_assign() {
        let mut vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        vec1 -= vec2;
        assert_eq!(vec1, ConstVector::from([-2, 0, 2]));
    }

    #[test]
    fn sub_assign_ref() {
        let mut vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        vec1 -= &vec2;
        assert_eq!(vec1, ConstVector::from([-2, 0, 2]));
        assert_eq!(vec2, ConstVector::from([3, 2, 1]));
    }

    #[test]
    fn sub() {
        let vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        assert_eq!(vec1 - vec2, ConstVector::from([-2, 0, 2]));
    }

    #[test]
    fn sub_ref() {
        let vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        assert_eq!(vec1 - &vec2, ConstVector::from([-2, 0, 2]));
        assert_eq!(vec2, ConstVector::from([3, 2, 1]));
    }
}
