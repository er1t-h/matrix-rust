use std::{iter::Sum, ops::Mul};

use crate::{error::MulVecError, Matrix, Vector};

impl<K> Matrix<K>
where
    K: Clone + Sum,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    ///
    /// Multiply a [Vector] by a [Matrix], and returns the corresponding [Vector].
    /// If the dimensions of the [Vector] doesn't correspond to those of the
    /// [Matrix], a [MulVecError] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    /// use matrix::Vector;
    ///
    /// let u = Matrix::from([[1., 0.], [0., 1.]]);
    /// let v = Vector::from([4., 2.]);
    /// let res = u.mul_vec(&v).unwrap();
    /// assert_eq!(res, [4., 2.]);
    /// ```
    ///
    /// # Complexity:
    /// For a `m` * `n` matrix and an `n`-vector.
    ///
    /// Time: O(mn)
    /// Space: O(m)
    ///
    pub fn mul_vec(&self, vec: &Vector<K>) -> Result<Vector<K>, MulVecError> {
        if self.dimensions.width != vec.len() {
            return Err(MulVecError::SizeMismatch(self.dimensions.width, vec.len()));
        }
        let mut result_vec: Vector<K> = Vector::new();
        for index in 0..self.dimensions.height {
            let matrix_line = self.content[index * self.dimensions.width..]
                .iter()
                .take(self.dimensions.width);
            result_vec.append(matrix_line.zip(vec.iter()).map(|(mx, vx)| mx * vx).sum())
        }
        Ok(result_vec)
    }

    ///
    /// Multiply a [Vector] by a [Matrix], and returns the corresponding [Vector].
    ///
    /// # Safety
    /// Make sure that the number of column of self is the same as the size of
    /// `vec`, or a non-sensical value might be returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Matrix;
    /// use matrix::Vector;
    ///
    /// let u = Matrix::from([[1., 0.], [0., 1.]]);
    /// let v = Vector::from([4., 2.]);
    /// let res = unsafe { u.mul_vec_unchecked(&v) };
    /// assert_eq!(res, [4., 2.]);
    /// ```
    ///
    /// # Complexity:
    /// For a `m` * `n` matrix and an `n`-vector.
    ///
    /// Time: O(mn)
    /// Space: O(m)
    ///
    pub unsafe fn mul_vec_unchecked(&self, vec: &Vector<K>) -> Vector<K> {
        let mut result_vec: Vector<K> = Vector::new();
        for index in 0..self.dimensions.height {
            let matrix_line = self.content[index * self.dimensions.width..]
                .iter()
                .take(self.dimensions.width);
            result_vec.append(matrix_line.zip(vec.iter()).map(|(mx, vx)| mx * vx).sum())
        }
        result_vec
    }
}

#[cfg(test)]
mod test {
    use crate::{error::MulVecError, Matrix, Vector};

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let v = Vector::from([4., 2.]);
            let res = u.mul_vec(&v).unwrap();
            assert_eq!(res, [4., 2.]);
            println!("{} * {} = {}", u, v, res);
        }
        {
            let u = Matrix::from([[2., 0.], [0., 2.]]);
            let v = Vector::from([4., 2.]);
            let res = u.mul_vec(&v).unwrap();
            assert_eq!(res, [8., 4.]);
            println!("{} * {} = {}", u, v, res);
        }
        {
            let u = Matrix::from([[2., -2.], [-2., 2.]]);
            let v = Vector::from([4., 2.]);
            let res = u.mul_vec(&v).unwrap();
            assert_eq!(res, [4., -4.]);
            println!("{} * {} = {}", u, v, res);
        }
    }

    #[test]
    fn errors() {
        {
            let u = Matrix::from([[1., 0., 0.75], [0., 1., 1.25]]);
            let v = Vector::from([4., 2.]);
            let res = u.mul_vec(&v);
            assert_eq!(res, Err(MulVecError::SizeMismatch(3, 2)));
        }
    }
}
