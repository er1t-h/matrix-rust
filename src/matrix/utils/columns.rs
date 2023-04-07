///
/// An iterator that go through a single column of a [Matrix](crate::Matrix).
///
/// # Notes:
///
/// See [MatrixIterByColumn](crate::matrix::utils::iterator::MatrixIterByColumn)
/// to go through all the columns of a [Matrix](crate::Matrix)
///
pub struct MatrixColumn<'a, K: Clone> {
    matrix: &'a [K],
    line_length: usize,
    stop: bool,
}

impl<'a, K: Clone> MatrixColumn<'a, K> {
    pub(super) fn new(matrix: &'a [K], column_nb: usize, line_length: usize) -> Self {
        let end_index = matrix.len() - (line_length - column_nb);
        Self {
            matrix: &matrix[column_nb..end_index + 1],
            line_length,
            stop: false,
        }
    }
}

impl<'a, K: Clone> Iterator for MatrixColumn<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop || self.matrix.is_empty() {
            None
        } else if self.matrix.len() < self.line_length {
            self.stop = true;
            self.matrix.get(0)
        } else {
            let tmp = self.matrix.get(0);
            self.matrix = &self.matrix[self.line_length..];
            tmp
        }
    }
}

impl<'a, K: Clone> DoubleEndedIterator for MatrixColumn<'a, K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.stop || self.matrix.is_empty() {
            None
        } else if self.matrix.len() < self.line_length {
            self.stop = true;
            self.matrix.last()
        } else {
            let tmp = self.matrix.last();
            self.matrix = &self.matrix[..self.matrix.len() - self.line_length];
            tmp
        }
    }
}

///
/// An iterator that go through a single column of a [Matrix](crate::Matrix),
/// yielding mutable references.
///
/// # Notes:
///
/// See [MatrixIterByColumnMut](crate::matrix::utils::iterator::MatrixIterByColumnMut)
/// to go through all the columns of a [Matrix](crate::Matrix).
///
pub struct MatrixColumnMut<'a, K: Clone> {
    matrix: &'a mut [K],
    line_length: usize,
    stop: bool,
}

impl<'a, K: Clone> MatrixColumnMut<'a, K> {
    pub(super) fn new(matrix: &'a mut [K], column_nb: usize, line_length: usize) -> Self {
        let end_index = matrix.len() - (line_length - column_nb);
        Self {
            matrix: &mut matrix[column_nb..end_index + 1],
            line_length,
            stop: false,
        }
    }
}

impl<'a, K: Clone> Iterator for MatrixColumnMut<'a, K> {
    // !
    // ! About unsafe:
    // ! I'm using unsafe because the borrow checker can't check that
    // ! I can return only once each item (since I'm advancing each time)
    // ! Since each item is part of matrix, we know that they have the same
    // ! lifetime.
    // ! And when I change self.matrix, I'm always advancing.
    // !
    type Item = &'a mut K;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stop || self.matrix.is_empty() {
            None
        } else if self.matrix.len() < self.line_length {
            self.stop = true;
            let tmp: *mut K = self.matrix.get_mut(0).unwrap();
            Some(unsafe { &mut *tmp })
        } else {
            let tmp_value: *mut K = self.matrix.get_mut(0).unwrap();
            let tmp_matrix: *mut [K] = &mut self.matrix[self.line_length..];
            self.matrix = unsafe { &mut *tmp_matrix };
            Some(unsafe { &mut *tmp_value })
        }
    }
}

impl<'a, K: Clone> DoubleEndedIterator for MatrixColumnMut<'a, K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.stop || self.matrix.is_empty() {
            None
        } else if self.matrix.len() < self.line_length {
            self.stop = true;
            let tmp: *mut K = self.matrix.get_mut(self.matrix.len() - 1).unwrap();
            Some(unsafe { &mut *tmp })
        } else {
            let matrix_len = self.matrix.len();
            let tmp_value: *mut K = self.matrix.get_mut(self.matrix.len() - 1).unwrap();
            let tmp_matrix: *mut [K] = &mut self.matrix[..matrix_len - self.line_length];
            self.matrix = unsafe { &mut *tmp_matrix };
            Some(unsafe { &mut *tmp_value })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;

    #[test]
    fn iter_rev() {
        let mat = Matrix::from([[1, 0], [2, 0], [3, 0]]);
        let mut iter = mat.get_column(0).unwrap().rev();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut mat = Matrix::from([[1, 0], [2, 0], [3, 0]]);
        {
            let mut iter_first_col = mat.get_column_mut(0).unwrap();
            *iter_first_col.next().unwrap() = 3;
            *iter_first_col.next().unwrap() = 4;
            *iter_first_col.next().unwrap() = 5;
            let mut iter_first_col = mat.get_column(0).unwrap();
            assert_eq!(iter_first_col.next(), Some(&3));
            assert_eq!(iter_first_col.next(), Some(&4));
            assert_eq!(iter_first_col.next(), Some(&5));
            assert_eq!(iter_first_col.next(), None);
        }
        {
            let mut iter_first_col = mat.get_column_mut(1).unwrap();
            *iter_first_col.next().unwrap() = 9;
            *iter_first_col.next().unwrap() = 9;
            *iter_first_col.next().unwrap() = 6;
            let mut iter_second_col = mat.get_column(1).unwrap();
            assert_eq!(iter_second_col.next(), Some(&9));
            assert_eq!(iter_second_col.next(), Some(&9));
            assert_eq!(iter_second_col.next(), Some(&6));
            assert_eq!(iter_second_col.next(), None);
        }
    }
}
