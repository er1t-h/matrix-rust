use std::ops::{Div, DivAssign, Mul, MulAssign, SubAssign};

use crate::{
    traits::{IsZero, One},
    Matrix,
};

impl<K> Matrix<K>
where
    for<'a> K: Clone + One + Default + MulAssign<&'a K> + SubAssign<&'a K> + DivAssign<&'a K>,
    for<'a> &'a K: PartialEq + Mul<&'a K, Output = K> + Div<&'a K, Output = K> + IsZero,
{
    ///
    /// Returns the rank of a matrix.
    ///
    /// # Panics
    /// Never.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    /// assert_eq!(u.rank(), 2);
    /// ```
    ///
    #[must_use]
    pub fn rank(&self) -> usize {
        let mul_identity = K::one();
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

    use crate::{complex::cpl, Matrix};

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            let res = u.rank();
            assert_eq!(res, 3);
            println!("{res}");
        }
        {
            let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
            let res = u.rank();
            assert_eq!(res, 2);
            println!("{res}");
            // 2
        }
        {
            let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
            let res = u.rank();
            assert_eq!(res, 3);
            println!("{res}");
            // 3
        }
    }

    #[test]
    fn with_complex() {
        let u = Matrix::from([
            [cpl!(1. + 2. i), cpl!(2. + 1. i), cpl!(4. - 4. i)],
            [cpl!(2. + 4. i), cpl!(4. + 2. i), cpl!(8. - 8. i)],
            [cpl!(3. + 5. i), cpl!(5. - 2. i), cpl!(0. + 3. i)],
        ]);
        let res = u.rank();
        assert_eq!(res, 2);
    }
}
