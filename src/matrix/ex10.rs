use crate::Matrix;

use super::Dimensions;

impl<K> Matrix<K>
where
    K: Clone + Default,
    for<'a> &'a K: PartialEq,
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
        if first_non_zero_column == self.dimensions.width {
            return self.clone(); // If all columns are zero, we return a clone of self
        }
        let first_line = self.get_line_slice(first_non_zero_line).unwrap();
        let mut return_matrix = Matrix {
            content: Vec::from(first_line),
            dimensions: Dimensions {
                height: 1,
                width: self.dimensions.width,
            },
        };
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
            assert_eq!(res, [[1.0, 0.0], [0.0, 1.0]]);
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
