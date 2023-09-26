use std::ops::{Add, AddAssign};

use crate::{error::MatrixOperationError, traits::SafeAdd, Matrix};

impl<'a, K> SafeAdd<&'a Self> for Matrix<K>
where
    K: Clone + AddAssign<&'a K>,
{
    type Error = MatrixOperationError;
    ///
    /// Adds another `Matrix` to self.
    ///
    /// If the size of the two Matrixes differ, a [`MatrixOperationError`] is returned
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    /// use matrix::traits::SafeAdd;
    ///
    /// let mut lhs = Matrix::from([[5, 4], [3, 2]]);
    /// let rhs = Matrix::from([[5, 6], [7, 8]]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [[10, 10], [10, 10]])
    /// ```
    ///
    /// # Errors
    /// If the dimensions of the two matrix are different, returns a
    /// [`NotSameSize`](MatrixOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    fn safe_add_assign(&mut self, rhs: &'a Self) -> Result<(), Self::Error> {
        if self.dimensions != rhs.dimensions {
            return Err(MatrixOperationError::NotSameSize(
                self.dimensions,
                rhs.dimensions,
            ));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> SafeAdd for Matrix<K>
where
    K: Clone + AddAssign<K>,
{
    type Error = MatrixOperationError;
    ///
    /// Adds another `Matrix` to self.
    ///
    /// If the size of the two Matrixes differ, a [`MatrixOperationError`] is returned
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    /// use matrix::traits::SafeAdd;
    ///
    /// let mut lhs = Matrix::from([[5, 4], [3, 2]]);
    /// let rhs = Matrix::from([[5, 6], [7, 8]]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [[10, 10], [10, 10]])
    /// ```
    ///
    /// # Errors
    /// If the dimensions of the two matrix are different, returns a
    /// [`NotSameSize`](MatrixOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    fn safe_add_assign(&mut self, rhs: Self) -> Result<(), Self::Error> {
        if self.dimensions != rhs.dimensions {
            return Err(MatrixOperationError::NotSameSize(
                self.dimensions,
                rhs.dimensions,
            ));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> AddAssign<&Self> for Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        let _ = self.safe_add_assign(rhs);
    }
}
impl<K> AddAssign for Matrix<K>
where
    K: Clone + AddAssign<K>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        let _ = self.safe_add_assign(rhs);
    }
}
impl<K> Add<&Self> for Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<K> Add for Matrix<K>
where
    K: Clone + AddAssign<K>,
{
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
