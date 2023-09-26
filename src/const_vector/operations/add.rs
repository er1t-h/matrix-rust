use std::ops::{Add, AddAssign};

use crate::const_vector::ConstVector;

impl<K, const SIZE: usize> AddAssign for ConstVector<K, SIZE>
where
    K: AddAssign<K>,
{
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs += rhs;
        }
    }
}

impl<K, const SIZE: usize> AddAssign<&Self> for ConstVector<K, SIZE>
where
    for<'a> K: AddAssign<&'a K>,
{
    fn add_assign(&mut self, rhs: &Self) {
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs += rhs;
        }
    }
}

impl<K, const SIZE: usize> Add for ConstVector<K, SIZE>
where
    Self: AddAssign<Self>,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<K, const SIZE: usize> Add<&Self> for ConstVector<K, SIZE>
where
    for<'a> Self: AddAssign<&'a Self>,
{
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::const_vector::ConstVector;

    #[test]
    fn add_assign() {
        let mut vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        vec1 += vec2;
        assert_eq!(vec1, ConstVector::from([4, 4, 4]));
    }

    #[test]
    fn add_assign_ref() {
        let mut vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        vec1 += &vec2;
        assert_eq!(vec1, ConstVector::from([4, 4, 4]));
        assert_eq!(vec2, ConstVector::from([3, 2, 1]));
    }

    #[test]
    fn add() {
        let vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        assert_eq!(vec1 + vec2, ConstVector::from([4, 4, 4]));
    }

    #[test]
    fn add_ref() {
        let vec1 = ConstVector::from([1, 2, 3]);
        let vec2 = ConstVector::from([3, 2, 1]);
        assert_eq!(vec1 + &vec2, ConstVector::from([4, 4, 4]));
        assert_eq!(vec2, ConstVector::from([3, 2, 1]));
    }
}
