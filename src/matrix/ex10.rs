use std::{
    fmt::Display,
    ops::{Div, DivAssign, Mul, MulAssign, SubAssign},
};

use crate::{traits::IsZero, Matrix};

use super::Dimensions;

impl<K> Matrix<K>
where
    for<'a> K: Display + Clone + Default + MulAssign<&'a K> + SubAssign<&'a K> + DivAssign<&'a K>,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K> + IsZero,
{
    pub fn row_echelon(&self) -> Self {
        // First, we skip all zero columns
        let mut first_non_zero_column = 0;
        let mut rows_set = 0;
        let mut return_matrix = self.clone();
        while first_non_zero_column < self.dimensions.width {
            let mut first_non_zero_line = 0;
            let mut end = true;
            for i in first_non_zero_column..self.dimensions.width {
                if return_matrix
                    .get_column(i)
                    .unwrap()
                    .skip(rows_set)
                    .any(|x| !x.is_zero())
                {
                    end = false;
                    first_non_zero_column = i;
                    first_non_zero_line = self
                        .get_column(i)
                        .unwrap()
                        .skip(rows_set)
                        .position(|x| !x.is_zero())
                        .unwrap()
                        + rows_set;
                    break;
                }
            }
            if end {
                return return_matrix;
            }
            return_matrix.swap_line(rows_set, first_non_zero_line);
            let first_copy = return_matrix
                .get(rows_set, first_non_zero_column)
                .unwrap()
                .clone();
            for elt in return_matrix
                .get_line_mut(rows_set)
                .unwrap()
                .skip(first_non_zero_column)
            {
                *elt /= &first_copy;
            }
            rows_set += 1;
            for non_treated_line in rows_set..return_matrix.dimensions.height {
                let coeff = {
                    let first_number_of_new_line = return_matrix
                        .get(non_treated_line, first_non_zero_column)
                        .unwrap();
                    if first_number_of_new_line.is_zero() {
                        continue;
                    }
                    let pivot = return_matrix.get(0, first_non_zero_column).unwrap();
                    first_number_of_new_line / pivot
                };
                for elt_index in first_non_zero_column..return_matrix.dimensions.width {
                    let tmp = &coeff * return_matrix.get(0, elt_index).unwrap();
                    *return_matrix.get_mut(non_treated_line, elt_index).unwrap() -= &tmp;
                }
            }
            first_non_zero_column += 1;
        }
        return_matrix
    }

    pub fn reduced_row_echelon(&self) -> Self {
        let mut return_matrix = self.row_echelon();
        println!("Test: {}", &return_matrix);
        let mut rows_set = 0;
        // For each line starting at the bottom
        for index_line in (1..return_matrix.dimensions.height).rev() {
            // Take the pivot
            let Some(pivot_position) = return_matrix.get_line(index_line).unwrap().position(|x| !x.is_zero()) else {
                rows_set += 1;
                continue;
            };
            // For each line above it
            for changing_index in 0..self.dimensions.height - rows_set - 1 {
                // For each number in that line
                for i in (pivot_position..return_matrix.dimensions.width).rev() {
                    let ratio = return_matrix.get(changing_index, pivot_position).unwrap();
                    let to_sub = ratio * &return_matrix.get(index_line, i).unwrap();
                    *return_matrix.get_mut(changing_index, i).unwrap() -= &to_sub;
                }
            }
            rows_set += 1;
        }
        return_matrix
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            let res = u.row_echelon();
            assert_eq!(res, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
            println!("Row echelon of {u} = {res}");
        }
        {
            let u = Matrix::from([[1., 2.], [3., 4.]]);
            let res = u.row_echelon();
            assert_eq!(res, [[1.0, 2.0], [0.0, 1.0]]);
            println!("Row echelon of {u} = {res}");
        }
        {
            let u = Matrix::from([[1., 2.], [2., 4.]]);
            let res = u.row_echelon();
            assert_eq!(res, [[1.0, 2.0], [0.0, 0.0]]);
            println!("Row echelon of {u} = {res}");
        }
        {
            let u = Matrix::from([
                [8., 5., -2., 4., 28.],
                [4., 2.5, 20., 4., -4.],
                [8., 5., 1., 4., 17.],
            ]);
            let res = u.reduced_row_echelon();
            assert_eq!(
                res,
                [
                    [1.0, 0.625, 0.0, 0.0, -12.1666667],
                    [0.0, 0.0, 1.0, 0.0, -3.6666667],
                    [0.0, 0.0, 0.0, 1.0, 29.5]
                ]
            );
            println!("Row echelon of {u} = {res}");
        }
    }
}
