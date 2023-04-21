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
    use pretty_assertions::assert_eq;

    use crate::{complex::cpl, Matrix};

    #[test]
    fn basic() {
        let mat = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        let res = mat.transpose();
        assert_eq!(res, [[1, 4], [2, 5], [3, 6]]);
        println!("Transpose of {} is {}", mat, res);
        let mat = Matrix::from([[1]]);
        let res = mat.transpose();
        assert_eq!(res, [[1]]);
        println!("Transpose of {} is {}", mat, res);
        let mat = Matrix::from([[1], [2]]);
        let res = mat.transpose();
        assert_eq!(res, [[1, 2]]);
        println!("Transpose of {} is {}", mat, res);
    }

    #[test]
    fn with_complex() {
        {
            let mat = Matrix::from([[cpl!(5, -3), cpl!(6, 2)], [cpl!(0, -3), cpl!(-7, 2)]]);
            let res = mat.transpose();
            assert_eq!(res, [[cpl!(5, -3), cpl!(0, -3)], [cpl!(6, 2), cpl!(-7, 2)]]);
            println!("Transpose of {} is {}", mat, res);
        }
        {
            let mat = Matrix::from([[cpl!(65, -12), cpl!(64, 21)], [cpl!(0, -32), cpl!(-71, 20)]]);
            let res = mat.transpose();
            assert_eq!(
                res,
                [[cpl!(65, -12), cpl!(0, -32)], [cpl!(64, 21), cpl!(-71, 20)]]
            );
            println!("Transpose of {} is {}", mat, res);
        }
    }
}
