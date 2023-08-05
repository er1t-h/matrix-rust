use std::mem::MaybeUninit;

use crate::static_asserts::{AssertNonZero, AssertNonZeroSizeType};

mod column;
mod operations;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstMatrix<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    content: [[K; COL_NUMBER]; ROW_NUMBER],
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> From<[[K; COL_NUMBER]; ROW_NUMBER]>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    #[allow(clippy::no_effect, path_statements)]
    fn from(matrix: [[K; COL_NUMBER]; ROW_NUMBER]) -> Self {
        AssertNonZero::<COL_NUMBER>::OK;
        AssertNonZero::<ROW_NUMBER>::OK;
        AssertNonZeroSizeType::<K>::OK;

        Self { content: matrix }
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    pub fn get(&self, col: usize, row: usize) -> Option<&K> {
        self.content.get(row).and_then(|x| x.get(col))
    }

    pub fn get_mut(&mut self, col: usize, row: usize) -> Option<&mut K> {
        self.content.get_mut(row).and_then(|x| x.get_mut(col))
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    ConstMatrix<MaybeUninit<K>, ROW_NUMBER, COL_NUMBER>
{
    ///
    /// Returns the value at the position `col`;`row`.
    ///
    /// # Safety
    ///
    /// If this function is called twice on the same element, it results in
    /// Undefined Behaviour.
    ///
    unsafe fn get_value(&self, col: usize, row: usize) -> Option<K> {
        self.content
            .get(row)
            .and_then(|x| x.get(col))
            .map(|x| unsafe { x.assume_init_read() })
    }
}
