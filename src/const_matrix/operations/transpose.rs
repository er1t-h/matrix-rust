use crate::const_matrix::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    ///
    /// Returns the transpose the matrix
    ///
    pub fn transpose(self) -> ConstMatrix<K, COL_NUMBER, ROW_NUMBER> {
        let mut columns = self.iter_all_col_value();
        // Since we're transposing a COL_NUMBER x ROW_NUMBER into a ROW_NUMBER x COL_NUMBER matrix,
        // all iterators will be completely used; no more, no less
        let content = std::array::from_fn(|y| std::array::from_fn(|_| columns[y].next().unwrap_or_else(|| unreachable!())));
        ConstMatrix { content }
    }
}

impl<K: Clone, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    ///
    /// Returns a transposed version of the matrix
    ///
    pub fn transposed(&self) -> ConstMatrix<K, COL_NUMBER, ROW_NUMBER> {
        self.clone().transpose()
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
