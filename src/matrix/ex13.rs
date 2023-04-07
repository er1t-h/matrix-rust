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
    ///
    /// Returns the rank of a matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    /// assert_eq!(u.rank(), 2);
    /// ```
    ///
    pub fn rank(&self) -> usize {
        let mul_identity = K::mul_identity();
        let return_matrix = self.reduced_row_echelon();
        for i in 0..self.dimensions.height.min(self.dimensions.width) {
            if return_matrix.get(i, i).unwrap() != &mul_identity {
                return i;
            }
        }
        self.dimensions.width
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            let res = u.rank();
            assert_eq!(res, 3);
            println!("{}", res);
        }
        {
            let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
            let res = u.rank();
            assert_eq!(res, 2);
            println!("{}", res);
            // 2
        }
        {
            let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
            let res = u.rank();
            assert_eq!(res, 3);
            println!("{}", res);
            // 3
        }
    }
}
