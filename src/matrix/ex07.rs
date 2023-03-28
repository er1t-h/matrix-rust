use std::{
    iter::Sum,
    ops::{AddAssign, Mul},
};

use crate::{error::MulVecError, Matrix, Vector};

impl<K> Matrix<K>
where
    K: Clone + Sum + Default + AddAssign,
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
        Ok(self.mul_vec_internal(vec.iter(), vec.len()))
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
        self.mul_vec_internal(vec.iter(), vec.size())
    }

    #[inline(always)]
    fn mul_vec_internal<'a, T>(&self, vec: T, size: usize) -> Vector<K>
    where
        T: Iterator<Item = &'a K>,
        K: 'a,
    {
        let mut result_vec: Vector<K> = Vector::fill(&K::default(), size);
        for (vector_index, vector_elt) in vec.enumerate() {
            let col = self.get_column(vector_index).unwrap();
            for (index, matrix_elt) in col.enumerate() {
                *result_vec.get_mut(index).unwrap() += matrix_elt * vector_elt;
            }
        }
        result_vec
    }

    pub fn mul_mat(&self, rhs: &Self) -> Result<Self, ()> {
        let mut return_matrix =
            Matrix::from(self.mul_vec_internal(rhs.get_column(0).unwrap(), rhs.dimensions.height));
        for index in 1..rhs.dimensions.width {
            let return_vec =
                self.mul_vec_internal(rhs.get_column(index).unwrap(), rhs.dimensions.height);
            return_matrix.append_column(&return_vec);
        }
        Ok(return_matrix)
    }
}

#[cfg(test)]
mod test {
    use crate::{error::MulVecError, Matrix, Vector};

    #[test]
    fn example_vec() {
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
    fn example_mat() {
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let v = Matrix::from([[1., 0.], [0., 1.]]);
            let res = u.mul_mat(&v).unwrap();
            let expect = [[1., 0.], [0., 1.]];
            assert_eq!(res, expect);
            println!("{} * {} = {}", u, v, res);
        }
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let v = Matrix::from([[2., 1.], [4., 2.]]);
            let res = u.mul_mat(&v).unwrap();
            let expect = [[2., 1.], [4., 2.]];
            assert_eq!(res, expect);
            println!("{} * {} = {}", u, v, res);
        }
        {
            let u = Matrix::from([[3., -5.], [6., 8.]]);
            let v = Matrix::from([[2., 1.], [4., 2.]]);
            let res = u.mul_mat(&v).unwrap();
            let expect = [[-14., -7.], [44., 22.]];
            assert_eq!(res, expect);
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