use std::ops::{Mul, MulAssign};

use crate::Matrix;

impl<K> MulAssign<&K> for Matrix<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    ///
    /// Multiply a scalar into self.
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut lhs = Matrix::from([[5, 10], [6, 3]]);
    /// lhs *= 5;
    /// assert_eq!(lhs, [[25, 50], [30, 15]])
    /// ```
    /// Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    fn mul_assign(&mut self, rhs: &K) {
        for nb in &mut self.content {
            *nb *= rhs;
        }
    }
}
impl<K> MulAssign<K> for Matrix<K>
where
    K: Clone + MulAssign<K>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: K) {
        for nb in &mut self.content {
            *nb *= rhs.clone();
        }
    }
}
impl<K> Mul<&K> for Matrix<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: &K) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<K> Mul<K> for Matrix<K>
where
    K: Clone + MulAssign<K>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: K) -> Self::Output {
        self *= rhs;
        self
    }
}
