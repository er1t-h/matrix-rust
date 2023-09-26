use std::ops::{Add, AddAssign};

use crate::{error::VectorOperationError, traits::SafeAdd, Vector};

impl<K> SafeAdd for Vector<K>
where
    for<'a> K: Clone + AddAssign<K>,
{
    type Error = VectorOperationError;
    ///
    /// Adds another `Vector` to self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeAdd;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [18, 59])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_add_assign(&mut self, rhs: Self) -> Result<(), Self::Error> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> SafeAdd<&Self> for Vector<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Error = VectorOperationError;
    ///
    /// Adds another `Vector` to self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeAdd;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [18, 59])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_add_assign(&mut self, rhs: &Self) -> Result<(), Self::Error> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> AddAssign<&Self> for Vector<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        let _ = self.safe_add_assign(rhs);
    }
}

impl<K> AddAssign for Vector<K>
where
    for<'a> K: Clone + AddAssign<K>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let _ = self.safe_add_assign(rhs);
    }
}

impl<K> Add<&Self> for Vector<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<K> Add for Vector<K>
where
    for<'a> K: Clone + AddAssign<K>,
{
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
