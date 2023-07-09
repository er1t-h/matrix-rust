use std::mem::{self, MaybeUninit};

use super::ConstMatrix;

impl<K: Sized, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    ///
    /// Returns an array containing all the elements of the `index`th column of
    /// `self`.
    ///
    /// # Panics
    /// If `index` >= `COL_NUMBER`.
    ///
    pub fn column(self, index: usize) -> [K; ROW_NUMBER] {
        assert!(index < COL_NUMBER, "column out of range");
        // See `initializing an array element-by-element` in the doc:
        // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut array: [MaybeUninit<K>; ROW_NUMBER] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (emplace, row) in array.iter_mut().zip(self.content) {
            emplace.write(row.into_iter().nth(index).unwrap());
        }

        unsafe { mem::transmute_copy::<_, [K; ROW_NUMBER]>(&array) }
    }
}
