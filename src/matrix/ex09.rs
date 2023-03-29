use crate::Matrix;

use super::Dimensions;

impl<K> Matrix<K>
where
    K: Clone,
{
    pub fn transpose(&self) -> Self {
        Self {
            content: self.columns().cloned().collect(),
            dimensions: Dimensions {
                width: self.dimensions.height,
                height: self.dimensions.width,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;

    #[test]
    fn basic() {
        let mat = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        let res = mat.transpose();
    }
}
