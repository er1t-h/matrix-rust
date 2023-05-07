use std::ops::{Div, DivAssign, Mul, MulAssign, SubAssign};

use crate::{
    error::InverseError,
    traits::{IsZero, One},
    Matrix,
};

impl<K> Matrix<K>
where
    for<'a> K: Clone + One + Default + MulAssign<&'a K> + SubAssign<&'a K> + DivAssign<&'a K>,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K> + IsZero,
{
    ///
    /// Returns the inverse of a matrix.
    ///
    /// # Panics
    /// Never.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    /// assert_eq!(u.inverse().unwrap(), [[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
    /// ```
    ///
    /// # Errors
    /// If the matrix is not a square, returns a [`NotSquareMatrix`](InverseError::NotSquareMatrix)
    ///
    pub fn inverse(&self) -> Result<Self, InverseError> {
        if !self.is_square() {
            return Err(InverseError::NotSquareMatrix);
        }
        let mul_identity = K::one();
        let mut return_matrix = Self::augmented_matrix(
            self,
            &Self::identity(&mul_identity, self.dimensions.height).unwrap(),
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
    use crate::{complex::cpl, error::InverseError, Matrix};
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

    #[test]
    #[ignore = "rounding problem, and ordering for two complex numbers is not defined"]
    fn with_complex() {
        let u = Matrix::from([
            [cpl!(5., 2.), cpl!(3., -5.)],
            [cpl!(-89., 21.), cpl!(15., -4.)],
        ]);
        let res = u.inverse().unwrap();
        assert_eq!(
            res,
            Matrix::from([
                [
                    cpl!(
                        8990977669214025. / 488703345768541900.,
                        5148354073866157. / 244351672884270940.
                    ),
                    cpl!(
                        -4613384500822007. / 440637585861613000.,
                        -2736400605207637. / 3133422832793692700.
                    )
                ],
                [
                    cpl!(
                        300867642724715. / 2872515237914987.,
                        2922751545492027. / 22980121903319896.
                    ),
                    cpl!(
                        181659951142581. / 244351672884270940.,
                        2363959189541009. / 244351672884270940.
                    )
                ]
            ])
        )
    }
}
