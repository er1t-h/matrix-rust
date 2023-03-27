use crate::Matrix;

///
/// An iterator that go through a [Matrix] column by column instead of line by
/// line.
///
/// # Notes:
///
/// See [MatrixColumn](crate::matrix::utils::columns::MatrixColumn) to go
/// through only one column.
///
pub struct MatrixIterByColumn<'a, K: Clone> {
    matrix: &'a Matrix<K>,
    current_line: usize,
    current_column: usize,
}

impl<'a, K: Clone> Iterator for MatrixIterByColumn<'a, K> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.matrix.get(self.current_line, self.current_column)?;
        if self.current_line == self.matrix.dimensions.height - 1 {
            self.current_line = 0;
            self.current_column += 1;
        } else {
            self.current_line += 1;
        }
        Some(tmp)
    }
}

impl<'a, K: Clone> MatrixIterByColumn<'a, K> {
    pub(super) fn new(matrix: &'a Matrix<K>) -> MatrixIterByColumn<'a, K> {
        Self {
            matrix,
            current_line: 0,
            current_column: 0,
        }
    }
}

///
/// An iterator that go through a [Matrix] column by column instead of line by
/// line, yielding mutable references.
///
/// # Notes:
///
/// See [MatrixColumnMut](crate::matrix::utils::columns::MatrixColumnMut) to go
/// through only one column.
///
pub struct MatrixIterByColumnMut<'a, K: Clone> {
    matrix: &'a mut Matrix<K>,
    current_line: usize,
    current_column: usize,
}

impl<'a, K: Clone> Iterator for MatrixIterByColumnMut<'a, K> {
    type Item = &'a mut K;
    // !
    // ! About unsafe:
    // ! Forced to use unsafe code because of borrow checker.
    // ! Due to the the incrementation at each call, we know we will never return
    // ! twice the same object, so the use to unsafe won't introduce any errors
    // !
    fn next(&mut self) -> Option<Self::Item> {
        let height = self.matrix.dimensions.height;
        let tmp = self
            .matrix
            .get_mut(self.current_line, self.current_column)?;
        if self.current_line == height - 1 {
            self.current_line = 0;
            self.current_column += 1;
        } else {
            self.current_line += 1;
        }
        let tmp_ptr: *mut K = tmp;
        unsafe { Some(&mut *tmp_ptr) }
    }
}

impl<'a, K: Clone> MatrixIterByColumnMut<'a, K> {
    pub(super) fn new(matrix: &'a mut Matrix<K>) -> MatrixIterByColumnMut<'a, K> {
        Self {
            matrix,
            current_line: 0,
            current_column: 0,
        }
    }
}
