use std::ops::{Mul, MulAssign};

use crate::const_matrix::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> MulAssign<K>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> K: MulAssign<&'a K>,
{
    fn mul_assign(&mut self, rhs: K) {
        for lhs in self.content.iter_mut().flatten() {
            *lhs *= &rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> MulAssign<&K>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> K: MulAssign<&'a K>,
{
    fn mul_assign(&mut self, rhs: &K) {
        for lhs in self.content.iter_mut().flatten() {
            *lhs *= rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Mul<K>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    Self: MulAssign<K>,
{
    type Output = Self;
    fn mul(mut self, rhs: K) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Mul<&K>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> Self: MulAssign<&'a K>,
{
    type Output = Self;
    fn mul(mut self, rhs: &K) -> Self::Output {
        self *= rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn mul_assign() {
        let mut m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        m1 *= 5;
        assert_eq!(m1, ConstMatrix::from([[5, 10], [15, 20]]));
    }

    #[test]
    fn mul_assign_ref() {
        let mut m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let multiplicand = 3;
        m1 *= &3;
        assert_eq!(m1, ConstMatrix::from([[3, 6], [9, 12]]));
        assert_eq!(multiplicand, 3);
    }

    #[test]
    fn mul() {
        let m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        assert_eq!(m1 * 2, ConstMatrix::from([[2, 4], [6, 8]]));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn mul_ref() {
        let m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let multiplicand = 9;
        assert_eq!(m1 * &multiplicand, ConstMatrix::from([[9, 18], [27, 36]]));
        assert_eq!(multiplicand, 9);
    }
}
