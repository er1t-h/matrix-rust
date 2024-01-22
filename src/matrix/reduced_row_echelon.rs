//!
//! Implementation of a reduced row echelon form for a [`Matrix`]
//!

use std::ops::{Div, DivAssign, Mul, MulAssign, SubAssign};

use crate::{
    traits::{IsZero, One},
    Matrix,
};

impl<K> Matrix<K>
where
    for<'a> K: Clone + Default + MulAssign<&'a K> + SubAssign<&'a K> + DivAssign<&'a K> + One,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K> + IsZero,
{
    ///
    /// Returns the row echelon form of a Matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1., 2.], [3., 4.]]);
    /// assert_eq!(mat.row_echelon(), [[1., 2.], [0., 1.]]);
    /// ```
    ///
    pub(crate) fn row_echelon_internal(&self) -> (Self, usize, K) {
        // First, we skip all zero columns
        let mut first_non_zero_column = 0;
        let mut rows_set = 0;
        let mut swap_number = 0;
        let mut factor = K::one();
        let mut return_matrix = self.clone();
        while first_non_zero_column < self.dimensions.width {
            let mut first_non_zero_line = 0;
            let mut end = true;
            for i in first_non_zero_column..self.dimensions.width {
                //; `i` is bounded by self.dimensions.width, so `get_column(i)`
                //; will never return None
                if let Some(non_zero_line) = return_matrix
                    .get_column(i)
                    .unwrap_or_else(|| unreachable!())
                    .skip(rows_set)
                    .position(|x| !x.is_zero())
                {
                    end = false;
                    //; `i` is bounded by self.dimensions.width, so `get_column(i)`
                    //; will never return None
                    first_non_zero_line = non_zero_line + rows_set;
                    first_non_zero_column = i;
                    break;
                }
            }
            if end {
                return (return_matrix, 0, factor);
            }
            if rows_set != first_non_zero_line {
                return_matrix.swap_line(rows_set, first_non_zero_line);
                swap_number += 1;
            }
            //; `rows_set` represents the number of row already treated, so it can't
            //; more than the number of rows of the matrix itself.
            //;
            //; `first_non_zero_column` starts at 0, and is either incremented in loop end
            //; (but checked on the loop beginning), or set to `i`, which is bounded
            //; to the width of the matrix
            let first_copy = return_matrix
                .get(rows_set, first_non_zero_column)
                .unwrap_or_else(|| unreachable!())
                .clone();
            factor *= &first_copy;
            //; `rows_set` represents the number of row already treated, so it can't
            //; more than the number of rows of the matrix itself.
            for elt in return_matrix
                .get_line_mut(rows_set)
                .unwrap_or_else(|| unreachable!())
                .skip(first_non_zero_column)
            {
                *elt /= &first_copy;
            }
            //; `non_treated_line` is bound to `dimensions.height`
            //;
            //; `first_non_zero_column` starts at 0, and is either incremented in loop end
            //; (but checked on the loop beginning), or set to `i`, which is bounded
            //; to the width of the matrix
            for non_treated_line in rows_set + 1..return_matrix.dimensions.height {
                let coeff = {
                    let first_number_of_new_line = return_matrix
                        .get(non_treated_line, first_non_zero_column)
                        .unwrap_or_else(|| unreachable!());
                    if first_number_of_new_line.is_zero() {
                        continue;
                    }
                    let pivot = return_matrix.get(rows_set, first_non_zero_column).unwrap_or_else(|| unreachable!());
                    first_number_of_new_line / pivot
                };
                //; `elt_index` is bound to `return_matrix.dimensions.width`
                for elt_index in first_non_zero_column..return_matrix.dimensions.width {
                    let tmp = &coeff * return_matrix.get(rows_set, elt_index).unwrap_or_else(|| unreachable!());
                    *return_matrix.get_mut(non_treated_line, elt_index).unwrap_or_else(|| unreachable!()) -= &tmp;
                }
            }
            rows_set += 1;
            first_non_zero_column += 1;
        }
        (return_matrix, swap_number, factor)
    }

    pub fn row_echelon(&self) -> Self {
        self.row_echelon_internal().0
    }

    ///
    /// Returns the reduced row echelon form of a matrix
    ///
    /// # Panics
    /// Never.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1., 2.], [3., 4.]]);
    /// assert_eq!(mat.reduced_row_echelon(), [[1., 0.], [0., 1.]]);
    /// ```
    ///
    pub fn reduced_row_echelon(&self) -> Self {
        let mut return_matrix = self.row_echelon();
        //; `index_line` is bound to `return_matrix.dimensions.height`
        // For each line
        for index_line in 1..return_matrix.dimensions.height {
            //; `pivot_position` is bound to `return_matrix.dimensions.width`
            // * Take the pivot
            let Some(pivot_position) = return_matrix
                .get_line(index_line)
                .unwrap_or_else(|| unreachable!())
                .position(|x| !x.is_zero())
            else {
                continue;
            };
            //; `changing_index` is bound to `index_line`, which is itself bound
            //; to `return_matrix.dimensions.height`
            // For each line above it
            for changing_index in 0..index_line {
                // For each number in that line
                for i in (pivot_position..return_matrix.dimensions.width).rev() {
                    let ratio = return_matrix.get(changing_index, pivot_position).unwrap_or_else(|| unreachable!());
                    let to_sub = ratio * return_matrix.get(index_line, i).unwrap_or_else(|| unreachable!());
                    *return_matrix.get_mut(changing_index, i).unwrap_or_else(|| unreachable!()) -= &to_sub;
                }
            }
        }
        return_matrix
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u: Matrix<f64> = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            let res = u.reduced_row_echelon();
            assert_eq!(res, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
            println!("Row echelon of {u} = {res}");
        }
        {
            let u: Matrix<f64> = Matrix::from([[1., 2.], [3., 4.]]);
            let res = u.reduced_row_echelon();
            assert_eq!(res, [[1.0, 0.0], [0.0, 1.0]]);
            println!("Row echelon of {u} = {res}");
        }
        {
            let u: Matrix<f64> = Matrix::from([[1., 2.], [2., 4.]]);
            let res = u.reduced_row_echelon();
            assert_eq!(res, [[1.0, 2.0], [0.0, 0.0]]);
            println!("Row echelon of {u} = {res}");
        }
        {
            let u: Matrix<f64> = Matrix::from([
                [8., 5., -2., 4., 28.],
                [4., 2.5, 20., 4., -4.],
                [8., 5., 1., 4., 17.],
            ]);
            let res = u.reduced_row_echelon();
            assert!(res.approx_eq(
                &[
                    [1.0, 0.625, 0.0, 0.0, -12.166_666_7],
                    [0.0, 0.0, 1.0, 0.0, -3.666_666_7],
                    [0.0, 0.0, 0.0, 1.0, 29.5]
                ],
                &0.00001
            ));
            println!("Row echelon of {u} = {res}");
        }
    }

    #[test]
    #[ignore = "rounding problem, and ordering for two complex numbers is not defined"]
    fn with_complex() {
        // {
        //     let mat = Matrix::from([
        //         [cpl!(5., 2.), cpl!(2., 4.), cpl!(3., 5.)],
        //         [cpl!(5., 7.), cpl!(4., 2.), cpl!(1., 1.)],
        //     ]);
        //     let res = mat.reduced_row_echelon();
        //     assert_eq!(
        //         res,
        //         [
        //             [cpl!(1., 0.), cpl!(0., 0.), cpl!(-(46. / 353.), 186. / 353.)],
        //             [cpl!(0., 0.), cpl!(1., 0.), cpl!(703. / 706., 479. / 706.)]
        //         ]
        //     )
        // }

        let mat: Matrix<f64> = Matrix::from([[1., 3., 6.], [18., 9., 7.]]);
        let res = mat.reduced_row_echelon();
        println!("{res}");
    }
}
