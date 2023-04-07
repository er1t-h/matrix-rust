use std::ops::{Div, DivAssign, Mul, MulAssign, SubAssign};

use crate::{
    error::InverseError,
    traits::{IsZero, MulIdentity},
    Matrix,
};

impl<K> Matrix<K>
where
    K: Clone + MulIdentity + Default,
    for<'a> K: MulAssign<&'a K> + SubAssign<&'a K> + DivAssign<&'a K>,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K> + IsZero,
{
    ///
    /// Returns the inverse of a matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    /// assert_eq!(u.inverse().unwrap(), [[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
    /// ```
    ///
    pub fn inverse(&self) -> Result<Self, InverseError> {
        if !self.is_square() {
            return Err(InverseError::NotSquareMatrix);
        }
        let mul_identity = K::mul_identity();
        let mut return_matrix = Matrix::augmented_matrix(
            self,
            &Matrix::identity(&mul_identity, self.dimensions.height).unwrap(),
        )
        .unwrap();
        return_matrix = return_matrix.reduced_row_echelon();
        for i in 0..self.dimensions.height {
            if return_matrix.get(i, i).unwrap() != &mul_identity {
                return Err(InverseError::SingularMatrix);
            }
        }
        Ok(return_matrix
            .submatrix(
                self.dimensions.width..return_matrix.dimensions.width,
                0..self.dimensions.height,
            )
            .unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::{error::InverseError, Matrix};
    use pretty_assertions::assert_eq;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            let res: Matrix<f64> = u.inverse().unwrap();
            assert_eq!(res, [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            println!("{}", res);
            // [1.0, 0.0, 0.0]
            // [0.0, 1.0, 0.0]
            // [0.0, 0.0, 1.0]
        }
        {
            let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
            let res: Matrix<f64> = u.inverse().unwrap();
            assert_eq!(res, [[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
            println!("{}", res);
            // [0.5, 0.0, 0.0]
            // [0.0, 0.5, 0.0]
            // [0.0, 0.0, 0.5]
        }
        {
            let u: Matrix<f64> = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
            let res = u.inverse().unwrap();
            assert!(res.approx_eq(
                &[
                    [0.649425287, 0.097701149, -0.655172414],
                    [-0.781609195, -0.126436782, 0.965517241],
                    [0.143678161, 0.074712644, -0.206896552]
                ],
                &0.00001
            ));
            println!("{}", res);
            // [0.649425287, 0.097701149, -0.655172414]
            // [-0.781609195, -0.126436782, 0.965517241]
            // [0.143678161, 0.074712644, -0.206896552]
        }
    }

    #[test]
    fn errors() {
        {
            let mat = Matrix::from([[1_i32, 2], [3, 4], [5, 6]]);
            assert_eq!(mat.inverse(), Err(InverseError::NotSquareMatrix));
        }
        {
            let mat = Matrix::from([[1, 2], [2, 4]]);
            assert_eq!(mat.inverse(), Err(InverseError::SingularMatrix));
        }
    }
}
