use std::ops::{AddAssign, Mul};

use crate::{const_matrix::ConstMatrix, const_vector::ConstVector};

impl<K, const ROW_LHS: usize, const COL_LHS_ROW_RHS: usize, const COL_RHS: usize>
    Mul<ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>> for ConstMatrix<K, ROW_LHS, COL_LHS_ROW_RHS>
where
    K: Mul<K, Output = K> + AddAssign + Clone,
{
    type Output = ConstMatrix<K, ROW_LHS, COL_RHS>;
    fn mul(self, rhs: ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>) -> Self::Output {
        // `column_rhs` yields exactly `COL_RHS` elements
        let mut columns_rhs = rhs.iter_all_col_value().into_iter();
        // std::array::from_fn will run exactly `COL_RHS` times
        let to_transpose = std::array::from_fn(|_| {
            ConstVector::<K, COL_LHS_ROW_RHS>::mul_val_val(
                columns_rhs.next().unwrap_or_else(|| unreachable!()),
                self.clone().iter_all_col_value(),
            )
        });
        let tmp = ConstMatrix::from(to_transpose);
        tmp.transpose()
    }
}

impl<K, const ROW_LHS: usize, const COL_LHS_ROW_RHS: usize, const COL_RHS: usize>
    Mul<&ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>> for ConstMatrix<K, ROW_LHS, COL_LHS_ROW_RHS>
where
    for<'a> K: Mul<&'a K, Output = K> + AddAssign + Clone,
{
    type Output = ConstMatrix<K, ROW_LHS, COL_RHS>;
    fn mul(self, rhs: &ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>) -> Self::Output {
        // `column_rhs` yields exactly `COL_RHS` elements
        let mut columns_rhs = rhs.iter_all_col().into_iter();
        // std::array::from_fn will run exactly `COL_RHS` times
        let to_transpose = std::array::from_fn(|_| {
            ConstVector::<K, COL_LHS_ROW_RHS>::mul_ref_val(
                columns_rhs.next().unwrap_or_else(|| unreachable!()),
                self.clone().iter_all_col_value(),
            )
        });
        let tmp = ConstMatrix::from(to_transpose);
        tmp.transpose()
    }
}

impl<K, const ROW_LHS: usize, const COL_LHS_ROW_RHS: usize, const COL_RHS: usize>
    Mul<ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>> for &ConstMatrix<K, ROW_LHS, COL_LHS_ROW_RHS>
where
    for<'a> K: AddAssign + Clone + Mul<&'a K, Output = K>,
{
    type Output = ConstMatrix<K, ROW_LHS, COL_RHS>;
    fn mul(self, rhs: ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>) -> Self::Output {
        // `column_rhs` yields exactly `COL_RHS` elements
        let mut columns_rhs = rhs.iter_all_col_value().into_iter();
        // std::array::from_fn will run exactly `COL_RHS` times
        let to_transpose = std::array::from_fn(|_| {
            ConstVector::<K, COL_LHS_ROW_RHS>::mul_val_ref(
                columns_rhs.next().unwrap_or_else(|| unreachable!()),
                self.iter_all_col(),
            )
        });
        let tmp = ConstMatrix::from(to_transpose);
        tmp.transpose()
    }
}

// TODO: Not working because recursion at compile time
// impl<K, const ROW_LHS: usize, const COL_LHS_ROW_RHS: usize, const COL_RHS: usize>
//     Mul<&ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>> for &ConstMatrix<K, ROW_LHS, COL_LHS_ROW_RHS>
// where
//     K: AddAssign + Clone,
//     for <'a> &'a K: Mul<Output = K>
// {
//     type Output = ConstMatrix<K, ROW_LHS, COL_RHS>;
//     fn mul(self, rhs: &ConstMatrix<K, COL_LHS_ROW_RHS, COL_RHS>) -> Self::Output {
//         let mut columns_rhs = rhs.iter_all_col().into_iter();
//         let to_transpose = std::array::from_fn(|_| {
//             ConstVector::<K, COL_LHS_ROW_RHS>::mul_ref_ref(columns_rhs.next().unwrap(), self.iter_all_col())
//         });
//         let tmp = ConstMatrix::from(to_transpose);
//         tmp.transpose()
//     }
// }

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;
    #[test]
    fn example_mat() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let res = u * v;
            let expect = [[1., 0.], [0., 1.]];
            assert_eq!(res, ConstMatrix::from(expect));
        }
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let v = ConstMatrix::from([[2., 1.], [4., 2.]]);
            let res = u * v;
            let expect = [[2., 1.], [4., 2.]];
            assert_eq!(res, ConstMatrix::from(expect));
        }
        {
            let u = ConstMatrix::from([[3., -5.], [6., 8.]]);
            let v = ConstMatrix::from([[2., 1.], [4., 2.]]);
            let res = u * v;
            let expect = [[-14., -7.], [44., 22.]];
            assert_eq!(res, ConstMatrix::from(expect));
        }
    }

    #[test]
    fn not_same_size() {
        {
            let u = ConstMatrix::from([[1., 2.], [3., 4.], [5., 6.]]);
            let v = ConstMatrix::from([[1.], [2.]]);
            let res = u * v;
            let expect = [[5.], [11.], [17.]];
            assert_eq!(res, ConstMatrix::from(expect));
        }
    }
}
