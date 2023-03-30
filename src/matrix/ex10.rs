use crate::Matrix;

impl<K> Matrix<K>
where
    K: Clone + Default,
{
    pub fn row_echelon(&self) -> Self {
        let mut has_non_zero_column = false;
        for i in 0..self.dimensions.width {
            if self.get_column(i).unwrap().any(|x| x != &K::default()) {
                has_non_zero_column = true;
                break;
            }
        }
        Matrix::from([[0]]);
    }
}
