use std::ops::{AddAssign, Mul};

use crate::{const_matrix::ConstMatrix, const_vector::ConstVector};

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    Mul<ConstMatrix<K, ROW_NUMBER, COL_NUMBER>> for ConstVector<K, COL_NUMBER>
where
    K: Clone + Mul<K, Output = K> + AddAssign,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER>) -> Self::Output {
        let vector_elements = self.content.into_iter();
        let matrix_columns = rhs.iter_all_col_value();

        Self::mul_val_val(vector_elements, matrix_columns)
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    Mul<&ConstMatrix<K, ROW_NUMBER, COL_NUMBER>> for ConstVector<K, COL_NUMBER>
where
    for<'a> K: Clone + Mul<&'a K, Output = K> + AddAssign,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: &ConstMatrix<K, ROW_NUMBER, COL_NUMBER>) -> Self::Output {
        let mut vector_elements = self.content.into_iter();
        let mut matrix_columns = rhs.iter_all_col();

        let first_vector_elt = vector_elements.next().unwrap();

        let mut array: [K; ROW_NUMBER] =
            std::array::from_fn(|_| first_vector_elt.clone() * matrix_columns[0].next().unwrap());

        for (vector_element, matrix_column) in
            vector_elements.zip(matrix_columns.into_iter().skip(1))
        {
            for (emplace, matrix_element) in array.iter_mut().zip(matrix_column) {
                *emplace += vector_element.clone() * matrix_element;
            }
        }
        ConstVector::from(array)
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    Mul<ConstMatrix<K, ROW_NUMBER, COL_NUMBER>> for &ConstVector<K, COL_NUMBER>
where
    for<'a> K: AddAssign,
    for<'a> &'a K: Mul<K, Output = K>,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER>) -> Self::Output {
        let mut vector_elements = self.content.iter();
        let mut matrix_columns = rhs.iter_all_col_value();

        let first_vector_elt = vector_elements.next().unwrap();

        let mut array: [K; ROW_NUMBER] =
            std::array::from_fn(|_| first_vector_elt * matrix_columns[0].next().unwrap());

        for (vector_element, matrix_column) in
            vector_elements.zip(matrix_columns.into_iter().skip(1))
        {
            for (emplace, matrix_element) in array.iter_mut().zip(matrix_column) {
                *emplace += vector_element * matrix_element;
            }
        }
        ConstVector::from(array)
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    Mul<&ConstMatrix<K, ROW_NUMBER, COL_NUMBER>> for &ConstVector<K, COL_NUMBER>
where
    for<'a> K: AddAssign,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    type Output = ConstVector<K, ROW_NUMBER>;
    fn mul(self, rhs: &ConstMatrix<K, ROW_NUMBER, COL_NUMBER>) -> Self::Output {
        let mut vector_elements = self.content.iter();
        let mut matrix_columns = rhs.iter_all_col();

        let first_vector_elt = vector_elements.next().unwrap();

        let mut array: [K; ROW_NUMBER] =
            std::array::from_fn(|_| first_vector_elt * matrix_columns[0].next().unwrap());

        for (vector_element, matrix_column) in
            vector_elements.zip(matrix_columns.into_iter().skip(1))
        {
            for (emplace, matrix_element) in array.iter_mut().zip(matrix_column) {
                *emplace += vector_element * matrix_element;
            }
        }
        ConstVector::from(array)
    }
}

impl<K, const COL_NUMBER: usize> ConstVector<K, COL_NUMBER> {
    fn mul_val_val<IVec, IMat, const ROW_NUMBER: usize>(
        mut vector_elements: IVec,
        mut matrix_columns: [IMat; COL_NUMBER],
    ) -> ConstVector<K, ROW_NUMBER>
    where
        IVec: Iterator<Item = K>,
        IMat: Iterator<Item = K>,
        K: Clone + Mul<K, Output = K> + AddAssign,
    {
        // A vector has at least one element, so the unwrap never fails
        let first_vector_elt = vector_elements.next().unwrap();

        // A column of a matrix has ROW_NUMBER entries, so the unwrap never fails
        let mut array: [K; ROW_NUMBER] =
            std::array::from_fn(|_| first_vector_elt.clone() * matrix_columns[0].next().unwrap());

        // Here:
        //     Vector element starts at [1] (we already consumed the first)
        //     Matrix column must also start at [1], because we already went through the first column
        for (vector_element, matrix_column) in
            vector_elements.zip(matrix_columns.into_iter().skip(1))
        {
            //     Emplace starts at [0]
            //     We start by the first element of each column
            for (emplace, matrix_element) in array.iter_mut().zip(matrix_column) {
                *emplace += vector_element.clone() * matrix_element;
            }
        }
        ConstVector::from(array)
    }

    fn mul_val_ref<IVec, IMat, const ROW_NUMBER: usize>(
        mut vector_elements: IVec,
        mut matrix_columns: [IMat; COL_NUMBER],
    ) -> ConstVector<K, ROW_NUMBER>
    where
        IVec: Iterator<Item = K>,
        IMat: Iterator<Item = &K>,
        K: Clone + Mul<K, Output = K> + AddAssign,
    {
        let first_vector_elt = vector_elements.next().unwrap();

        let mut array: [K; ROW_NUMBER] =
            std::array::from_fn(|_| first_vector_elt.clone() * matrix_columns[0].next().unwrap());

        for (vector_element, matrix_column) in
            vector_elements.zip(matrix_columns.into_iter().skip(1))
        {
            for (emplace, matrix_element) in array.iter_mut().zip(matrix_column) {
                *emplace += vector_element.clone() * matrix_element;
            }
        }
        ConstVector::from(array)
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
            assert_eq!(v * u, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(v * u, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(v * u, ConstVector::from([4., -4.]));
        }
    }

    #[test]
    fn example_vec_mat_ref() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(v * &u, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(v * &u, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(v * &u, ConstVector::from([4., -4.]));
        }
    }

    #[test]
    fn example_vec_ref_mat_val() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&v * u, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&v * u, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&v * u, ConstVector::from([4., -4.]));
        }
    }

    #[test]
    fn example_vec_ref_mat_ref() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&v * &u, ConstVector::from([4., 2.]));
        }
        {
            let u = ConstMatrix::from([[2., 0.], [0., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&v * &u, ConstVector::from([8., 4.]));
        }
        {
            let u = ConstMatrix::from([[2., -2.], [-2., 2.]]);
            let v = ConstVector::from([4., 2.]);
            assert_eq!(&v * &u, ConstVector::from([4., -4.]));
        }
    }
}
