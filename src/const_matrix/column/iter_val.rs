use std::{
    iter::FusedIterator,
    mem::{self, MaybeUninit},
    ops::Range,
};

use crate::const_matrix::ConstMatrix;

pub struct SingleColumnIteratorValue<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    matrix: ConstMatrix<MaybeUninit<K>, ROW_NUMBER, COL_NUMBER>,
    column: usize,
    indexes: Range<usize>,
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>
{
    ///
    /// Creates a new iterator that will yield all values of the column, exactly
    /// one time, and drops all values not present in this column.
    ///
    pub fn new(matrix: ConstMatrix<K, ROW_NUMBER, COL_NUMBER>, column: usize) -> Self {
        assert!(
            column < COL_NUMBER,
            "expected column to be in the range 0..{COL_NUMBER}"
        );

        // We copy our matrix of K into a matrix of MaybeUninit<K> because we
        // will free all unused values of the matrix.
        let mut new_matrix: ConstMatrix<MaybeUninit<K>, ROW_NUMBER, COL_NUMBER> =
            unsafe { mem::transmute_copy(&matrix) };

        // We ask the compiler to not run the original matrix destructor,
        // because we will use its content
        std::mem::forget(matrix);

        // We drop all the values of the matrix other than the column we want
        // to iterate on. That way, we free all the values we won't use.
        for x in new_matrix.content.iter_mut() {
            for y in x
                .iter_mut()
                .enumerate()
                .filter_map(|(x, elt)| if x == column { None } else { Some(elt) })
            {
                unsafe {
                    y.assume_init_drop();
                }
            }
        }

        Self {
            matrix: new_matrix,
            column,
            indexes: 0..ROW_NUMBER,
        }
    }

    ///
    /// Creates a new iterator that will yield all values of the column, exactly
    /// one time.
    ///
    /// # Safety
    /// - The `matrix` must not be freed after a call to this function.
    /// - This function must never be called twice on the same matrix and column.
    ///
    pub unsafe fn new_by_ref(
        matrix: &ConstMatrix<K, ROW_NUMBER, COL_NUMBER>,
        column: usize,
    ) -> Self {
        assert!(
            column < COL_NUMBER,
            "expected column to be in the range 0..{COL_NUMBER}"
        );

        // We copy our matrix of K into a matrix of MaybeUninit<K> because we
        // will free all unused values of the matrix.
        let new_matrix: ConstMatrix<MaybeUninit<K>, ROW_NUMBER, COL_NUMBER> =
            unsafe { mem::transmute_copy(matrix) };

        Self {
            matrix: new_matrix,
            column,
            indexes: 0..ROW_NUMBER,
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Iterator
    for SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>
{
    // !
    // ! About unsafe:
    // !
    // ! Since we get the value depending on self.indexes.start, and that each
    // ! time we go through next we increase the value of self.indexes.start, we
    // ! can't return twice the same element.
    // !
    type Item = K;
    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            None
        } else {
            let tmp = unsafe { self.matrix.get_value(self.column, self.indexes.start) };
            self.indexes.start = self.indexes.start.saturating_add(1);
            tmp
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indexes.len(), Some(self.indexes.len()))
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> DoubleEndedIterator
    for SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>
{
    // !
    // ! About unsafe:
    // !
    // ! Since we get the value depending on self.indexes.end, and that each
    // ! time we go through next_back we decrease the value of self.indexes.end,
    // ! we can't return twice the same element.
    // !
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            None
        } else {
            self.indexes.end = self.indexes.end.saturating_sub(1);
            unsafe { self.matrix.get_value(self.column, self.indexes.end) }
        }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ExactSizeIterator
    for SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>
{
    fn len(&self) -> usize {
        self.indexes.len()
    }
}
impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> FusedIterator
    for SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>
{
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Drop
    for SingleColumnIteratorValue<K, ROW_NUMBER, COL_NUMBER>
{
    fn drop(&mut self) {
        // We go through all remaining values in order to free them.
        for _ in self {}
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::{column::iter_val::SingleColumnIteratorValue, ConstMatrix};

    #[test]
    #[should_panic(expected = "expected column to be in the range 0..2")]
    fn new() {
        let matrix = ConstMatrix::from([[Box::new(5), Box::new(3)], [Box::new(1), Box::new(6)]]);
        let _iter = SingleColumnIteratorValue::new(matrix, 2);
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn iterator() {
        let matrix = ConstMatrix::from([
            [Box::new(1)],
            [Box::new(2)],
            [Box::new(3)],
            [Box::new(4)],
            [Box::new(5)],
            [Box::new(6)],
            [Box::new(7)],
            [Box::new(8)],
            [Box::new(9)],
        ]);
        let mut iter = SingleColumnIteratorValue::new(matrix, 0);
        assert_eq!(iter.next(), Some(Box::new(1)));
        assert_eq!(iter.next(), Some(Box::new(2)));
        assert_eq!(iter.nth(2), Some(Box::new(5)));
        assert_eq!(iter.nth(0), Some(Box::new(6)));
        assert_eq!(iter.nth(3), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn double_ended_iterator() {
        let matrix = ConstMatrix::from([
            [Box::new(1)],
            [Box::new(2)],
            [Box::new(3)],
            [Box::new(4)],
            [Box::new(5)],
            [Box::new(6)],
            [Box::new(7)],
            [Box::new(8)],
            [Box::new(9)],
        ]);
        let mut iter = SingleColumnIteratorValue::new(matrix, 0);
        assert_eq!(iter.next_back(), Some(Box::new(9)));
        assert_eq!(iter.next_back(), Some(Box::new(8)));
        assert_eq!(iter.nth_back(3), Some(Box::new(4)));
        assert_eq!(iter.nth_back(0), Some(Box::new(3)));
        assert_eq!(iter.nth_back(3), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn size_hint() {
        let matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let iter = SingleColumnIteratorValue::new(matrix, 0);
        assert_eq!(iter.size_hint(), (9, Some(9)));
    }
}
