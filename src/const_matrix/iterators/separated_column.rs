use std::ops::Range;

use crate::const_matrix::ConstMatrix;

pub enum SeparatedColumnResult<K> {
    Content(K),
    EndOfColumn
}

pub struct ConstMatrixSeparatedColumnIterator<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    content: ConstMatrix<K, ROW_NUMBER, COL_NUMBER>,
    columns: Range<usize>,
    lines: Range<usize>,
}

impl <K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrixSeparatedColumnIterator<K, ROW_NUMBER, COL_NUMBER> {
    pub(in crate::const_matrix) fn new(matrix: ConstMatrix<K, ROW_NUMBER, COL_NUMBER>) -> Self {
        Self { content: matrix, columns: 0..COL_NUMBER, lines: 0..ROW_NUMBER }
    }
}

impl <K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Iterator for ConstMatrixSeparatedColumnIterator<K, ROW_NUMBER, COL_NUMBER> {
    type Item = SeparatedColumnResult<K>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.is_empty() {
            self.lines = 0..ROW_NUMBER;
            self.columns.start += 1;
            if self.columns.is_empty() {
                None
            } else {
                Some(SeparatedColumnResult::EndOfColumn)
            }
        } else {
            let tmp = self.content[self.lines.start][self.columns.start];
            self.lines.start += 1;
            tmp
        }
    }
}
