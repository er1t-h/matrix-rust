use std::mem::ManuallyDrop;

use self::iter_val::SingleColumnIteratorValue;

use super::ConstMatrix;

mod iter;
mod iter_mut;
mod iter_val;
pub use iter::SingleColumnIterator;
pub use iter_mut::SingleColumnIteratorMut;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    ///
    /// Returns an iterator over a single column of the matrix.
    ///
    /// The iterator yields all reference in the given `column` of the matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::const_matrix::ConstMatrix;
    ///
    /// let mat = ConstMatrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    /// let mut iter = mat.iter_col(1);
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&8));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    pub fn iter_col(&self, column: usize) -> SingleColumnIterator<'_, K, ROW_NUMBER, COL_NUMBER> {
        SingleColumnIterator::new(self, column)
    }

    ///
    /// Returns an iterator over a single column of the matrix, yielding values.
    ///
    /// The iterator yields all values in the given `column` of the matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::const_matrix::ConstMatrix;
    ///
    /// let mat = ConstMatrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    /// let mut iter = mat.iter_col_value(1);
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), Some(8));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    pub fn iter_col_value(
        self,
        column: usize,
    ) -> SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER> {
        SingleColumnIteratorValue::new(self, column)
    }

    ///
    /// Returns an iterator over a single column of the matrix, yielding mutable references.
    ///
    /// The iterator yields all reference in the given `column` of the matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::const_matrix::ConstMatrix;
    ///
    /// let mut mat = ConstMatrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    /// let mut iter = mat.iter_col_mut(1);
    /// *iter.next().unwrap() = 3;
    /// *iter.next().unwrap() = 6;
    /// *iter.next().unwrap() = 9;
    /// assert_eq!(mat, ConstMatrix::from([[1, 3, 3], [4, 6, 6], [7, 9, 9]]));
    /// ```
    ///
    pub fn iter_col_mut(
        &mut self,
        column: usize,
    ) -> SingleColumnIteratorMut<'_, K, ROW_NUMBER, COL_NUMBER> {
        SingleColumnIteratorMut::new(self, column)
    }

    ///
    /// Returns an array of iterator, over each of the columns of the matrix.
    ///
    /// The iterator yields all values in each column of the matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::ConstMatrix;
    ///
    /// let matrix = ConstMatrix::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// ]);
    /// let mut iter_columns = matrix.iter_all_col_value();
    /// assert_eq!(iter_columns[0].next(), Some(1));
    /// assert_eq!(iter_columns[1].next(), Some(2));
    /// assert_eq!(iter_columns[2].next(), Some(3));
    /// assert_eq!(iter_columns[2].next(), Some(6));
    /// assert_eq!(iter_columns[1].next(), Some(5));
    /// assert_eq!(iter_columns[0].next(), Some(4));
    /// ```
    ///
    pub fn iter_all_col_value(
        self,
    ) -> [SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>; COL_NUMBER] {
        let mat = ManuallyDrop::new(self);
        // ! About unsafe:
        // ! We know that each [`SingleColumnIteratorValue`] will go through their columns only.
        // ! At each call to the closure, `col` will increment. So we're never creating two iterators going through
        // ! the same column.
        // ! We also didn't drop the matrix, which means we won't have any double frees.
        std::array::from_fn(|col| unsafe { SingleColumnIteratorValue::new_by_ref(&mat, col) })
    }

    ///
    /// Returns an array of iterator, over each of the columns of the matrix.
    ///
    /// The iterator yields all references in each column of the matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::ConstMatrix;
    ///
    /// let matrix = ConstMatrix::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// ]);
    /// let mut iter_columns = matrix.iter_all_col();
    /// assert_eq!(iter_columns[0].next(), Some(&1));
    /// assert_eq!(iter_columns[1].next(), Some(&2));
    /// assert_eq!(iter_columns[2].next(), Some(&3));
    /// assert_eq!(iter_columns[2].next(), Some(&6));
    /// assert_eq!(iter_columns[1].next(), Some(&5));
    /// assert_eq!(iter_columns[0].next(), Some(&4));
    /// ```
    ///
    pub fn iter_all_col(
        &self,
    ) -> [SingleColumnIterator<'_, K, ROW_NUMBER, COL_NUMBER>; COL_NUMBER] {
        std::array::from_fn(|col| self.iter_col(col))
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    ///
    /// Returns an array containing all the elements of the `index`th column of
    /// `self`.
    ///
    /// # Panics
    /// If `index` >= `COL_NUMBER`.
    ///
    pub fn column(self, index: usize) -> [K; ROW_NUMBER] {
        assert!(index < COL_NUMBER, "column out of range");
        let mut line_iterator = self.content.into_iter();

        std::array::from_fn(|_| {
            // We take each line one by one
            let line = line_iterator.next().unwrap_or_else(|| unreachable!());
            // We already checked that the column number is valid
            line.into_iter()
                .nth(index)
                .unwrap_or_else(|| unreachable!())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn iter_all_col_value() {
        let mat = ConstMatrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let [mut iter1, mut iter2, mut iter3] = mat.iter_all_col_value();

        assert_eq!(iter1.next(), Some(1));
        assert_eq!(iter1.next(), Some(4));
        assert_eq!(iter1.next(), Some(7));
        assert_eq!(iter1.next(), None);

        assert_eq!(iter2.next(), Some(2));
        assert_eq!(iter2.next(), Some(5));
        assert_eq!(iter2.next(), Some(8));
        assert_eq!(iter2.next(), None);

        assert_eq!(iter3.next(), Some(3));
        assert_eq!(iter3.next(), Some(6));
        assert_eq!(iter3.next(), Some(9));
        assert_eq!(iter3.next(), None);
    }

    #[test]
    fn iter_all_col_value_leaks() {
        let mat = ConstMatrix::from([
            [Box::new(1), Box::new(2), Box::new(3)],
            [Box::new(4), Box::new(5), Box::new(6)],
            [Box::new(7), Box::new(8), Box::new(9)],
        ]);
        let [_iter1, _iter2, _iter3] = mat.iter_all_col_value();
    }
}
