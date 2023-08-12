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
            println!("{res}");
            // [1.0, 0.0, 0.0]
            // [0.0, 1.0, 0.0]
            // [0.0, 0.0, 1.0]
        }
        {
            let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
            let res: Matrix<f64> = u.inverse().unwrap();
            assert_eq!(res, [[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
            println!("{res}");
            // [0.5, 0.0, 0.0]
            // [0.0, 0.5, 0.0]
            // [0.0, 0.0, 0.5]
        }
        {
            let u: Matrix<f64> = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
            let res = u.inverse().unwrap();
            assert!(res.approx_eq(
                &[
                    [0.649_425_287, 0.097_701_149, -0.655_172_414],
                    [-0.781_609_195, -0.126_436_782, 0.965_517_241],
                    [0.143_678_161, 0.074_712_644, -0.206_896_552]
                ],
                &0.00001
            ));
            println!("{res}");
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
                        8_990_977_669_214_025. / 488_703_345_768_541_900.,
                        5_148_354_073_866_157. / 244_351_672_884_270_940.
                    ),
                    cpl!(
                        -4_613_384_500_822_007. / 440_637_585_861_613_000.,
                        -2_736_400_605_207_637. / 3_133_422_832_793_692_700.
                    )
                ],
                [
                    cpl!(
                        300_867_642_724_715. / 2_872_515_237_914_987.,
                        2_922_751_545_492_027. / 22_980_121_903_319_896.
                    ),
                    cpl!(
                        181_659_951_142_581. / 244_351_672_884_270_940.,
                        2_363_959_189_541_009. / 244_351_672_884_270_940.
                    )
                ]
            ])
        );
    }
}
