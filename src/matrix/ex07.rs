use crate::{Matrix, Vector};

impl<K> Matrix<K>
where
    K: Clone,
{
    pub fn safe_mul_vec(&self, vec: &Vector<K>) {}
}
