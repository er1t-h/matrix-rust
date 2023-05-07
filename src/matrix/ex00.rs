use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{error::MatrixOperationError, Matrix};

// Add/Sub implementation
impl<K> Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    ///
    /// Adds another `Matrix` to self.
    ///
    /// If the size of the two Matrixes differ, a [`MatrixOperationError`] is returned
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut lhs = Matrix::from([[5, 4], [3, 2]]);
    /// let rhs = Matrix::from([[5, 6], [7, 8]]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [[10, 10], [10, 10]])
    /// ```
    ///
    /// # Errors
    /// If the dimensions of the two matrix are different, returns a
    /// [`NotSameSize`](MatrixOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    pub fn safe_add_assign(&mut self, rhs: &Self) -> Result<(), MatrixOperationError> {
        if self.dimensions != rhs.dimensions {
            return Err(MatrixOperationError::NotSameSize(
                self.dimensions,
                rhs.dimensions,
            ));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.iter()) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    ///
    /// Subs another `Matrix` from self.
    ///
    /// If the size of the two Matrixes differ, a [`MatrixOperationError`] is returned
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut lhs = Matrix::from([[10, 10], [10, 10]]);
    /// let rhs = Matrix::from([[5, 6], [7, 8]]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [[5, 4], [3, 2]])
    /// ```
    ///
    /// # Errors
    /// If the dimensions of the two matrix are different, returns a
    /// [`NotSameSize`](MatrixOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    pub fn safe_sub_assign(&mut self, rhs: &Self) -> Result<(), MatrixOperationError> {
        if self.dimensions != rhs.dimensions {
            return Err(MatrixOperationError::NotSameSize(
                self.dimensions,
                rhs.dimensions,
            ));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.iter()) {
            *lhs -= rhs;
        }
        Ok(())
    }
}

// Add traits
impl<K> AddAssign<&Self> for Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        let _ = self.safe_add_assign(rhs);
    }
}
impl<K> AddAssign for Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}
impl<K> Add<&Self> for Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<K> Add for Matrix<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

// Sub traits
impl<K> SubAssign<&Self> for Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}
impl<K> SubAssign for Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}
impl<K> Sub<&Self> for Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<K> Sub for Matrix<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

// Multiplication by a scalar
impl<K> MulAssign<&K> for Matrix<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    ///
    /// Multiply a scalar into self.
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut lhs = Matrix::from([[5, 10], [6, 3]]);
    /// lhs *= 5;
    /// assert_eq!(lhs, [[25, 50], [30, 15]])
    /// ```
    /// Complexity:
    /// Linear: O(m*n) for a `m * n` Matrix
    ///
    fn mul_assign(&mut self, rhs: &K) {
        for nb in &mut self.content {
            *nb *= rhs;
        }
    }
}
impl<K> MulAssign<K> for Matrix<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: K) {
        *self *= &rhs;
    }
}
impl<K> Mul<&K> for Matrix<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: &K) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<K> Mul<K> for Matrix<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: K) -> Self::Output {
        self * &rhs
    }
}

#[cfg(test)]
mod test {
    use crate::{complex::cpl, matrix::Dimensions, Matrix};
    use pretty_assertions::assert_eq;

