use std::ops::Mul;

use crate::const_matrix::ConstMatrix;

// impl<K, const ROW_LHS_COL_RHS: usize, const COL_LHS: usize, const ROW_RHS: usize>
//     Mul<ConstMatrix<K, ROW_RHS, ROW_LHS_COL_RHS>> for ConstMatrix<K, ROW_LHS_COL_RHS, COL_LHS>
// {
//     type Output = ConstMatrix<K, ROW_RHS, COL_LHS>;
//     fn mul(self, rhs: ConstMatrix<K, ROW_RHS, ROW_LHS_COL_RHS>) -> Self::Output {}
// }
