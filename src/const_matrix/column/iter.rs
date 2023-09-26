use std::{iter::FusedIterator, ops::Range};

use crate::const_matrix::ConstMatrix;

///
/// Iterates on a single column of a Matrix.
///
pub struct SingleColumnIterator<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    matrix: &'a ConstMatrix<K, ROW_NUMBER, COL_NUMBER>,
    column: usize,
    indexes: Range<usize>,
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    SingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    pub(super) fn new(matrix: &'a ConstMatrix<K, ROW_NUMBER, COL_NUMBER>, column: usize) -> Self {
        assert!(
            column < COL_NUMBER,
            "expected column to be in the range 0..{COL_NUMBER}"
        );
        Self {
            matrix,
            column,
            indexes: 0..ROW_NUMBER,
        }
    }
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Iterator
    for SingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            None
        } else {
            let tmp = self.matrix.get(self.column, self.indexes.start);
            self.indexes.start = self.indexes.start.saturating_add(1);
            tmp
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indexes.len(), Some(self.indexes.len()))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n > self.indexes.len() {
            self.indexes.start = self.indexes.start.saturating_add(n);
            None
        } else {
            let new_index = self.indexes.start.saturating_add(n);
            self.indexes.start = new_index.saturating_add(1);
            self.matrix.get(self.column, new_index)
        }
    }
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> DoubleEndedIterator
    for SingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            None
        } else {
            self.indexes.end = self.indexes.end.saturating_sub(1);
            self.matrix.get(self.column, self.indexes.end)
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.indexes.end = self.indexes.end.saturating_sub(n.saturating_add(1));
        if n > self.indexes.len() {
            None
        } else {
            self.matrix.get(self.column, self.indexes.end)
        }
    }
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ExactSizeIterator
    for SingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    fn len(&self) -> usize {
        self.indexes.len()
    }
}
impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> FusedIterator
    for SingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
}

#[cfg(test)]
mod test {

    use crate::const_matrix::{column::SingleColumnIterator, ConstMatrix};

    #[test]
    #[should_panic(expected = "expected column to be in the range 0..2")]
    fn new() {
        let matrix = ConstMatrix::from([[5, 3], [1, 6]]);
        let _ = SingleColumnIterator::new(&matrix, 2);
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn iterator() {
        let matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let mut iter = SingleColumnIterator::new(&matrix, 0);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.nth(3), Some(&6));
        assert_eq!(iter.nth(0), Some(&7));
        assert_eq!(iter.nth(3), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn double_ended_iterator() {
        let matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let mut iter = SingleColumnIterator::new(&matrix, 0);
        assert_eq!(iter.next_back(), Some(&9));
        assert_eq!(iter.next_back(), Some(&8));
        assert_eq!(iter.nth_back(3), Some(&4));
        assert_eq!(iter.nth_back(0), Some(&3));
        assert_eq!(iter.nth_back(3), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn size_hint() {
        let matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let iter = SingleColumnIterator::new(&matrix, 0);
        assert_eq!(iter.size_hint(), (9, Some(9)));
    }
}
