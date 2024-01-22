use std::iter::Sum;
use std::ops::{AddAssign, Mul};

use crate::{error::MulMatError, Matrix};

impl<K> Matrix<K>
where
    K: Clone + Sum + Default + AddAssign,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    ///
    /// Multiplies a [Matrix] by another one, and returns the corresponding
    /// Matrix.
    ///
    /// If your input is already verified, you can use
    /// [`mul_mat_unchecked`](Matrix#method.mul_mat_unchecked).
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// use matrix::error::MulMatError;
    ///
    /// let mat1 = Matrix::from([[1, 2], [3, 4]]);
    /// let mat2 = Matrix::from([[1, 2], [3, 4], [5, 6]]);
    /// assert_eq!(mat1.mul_mat(&mat2), Err(MulMatError::SizeMismatch(2, 3)));
    /// ```
    ///
    /// # Errors
    /// If the width of `self` is not equal to the height of `rhs`, returns a
    /// [`SizeMismatch`](MulMatError::SizeMismatch)
    ///
    /// # Complexity
    /// For a `m` * `n` matrix, and a `n` * `p` matrix.
    ///
    /// Time: O(mnp)
    /// Space: O(mp)
    ///
    pub fn mul_mat(&self, rhs: &Self) -> Result<Self, MulMatError> {
        if self.dimensions.width != rhs.dimensions.height {
            return Err(MulMatError::SizeMismatch(
                self.dimensions.width,
                rhs.dimensions.height,
            ));
        }
        Ok(self.mul_mat_internal(rhs))
    }

    ///
    /// Multiplies a [Matrix] by another one, and returns the corresponding
    /// Matrix.
    ///
    /// # Safety
    /// The number of column of the left matrix must match the number of line of
    /// the right matrix. See [`mul_mat`](Matrix#method.mul_mat) for a safe
    /// method.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// use matrix::error::MulMatError;
    ///
    /// let mat1 = Matrix::from([[3., -5.], [6., 8.]]);
    /// let mat2 = Matrix::from([[2., 1.], [4., 2.]]);
    /// assert_eq!(unsafe { mat1.mul_mat_unchecked(&mat2) }, [[-14., -7.], [44., 22.]]);
    /// ```
    ///
    /// # Complexity
    /// For a `m` * `n` matrix, and a `n` * `p` matrix.
    ///
    /// Time: O(mnp)
    /// Space: O(mp)
    ///
    pub unsafe fn mul_mat_unchecked(&self, rhs: &Self) -> Self {
        self.mul_mat_internal(rhs)
    }

    ///
    /// Multiplies a [Matrix] by another one, and returns the corresponding
    /// Matrix.
    ///
    /// # Safety
    /// The number of column of the left matrix must match the number of line of
    /// the right matrix. See [`mul_mat`](Matrix#method.mul_mat) for a safe
    /// method.
    ///
    /// # Complexity
    /// For a `m` * `n` matrix, and a `n` * `p` matrix.
    ///
    /// Time: O(mnp)
    /// Space: O(mp)
    ///

    #[inline(always)]
    fn mul_mat_internal(&self, rhs: &Self) -> Self {
        // A matrix has is at least a 1x1 matrix, so `get_column(0)` will always return Some
        let mut return_matrix =
            Self::from(self.mul_vec_internal(rhs.get_column(0).unwrap_or_else(|| unreachable!()), rhs.dimensions.height));
        for index in 1..rhs.dimensions.width {
            // We use `index`, which is bounded by `rhs.dimensions.width`
            let return_vec =
                self.mul_vec_internal(rhs.get_column(index).unwrap_or_else(|| unreachable!()), rhs.dimensions.height);
            return_matrix.append_column(&return_vec);
        }
        return_matrix
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{complex::cpl, Matrix};

    #[test]
    fn example_mat() {
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let v = Matrix::from([[1., 0.], [0., 1.]]);
            let res = u.mul_mat(&v).unwrap();
            let expect = [[1., 0.], [0., 1.]];
            assert_eq!(res, expect);
            println!("{u} * {v} = {res}");
        }
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let v = Matrix::from([[2., 1.], [4., 2.]]);
            let res = u.mul_mat(&v).unwrap();
            let expect = [[2., 1.], [4., 2.]];
            assert_eq!(res, expect);
            println!("{u} * {v} = {res}");
        }
        {
            let u = Matrix::from([[3., -5.], [6., 8.]]);
            let v = Matrix::from([[2., 1.], [4., 2.]]);
            let res = u.mul_mat(&v).unwrap();
            let expect = [[-14., -7.], [44., 22.]];
            assert_eq!(res, expect);
            println!("{u} * {v} = {res}");
        }
    }

    #[test]
    fn mat_with_complex() {
        let u = Matrix::from([[cpl!(1, 5), cpl!(3, 2)], [cpl!(9, 0), cpl!(12, -4)]]);
        let v = Matrix::from([[cpl!(0, 5), cpl!(32, 1)], [cpl!(21, -7), cpl!(0, 0)]]);
        let res = u.mul_mat(&v).unwrap();
        assert_eq!(
            res,
            [
                [cpl!(52, 26), cpl!(27, 161)],
                [cpl!(224, -123), cpl!(288, 9)]
            ]
        );
    }
}
