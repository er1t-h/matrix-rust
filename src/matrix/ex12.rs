use std::ops::{Div, DivAssign, Mul, MulAssign, SubAssign};

use crate::{
    traits::{IsZero, MulIdentity},
    Matrix,
};

impl<K> Matrix<K>
where
    K: Clone + MulIdentity + Default,
    for<'a> K: MulAssign<&'a K> + SubAssign<&'a K> + DivAssign<&'a K>,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K> + IsZero,
{
    pub fn inverse(&self) -> Result<Self, ()> {
        if self.dimensions.height != self.dimensions.width {
            return Err(());
        }
        let mul_identity = K::mul_identity();
        let mut return_matrix = Matrix::augmented_matrix(
            self,
            &Matrix::identity(&mul_identity, self.dimensions.height).unwrap(),
        )
        .unwrap();
        return_matrix = return_matrix.reduced_row_echelon();
        for i in 0..self.dimensions.height {
            if return_matrix.get(i, i).unwrap() == &mul_identity {
                return Err(());
            }
        }
        Ok(return_matrix
            .submatrix(
                self.dimensions.width..self.dimensions.width * 2 - 1,
                0..self.dimensions.height,
            )
            .unwrap())
    }
}
