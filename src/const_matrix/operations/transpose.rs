use crate::const_matrix::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    ///
    /// Returns the transpose the matrix
    ///
    /// # Panics
    /// Never.
    ///
    pub fn transpose(self) -> ConstMatrix<K, COL_NUMBER, ROW_NUMBER> {
        let mut columns = self.iter_all_col_value();
        let content = std::array::from_fn(|y| std::array::from_fn(|_| columns[y].next().unwrap()));
        ConstMatrix { content }
    }
}

impl<K: Clone, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    ///
    /// Returns a transposed version of the matrix
    ///
    /// # Panics
    /// Never.
    ///
    pub fn transposed(&self) -> ConstMatrix<K, COL_NUMBER, ROW_NUMBER> {
        let mut columns = self.iter_all_col();
        let content =
            std::array::from_fn(|y| std::array::from_fn(|_| columns[y].next().unwrap().clone()));
        ConstMatrix { content }
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn basic() {
        let mat = ConstMatrix::from([[1, 2, 3], [4, 5, 6]]);
        let res = mat.transpose();
        assert_eq!(res, ConstMatrix::from([[1, 4], [2, 5], [3, 6]]));
        let mat = ConstMatrix::from([[1]]);
        let res = mat.transpose();
        assert_eq!(res, ConstMatrix::from([[1]]));
        let mat = ConstMatrix::from([[1], [2]]);
        let res = mat.transpose();
        assert_eq!(res, ConstMatrix::from([[1, 2]]));
    }
}
