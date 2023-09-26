use crate::{const_matrix::ConstMatrix, const_vector::ConstVector};
use std::ops::Mul;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Mul<ConstVector<K, COL_NUMBER>>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    ConstVector<K, COL_NUMBER>: Mul<Self, Output = ConstVector<K, ROW_NUMBER>>,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: ConstVector<K, COL_NUMBER>) -> Self::Output {
        rhs * self
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Mul<&ConstVector<K, COL_NUMBER>>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> &'a ConstVector<K, COL_NUMBER>: Mul<Self, Output = ConstVector<K, ROW_NUMBER>>,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: &ConstVector<K, COL_NUMBER>) -> Self::Output {
        rhs * self
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Mul<ConstVector<K, COL_NUMBER>>
    for &ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    ConstVector<K, COL_NUMBER>: Mul<Self, Output = ConstVector<K, ROW_NUMBER>>,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: ConstVector<K, COL_NUMBER>) -> Self::Output {
        rhs * self
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Mul<&ConstVector<K, COL_NUMBER>>
    for &ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
where
    for<'a> &'a ConstVector<K, COL_NUMBER>: Mul<Self, Output = ConstVector<K, ROW_NUMBER>>,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: &ConstVector<K, COL_NUMBER>) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod test {
    use crate::{const_matrix::ConstMatrix, const_vector::ConstVector};

    #[test]
    fn example_vec() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(u * v, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(u * v, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(u * v, ConstVector::from([4., -4.]));
        }
    }

    #[test]
    fn example_vec_mat_ref() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&u * v, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&u * v, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&u * v, ConstVector::from([4., -4.]));
        }
    }

    #[test]
    fn example_vec_ref_mat_val() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(u * &v, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(u * &v, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(u * &v, ConstVector::from([4., -4.]));
        }
    }

    #[test]
    fn example_vec_ref_mat_ref() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&u * &v, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&u * &v, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&u * &v, ConstVector::from([4., -4.]));
        }
    }
}
