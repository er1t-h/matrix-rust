use std::ops::MulAssign;

use crate::{const_matrix::ConstMatrix, traits::TermToTerm};

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> TermToTerm
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    K: MulAssign,
{
    fn term_to_term_mul_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self
            .content
            .iter_mut()
            .flatten()
            .zip(rhs.content.into_iter().flatten())
        {
            *lhs *= rhs;
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> TermToTerm<&Self>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> K: MulAssign<&'a K>,
{
    fn term_to_term_mul_assign(&mut self, rhs: &Self) {
        for (lhs, rhs) in self
            .content
            .iter_mut()
            .flatten()
            .zip(rhs.content.iter().flatten())
        {
            *lhs *= rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    use super::TermToTerm;

    #[test]
    fn mul_assign() {
        let mut m1 = ConstMatrix::from([[5, 3], [2, 4]]);
        let m2 = ConstMatrix::from([[1, 2], [2, 3]]);
        m1.term_to_term_mul_assign(m2);
        assert_eq!(m1, ConstMatrix::from([[5, 6], [4, 12]]));
    }

    #[test]
    fn mul_assign_ref() {
        let mut m1 = ConstMatrix::from([[5, 3], [2, 4]]);
        let m2 = ConstMatrix::from([[1, 2], [2, 3]]);
        m1.term_to_term_mul_assign(&m2);
        assert_eq!(m1, ConstMatrix::from([[5, 6], [4, 12]]));
        assert_eq!(m2, ConstMatrix::from([[1, 2], [2, 3]]));
    }

    #[test]
    fn mul() {
        let m1 = ConstMatrix::from([[5, 3], [2, 4]]);
        let m2 = ConstMatrix::from([[1, 2], [2, 3]]);
        assert_eq!(
            m1.term_to_term_mul(m2),
            ConstMatrix::from([[5, 6], [4, 12]])
        );
    }

    #[test]
    fn mul_ref() {
        let m1 = ConstMatrix::from([[5, 3], [2, 4]]);
        let m2 = ConstMatrix::from([[1, 2], [2, 3]]);
        assert_eq!(
            m1.term_to_term_mul(&m2),
            ConstMatrix::from([[5, 6], [4, 12]])
        );
        assert_eq!(m2, ConstMatrix::from([[1, 2], [2, 3]]));
    }
}
