use std::ops::{Sub, SubAssign};

use crate::{error::MatrixOperationError, traits::SafeSub, Matrix};

impl<'a, K> SafeSub<&'a Self> for Matrix<K>
where
    K: Clone + SubAssign<&'a K>,
{
    type Error = MatrixOperationError;
    ///
    /// Subs another `Matrix` from self.
    ///
    /// If the size of the two Matrixes differ, a [`MatrixOperationError`] is returned
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    /// use matrix::traits::SafeSub;
    ///
    /// let mut lhs = Matrix::from([[10, 10], [10, 10]]);
    /// let rhs = Matrix::from([[5, 6], [7, 8]]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [[5, 4], [3, 2]])
    /// ```
    ///
    /// # Errors
    /// If the dimensions of the two matrix are different, returns a
    /// [`NotSameSize`](MatrixOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    fn safe_sub_assign(&mut self, rhs: &'a Self) -> Result<(), Self::Error> {
        if self.dimensions != rhs.dimensions {
            return Err(MatrixOperationError::NotSameSize(
                self.dimensions,
                rhs.dimensions,
            ));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs) {
            *lhs -= rhs;
        }
        Ok(())
    }
}
impl<K> SafeSub for Matrix<K>
where
    K: Clone + SubAssign<K>,
{
    type Error = MatrixOperationError;
    ///
    /// Subs another `Matrix` from self.
    ///
    /// If the size of the two Matrixes differ, a [`MatrixOperationError`] is returned
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    /// use matrix::traits::SafeSub;
    ///
    /// let mut lhs = Matrix::from([[10, 10], [10, 10]]);
    /// let rhs = Matrix::from([[5, 6], [7, 8]]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [[5, 4], [3, 2]])
    /// ```
    ///
    /// # Errors
    /// If the dimensions of the two matrix are different, returns a
    /// [`NotSameSize`](MatrixOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    fn safe_sub_assign(&mut self, rhs: Self) -> Result<(), Self::Error> {
        if self.dimensions != rhs.dimensions {
            return Err(MatrixOperationError::NotSameSize(
                self.dimensions,
                rhs.dimensions,
            ));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs) {
            *lhs -= rhs;
        }
        Ok(())
    }
}

impl<K> SubAssign<&Self> for Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}
impl<K> SubAssign for Matrix<K>
where
    K: Clone + SubAssign<K>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}
impl<K> Sub<&Self> for Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<K> Sub for Matrix<K>
where
    K: Clone + SubAssign<K>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
