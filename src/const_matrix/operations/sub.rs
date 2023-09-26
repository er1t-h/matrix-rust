use std::ops::{Sub, SubAssign};

use crate::const_matrix::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> SubAssign
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    K: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self
            .content
            .iter_mut()
            .flatten()
            .zip(rhs.content.into_iter().flatten())
        {
            *lhs -= rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> SubAssign<&Self>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> K: SubAssign<&'a K>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        for (lhs, rhs) in self
            .content
            .iter_mut()
            .flatten()
            .zip(rhs.content.iter().flatten())
        {
            *lhs -= rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Sub
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    Self: SubAssign,
{
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Sub<&Self>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> Self: SubAssign<&'a Self>,
{
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn sub_assign() {
        let mut m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        m1 -= m2;
        assert_eq!(m1, ConstMatrix::from([[-3, -1], [1, 3]]));
    }

    #[test]
    fn sub_assign_ref() {
        let mut m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        m1 -= &m2;
        assert_eq!(m1, ConstMatrix::from([[-3, -1], [1, 3]]));
        assert_eq!(m2, ConstMatrix::from([[4, 3], [2, 1]]));
    }

    #[test]
    fn sub() {
        let m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        assert_eq!(m1 - m2, ConstMatrix::from([[-3, -1], [1, 3]]));
    }

    #[test]
    fn sub_ref() {
        let m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        assert_eq!(m1 - &m2, ConstMatrix::from([[-3, -1], [1, 3]]));
        assert_eq!(m2, ConstMatrix::from([[4, 3], [2, 1]]));
    }
}
