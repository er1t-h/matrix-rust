//!
//! Implementation of `determinant` for a [`Matrix`]
//!
//! # Implementations details
//!
//! Determinant for a matrix of up to `4×4` does not allocate.
//! For bigger dimensions, the process passes by a reduced row echelon, taking
//! far more time and space.
//!

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    error::DeterminantError,
    traits::{IsZero, One},
    Matrix,
};

// Matrix:
// 0 1
// 2 3
#[inline(always)]
fn determinant_2<K>(content: &[&K; 4]) -> K
where
    K: Clone + Default,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K>,
{
    &(content[0] * content[3]) - &(content[1] * content[2])
}

// Matrix:
// 0 1 2
// 3 4 5
// 6 7 8
#[inline(always)]
fn determinant_3<K>(content: &[&K; 9]) -> K
where
    K: Clone + Default,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K> + Add<&'a K, Output = K>,
{
    &(&(content[0] * &determinant_2(&[content[4], content[5], content[7], content[8]]))
        - &(content[1] * &determinant_2(&[content[3], content[5], content[6], content[8]])))
        + &(content[2] * &determinant_2(&[content[3], content[4], content[6], content[7]]))
}

// Matrix:
//  0  1  2  3
//  4  5  6  7
//  8  9 10 11
// 12 13 14 15
#[inline(always)]
fn determinant_4<K>(content: &[&K; 16]) -> K
where
    K: Clone + Default,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K> + Add<&'a K, Output = K>,
{
    &(&(&(content[0]
        * &determinant_3(&[
            content[5],
            content[6],
            content[7],
            content[9],
            content[10],
            content[11],
            content[13],
            content[14],
            content[15],
        ]))
        - &(content[1]
            * &determinant_3(&[
                content[4],
                content[6],
                content[7],
                content[8],
                content[10],
                content[11],
                content[12],
                content[14],
                content[15],
            ])))
        + &(content[2]
            * &determinant_3(&[
                content[4],
                content[5],
                content[7],
                content[8],
                content[9],
                content[11],
                content[12],
                content[13],
                content[15],
            ])))
        - &(content[3]
            * &determinant_3(&[
                content[4],
                content[5],
                content[6],
                content[8],
                content[9],
                content[10],
                content[12],
                content[13],
                content[14],
            ]))
}

impl<'a, K> Matrix<K>
where
    K: Clone + Default + 'a + Neg<Output = K> + One,
    for<'b> &'b K: Sub<&'b K, Output = K> + Mul<&'b K, Output = K> + Add<&'b K, Output = K>,
    for<'b> K: Display
        + Clone
        + Default
        + MulAssign<&'b K>
        + SubAssign<&'b K>
        + AddAssign<&'b K>
        + DivAssign<&'b K>,
    for<'b> &'b K: PartialEq + Mul<&'b K, Output = K> + Div<&'b K, Output = K> + IsZero,
{
    // ! I know it looks terrible, but it's the only way to do it without cloning the matrix
    // ! Moreover, it's the way that takes the less time to compute
    ///
    /// Returns the determinant of a matrix.
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    /// assert_eq!(mat.determinant().unwrap(), 8.)
    /// ```
    ///
    /// # Errors
    /// If the matrix is not a square, returns a [`NotSquareMatrix`](DeterminantError::NotSquareMatrix)
    ///
    pub fn determinant(&self) -> Result<K, DeterminantError> {
        match self.size() {
            (1, 1) => Ok(self.content[0].clone()),
            (2, 2) => Ok(determinant_2(&[&self[0], &self[1], &self[2], &self[3]])),
            (3, 3) => Ok(determinant_3(&[
                &self[0], &self[1], &self[2], &self[3], &self[4], &self[5], &self[6], &self[7],
                &self[8],
            ])),
            (4, 4) => Ok(determinant_4(&[
                &self[0], &self[1], &self[2], &self[3], &self[4], &self[5], &self[6], &self[7],
                &self[8], &self[9], &self[10], &self[11], &self[12], &self[13], &self[14],
                &self[15],
            ])),
            (x, y) if x == y => {
                let (row_echelon, swaps, factor) = self.row_echelon_internal();
                let mut trace = row_echelon.multiplicative_trace_internal();
                trace *= &factor;
                if swaps % 2 == 0 {
                    Ok(trace)
                } else {
                    Ok(-trace)
                }
            }
            _ => Err(DeterminantError::NotSquareMatrix),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::assert_eq_float;
    use crate::complex::cpl;
    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., -1.], [-1., 1.]]);
            let res = u.determinant().unwrap();
            assert_eq_float!(res, 0.);
            println!("det({u}) = {res:?}");
            // 0.0
        }
        {
            let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
            let res = u.determinant().unwrap();
            assert_eq_float!(res, 8.);
            println!("det({u}) = {res:?}");
            // 8.0
        }
        {
            let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
            let res = u.determinant().unwrap();
            assert_eq_float!(res, -174.);
            println!("det({u}) = {res:?}");
            // -174.0
        }
        {
            let u = Matrix::from([
                [8., 5., -2., 4.],
                [4., 2.5, 20., 4.],
                [8., 5., 1., 4.],
                [28., -4., 17., 1.],
            ]);
            let res = u.determinant().unwrap();
            assert_eq_float!(res, 1032.);
            println!("det({u}) = {res:?}");
            // 1032
        }
    }

    #[test]
    fn matrix5() {
        let u = Matrix::from([
            [8., 5., -2., 4., 4.],
            [2.5, 20., 4., 8., 5.],
            [1., 4., 28., -4., 17.],
            [1., 4., 2., 0.5, 41.],
            [21., 8., 5., 10., 24.],
        ]);
        let res = u.determinant().unwrap();
        assert_eq_float!(res, -627_635.25);
        println!("det({u}) = {res:?}");
    }

    #[test]
    fn with_complex() {
        let u = Matrix::from([
            [cpl!(5., 2.), cpl!(3., 4.), cpl!(1., 0.)],
            [cpl!(4., 12.), cpl!(-4., 3.), cpl!(8., -5.)],
            [cpl!(0., 0.), cpl!(7., 3.), cpl!(-5., -7.)],
        ]);
        let res = u.determinant().unwrap();
        assert_eq!(res, cpl!(-750., 164.));
        println!("det({u}) = {res:?}");
    }
}
