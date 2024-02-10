use std::mem::MaybeUninit;

mod column;
mod constructor;
mod operations;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConstMatrix<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    content: [[K; COL_NUMBER]; ROW_NUMBER],
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

pub type SquareMat<T, const SIZE: usize> = ConstMatrix<T, SIZE, SIZE>;
pub type Mat2<T = f64> = SquareMat<T, 2>;
pub type Mat3<T = f64> = SquareMat<T, 3>;
pub type Mat4<T = f64> = SquareMat<T, 4>;
