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
        // We can index on `y` because `std::array::from_fn` will give index in range `0..COL_NUMBER`
        // And `columns` is an array of size `COL_NUMBER`

        // Using `next` won't yield `None` because each array inside content are of size `ROW_NUMBER`
        // And each `SingleColumnIteratorValue` of `columns` has exactly `ROW_NUMBER` elements
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
    /// # Panics
    /// Never.
    ///
    pub fn transposed(&self) -> ConstMatrix<K, COL_NUMBER, ROW_NUMBER> {
        let mut columns = self.iter_all_col();
        // We can index on `y` because `std::array::from_fn` will give index in range `0..COL_NUMBER`
        // And `columns` is an array of size `COL_NUMBER`

        // Using `next` won't yield `None` because each array inside content are of size `ROW_NUMBER`
        // And each `SingleColumnIterator` of `columns` has exactly `ROW_NUMBER` elements
        let content =
            std::array::from_fn(|y| std::array::from_fn(|_| columns[y].next().unwrap_or_else(|| unreachable!()).clone()));
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
