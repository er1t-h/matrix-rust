use std::ops::Range;

use super::ConstMatrix;

pub struct ConstMatrixSingleColumnIterator<'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    content: &'a ConstMatrix<K, ROW_NUMBER, COL_NUMBER>,
    column: usize,
    range: Range<usize>
}

impl <'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    ConstMatrixSingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    pub(in crate::const_matrix) fn new(matrix: &'a ConstMatrix<K, ROW_NUMBER, COL_NUMBER>, column_index: usize) -> Self {
        Self { content: matrix, column: column_index, range: 0..ROW_NUMBER }
    }
}

impl <'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Iterator
    for ConstMatrixSingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.is_empty() {
            None
        } else {
            let tmp = &self.content[self.range.start][self.column];
            self.range.start += 1;
            Some(tmp)
        }
    }
}

impl <'a, K, const ROW_NUMBER: usize, const COL_NUMBER: usize> DoubleEndedIterator
    for ConstMatrixSingleColumnIterator<'a, K, ROW_NUMBER, COL_NUMBER>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.range.is_empty() {
            None
        } else {
            self.range.end -= 1;
            let tmp = &self.content[self.range.end][self.column];
            Some(tmp)
        }
    }
}