    #[test]
    fn safe_add_assign() {
        {
            let mut lhs = Matrix::from([[1, 2, 3], [4, 5, 6]]);
            let rhs = Matrix::from([[6, 5, 4], [3, 2, 1]]);
            let trash = Matrix::from([[6, 5, 4, 3, 2, 1]]);

            assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[7; 3], [7; 3]]);
            assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[13, 12, 11], [10, 9, 8]]);
            assert_eq!(
                lhs.safe_add_assign(&trash),
                Err(crate::error::MatrixOperationError::NotSameSize(
                    Dimensions {
                        height: 2,
                        width: 3
                    },
                    Dimensions {
                        width: 6,
                        height: 1
                    }
                ))
            )
        }
        {
            let mut mat1 = Matrix::from([[1, 2], [3, 4]]);
            let mut mat2 = Matrix::from([[1, 6], [-12, -3]]);
            let mat3 = Matrix::from([[1, -4], [-2, 4]]);
            assert_eq!(mat2.safe_add_assign(&mat3), Ok(()));
            assert_eq!(mat2, [[2, 2], [-14, 1]]);
            assert_eq!(mat1.safe_add_assign(&mat2), Ok(()));
            assert_eq!(mat1, [[3, 4], [-11, 5]]);
        }
    }

    #[test]
    fn safe_sub_assign() {
        {
            let mut lhs = Matrix::from([[1, 2, 3], [4, 5, 6]]);
            let rhs = Matrix::from([[6, 5, 4], [3, 2, 1]]);
            let trash = Matrix::from([[6, 5, 4, 3, 2, 1]]);

            assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[-5, -3, -1], [1, 3, 5]]);
            assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[-11, -8, -5], [-2, 1, 4]]);
            assert_eq!(
                lhs.safe_sub_assign(&trash),
                Err(crate::error::MatrixOperationError::NotSameSize(
                    Dimensions {
                        height: 2,
                        width: 3
                    },
                    Dimensions {
                        width: 6,
                        height: 1
                    }
                ))
            )
        }
        {
            let mut mat1 = Matrix::from([[1, 2], [3, 4]]);
            let mut mat2 = Matrix::from([[1, 6], [-12, -3]]);
            let mat3 = Matrix::from([[1, -4], [-2, 4]]);
            assert_eq!(mat2.safe_sub_assign(&mat3), Ok(()));
            assert_eq!(mat2, [[0, 10], [-10, -7]]);
            assert_eq!(mat1.safe_sub_assign(&mat2), Ok(()));
            assert_eq!(mat1, [[1, -8], [13, 11]]);
        }
    }

    #[test]
    fn mul_assign() {
        let mut mat1 = Matrix::from([[1, 5], [8, 4]]);
        mat1 *= 4;
        assert_eq!(mat1, [[4, 20], [32, 16]]);
    }

    #[test]
    fn traits() {
        let mat1 = Matrix::from([[10, 1, 5], [5, 3, 8]]);
        let mat2 = mat1.clone() * 5;
        assert_eq!(mat2, [[50, 5, 25], [25, 15, 40]]);
        let mat3 = mat2.clone() + &mat1;
        assert_eq!(mat3, [[60, 6, 30], [30, 18, 48]]);
        let mat4 = mat3 - &mat1;
        assert_eq!(mat4, mat2);
    }

    #[test]
    fn example() {
        {
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u += v;
            println!("{}", u);
            assert_eq!(u, [[8., 6.], [1., 6.]]);
        }
        {
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u -= v;
            println!("{}", u);
            assert_eq!(u, [[-6., -2.], [5., 2.]]);
        }
        {
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            u *= 2.;
            println!("{}", u);
            assert_eq!(u, [[2., 4.], [6., 8.]]);
        }
    }

    #[test]
    fn add_with_complex() {
        let mut u = Matrix::from([[cpl!(1., 2.), cpl!(3., 4.)], [cpl!(5., 6.), cpl!(7., 8.)]]);
        let v = Matrix::from([[cpl!(8., 7.), cpl!(6., 5.)], [cpl!(4., 3.), cpl!(2., 1.)]]);
        u += &v;
        assert_eq!(u, Matrix::fill(cpl!(9., 9.), 2, 2).unwrap())
    }

    #[test]
    fn sub_with_complex() {
        let mut u = Matrix::from([[cpl!(1., 2.), cpl!(3., 4.)], [cpl!(5., 6.), cpl!(7., 8.)]]);
        let v = Matrix::from([[cpl!(8., 7.), cpl!(6., 5.)], [cpl!(4., 3.), cpl!(2., 1.)]]);
        u -= &v;
        assert_eq!(
            u,
            [
                [cpl!(-7., -5.), cpl!(-3., -1.)],
                [cpl!(1., 3.), cpl!(5., 7.)]
            ]
        );
    }

    #[test]
    fn scale_mul_with_complex() {
        let mut u = Matrix::from([[cpl!(1., 2.), cpl!(3., 4.)], [cpl!(5., 6.), cpl!(7., 8.)]]);
        u *= cpl!(5., 2.);
        assert_eq!(
            u,
            [
                [cpl!(1., 12.), cpl!(7., 26.)],
                [cpl!(13., 40.), cpl!(19., 54.)]
            ]
        );
    }
}
