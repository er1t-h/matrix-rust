use std::{
    mem::MaybeUninit,
    ops::{AddAssign, Mul, MulAssign},
};

use crate::{const_matrix::ConstMatrix, const_vector::ConstVector};

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize>
    MulAssign<ConstMatrix<K, ROW_NUMBER, COL_NUMBER>> for ConstVector<K, COL_NUMBER>
where
    K: Clone + Mul<K, Output = K> + AddAssign,
{
    fn mul_assign(&mut self, rhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER>) {
        for (column_index, vector_number) in self.content.iter_mut().enumerate() {
            let mut column_iterator = rhs.iter_col(column_index);
            let vector_initial_elt = vector_number.clone();
            *vector_number = vector_initial_elt.clone() * column_iterator.next().unwrap().clone();
            for matrix_number in column_iterator {
                *vector_number += matrix_number.clone() * vector_initial_elt.clone();
            }
        }
    }
}
