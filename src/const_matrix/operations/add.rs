use std::ops::{Add, AddAssign};

use crate::const_matrix::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> AddAssign
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    K: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self
            .content
            .iter_mut()
            .flatten()
            .zip(rhs.content.into_iter().flatten())
        {
            *lhs += rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> AddAssign<&Self>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> K: AddAssign<&'a K>,
{
    fn add_assign(&mut self, rhs: &Self) {
        for (lhs, rhs) in self
            .content
            .iter_mut()
            .flatten()
            .zip(rhs.content.iter().flatten())
        {
            *lhs += rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Add
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    Self: AddAssign,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Add<&Self>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> Self: AddAssign<&'a Self>,
{
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn add_assign() {
        let mut m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        m1 += m2;
        assert_eq!(m1, ConstMatrix::from([[5, 5], [5, 5]]));
    }

    #[test]
    fn add_assign_ref() {
        let mut m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        m1 += &m2;
        assert_eq!(m1, ConstMatrix::from([[5, 5], [5, 5]]));
        assert_eq!(m2, ConstMatrix::from([[4, 3], [2, 1]]));
    }

    #[test]
    fn add() {
        let m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        assert_eq!(m1 + m2, ConstMatrix::from([[5, 5], [5, 5]]));
    }

    #[test]
    fn add_ref() {
        let m1 = ConstMatrix::from([[1, 2], [3, 4]]);
        let m2 = ConstMatrix::from([[4, 3], [2, 1]]);
        assert_eq!(m1 + &m2, ConstMatrix::from([[5, 5], [5, 5]]));
        assert_eq!(m2, ConstMatrix::from([[4, 3], [2, 1]]));
    }
}
