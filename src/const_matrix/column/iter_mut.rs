// !
// ! About the use of unsafe (in all iterator functions):
// !
// ! We need to be sure that we return each item exactly once.
// ! The borrow checker can't really assert this. However, on every call
// ! returning a mutable reference, we modified self.indexes, so that we
// ! always take another item. That way, we never yield twice the same item.
// !

use std::{iter::FusedIterator, ops::Range};

use crate::const_matrix::ConstMatrix;

///
/// Iterates on a single column of a Matrix, yielding mutable reference.
///
pub struct SingleColumnIteratorMut<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    matrix: &'a mut ConstMatrix<K, ROW_NUMBER, COL_NUMBER>,
    column: usize,
    indexes: Range<usize>,
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    SingleColumnIteratorMut<'a, K, ROW_NUMBER, COL_NUMBER>
{
    pub(super) fn new(
        matrix: &'a mut ConstMatrix<K, ROW_NUMBER, COL_NUMBER>,
        column: usize,
    ) -> Self {
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
    for SingleColumnIteratorMut<'a, K, ROW_NUMBER, COL_NUMBER>
{
    type Item = &'a mut K;
    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            None
        } else {
            let tmp = self.matrix.get_mut(self.column, self.indexes.start);
            self.indexes.start = self.indexes.start.saturating_add(1);
            tmp.map(|x| unsafe { &mut *(x as *mut K) })
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
            let tmp = self.matrix.get_mut(self.column, new_index);
            tmp.map(|x| unsafe { &mut *(x as *mut K) })
        }
    }
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> DoubleEndedIterator
    for SingleColumnIteratorMut<'a, K, ROW_NUMBER, COL_NUMBER>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            None
        } else {
            self.indexes.end = self.indexes.end.saturating_sub(1);
            self.matrix
                .get_mut(self.column, self.indexes.end)
                .map(|x| unsafe { &mut *(x as *mut K) })
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.indexes.end = self.indexes.end.saturating_sub(n.saturating_add(1));
        if n > self.indexes.len() {
            None
        } else {
            self.matrix
                .get_mut(self.column, self.indexes.end)
                .map(|x| unsafe { &mut *(x as *mut K) })
        }
    }
}

impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ExactSizeIterator
    for SingleColumnIteratorMut<'a, K, ROW_NUMBER, COL_NUMBER>
{
    fn len(&self) -> usize {
        self.indexes.len()
    }
}
impl<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> FusedIterator
    for SingleColumnIteratorMut<'a, K, ROW_NUMBER, COL_NUMBER>
{
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use crate::const_matrix::{column::SingleColumnIteratorMut, ConstMatrix};

    #[test]
    #[should_panic(expected = "expected column to be in the range 0..2")]
    fn new() {
        let mut matrix = ConstMatrix::from([[5, 3], [1, 6]]);
        let _ = SingleColumnIteratorMut::new(&mut matrix, 2);
    }

    #[test]
    fn iterator() {
        let mut matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let mut iter = SingleColumnIteratorMut::new(&mut matrix, 0);
        *iter.next().unwrap() = 9;
        *iter.next().unwrap() = 8;
        *iter.next().unwrap() = 7;
        *iter.nth(2).unwrap() = 4;
        assert_eq!(
            matrix,
            ConstMatrix::from([[9], [8], [7], [4], [5], [4], [7], [8], [9]])
        );
    }

    #[test]
    fn double_ended_iterator() {
        let mut matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let mut iter = SingleColumnIteratorMut::new(&mut matrix, 0);
        *iter.next_back().unwrap() = 1;
        *iter.next_back().unwrap() = 2;
        *iter.next_back().unwrap() = 3;
        *iter.nth_back(2).unwrap() = 6;
        assert_eq!(
            matrix,
            ConstMatrix::from([[1], [2], [3], [6], [5], [6], [3], [2], [1]])
        );
    }

    #[test]
    fn size_hint() {
        let mut matrix = ConstMatrix::from([[1], [2], [3], [4], [5], [6], [7], [8], [9]]);
        let iter = SingleColumnIteratorMut::new(&mut matrix, 0);
        assert_eq!(iter.size_hint(), (9, Some(9)));
    }
}
