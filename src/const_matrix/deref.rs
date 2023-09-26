use std::ops::{Deref, DerefMut};

use super::ConstMatrix;

impl <K, const ROW_NUMBER: usize, const COL_NUMBER: usize> Deref for ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    type Target = [[K; COL_NUMBER]; ROW_NUMBER];
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl <K, const ROW_NUMBER: usize, const COL_NUMBER: usize> DerefMut for ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
