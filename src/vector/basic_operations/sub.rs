use std::ops::{Sub, SubAssign};

use crate::{error::VectorOperationError, traits::SafeSub, Vector};

impl<K> SafeSub for Vector<K>
where
    for<'a> K: Clone + SubAssign<K>,
{
    type Error = VectorOperationError;
    ///
    /// Substracts another `Vector` from self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeSub;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [12, -55])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_sub_assign(&mut self, rhs: Self) -> Result<(), VectorOperationError> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs -= rhs;
        }
        Ok(())
    }
}

impl<K> SafeSub<&Self> for Vector<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Error = VectorOperationError;
    ///
    /// Substracts another `Vector` from self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeSub;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [12, -55])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_sub_assign(&mut self, rhs: &Self) -> Result<(), VectorOperationError> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs -= rhs;
        }
        Ok(())
    }
}

impl<K> SubAssign<&Self> for Vector<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}

impl<K> SubAssign for Vector<K>
where
    for<'a> K: Clone + SubAssign<K>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}

impl<K> Sub<&Self> for Vector<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<K> Sub for Vector<K>
where
    for<'a> K: Clone + SubAssign<K>,
{
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
