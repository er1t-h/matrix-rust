use std::ops::{Mul, MulAssign};

use crate::Vector;

impl<K> MulAssign<&K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    ///
    /// Multiplies a scalar into self.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// lhs *= 3;
    /// assert_eq!(lhs, [45, 6])
    /// ```
    ///
    /// # Complexity:
    /// Linear in the `size` of the `self`.
    ///
    #[inline]
    fn mul_assign(&mut self, rhs: &K) {
        for elt in &mut self.content {
            *elt *= rhs;
        }
    }
}

impl<K> MulAssign<K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<K>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: K) {
        for elt in &mut self.content {
            *elt *= rhs.clone();
        }
    }
}

impl<K> Mul<&K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: &K) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<K> Mul<K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<K>,
{
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: K) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<K> Mul<&K> for &Vector<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Vector<K>;
    #[inline]
    fn mul(self, rhs: &K) -> Self::Output {
        let mut ret = self.clone();
        ret *= rhs;
        ret
    }
}

impl<K> Mul<K> for &Vector<K>
where
    for<'a> K: Clone + MulAssign<K>,
{
    type Output = Vector<K>;
    #[inline]
    fn mul(self, rhs: K) -> Self::Output {
        let mut ret = self.clone();
        ret *= rhs;
        ret
    }
}
