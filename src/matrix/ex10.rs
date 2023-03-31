use std::ops::{Div, Mul, MulAssign, SubAssign};

use crate::{traits::IsZero, Matrix};

use super::Dimensions;

impl<K> Matrix<K>
where
    for<'a> K: Clone + Default + MulAssign<&'a K> + SubAssign<&'a K>,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K>,
    for<'a> &'a mut K: IsZero,
{
    pub fn row_echelon(&self) -> Self {
        // First, we skip all zero columns
        let mut first_non_zero_column = self.dimensions.width;
        let mut first_non_zero_line = 0;
        for i in 0..self.dimensions.width {
            if self.get_column(i).unwrap().any(|x| x != &K::default()) {
                first_non_zero_column = i;
                first_non_zero_line = self
                    .get_column(i)
                    .unwrap()
                    .position(|x| x != &K::default())
                    .unwrap();
                break;
            }
        }
        let mut return_matrix = self.clone();
        if first_non_zero_column == self.dimensions.width {
            return return_matrix;
        }
        return_matrix.swap_line(0, first_non_zero_line);
        for non_treated_line in 1..return_matrix.dimensions.height {
            let coeff = {
                let first_number_of_new_line = return_matrix
                    .get(non_treated_line, first_non_zero_column)
                    .unwrap();
                let pivot = return_matrix.get(0, first_non_zero_column).unwrap();
                first_number_of_new_line / pivot
            };
            for elt_index in first_non_zero_column..return_matrix.dimensions.width {
                let tmp = &coeff * return_matrix.get(0, elt_index).unwrap();
                *return_matrix.get_mut(non_treated_line, elt_index).unwrap() -= &tmp;
            }
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
            assert_eq!(res, [[1.0, 2.0], [0.0, -2.0]]);
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
            let res = u.row_echelon();
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
