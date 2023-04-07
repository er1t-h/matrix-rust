use std::ops::AddAssign;

use crate::{error::TraceError, Matrix};

impl<K> Matrix<K>
where
    K: Clone,
    for<'a> K: AddAssign<&'a K>,
{
    ///
    /// Returns the trace of a matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// use matrix::error::TraceError;
    ///
    /// let mat = Matrix::from([[2, 1], [5, 3], [5, 3]]);
    /// assert_eq!(mat.trace(), Err(TraceError::NotSquareMatrix));
    ///
    /// let mat = Matrix::from([[2, 1], [5, 3]]);
    /// assert_eq!(mat.trace(), Ok(5));
    /// ```
    ///
    /// # Complexity
    /// For an `n` * `n` matrix:
    /// Time: O(n)
    /// Space: O(1)
    ///
    pub fn trace(&self) -> Result<K, TraceError> {
        if !self.is_square() {
            return Err(TraceError::NotSquareMatrix);
        }
        Ok(self.trace_internal())
    }

    ///
    /// Returns the trace of a matrix.
    ///
    /// # Safety
    /// Make sure that the matrix is a square matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// use matrix::error::TraceError;
    ///
    /// let mat = Matrix::from([[2, 1], [5, 3]]);
    /// assert_eq!(unsafe { mat.trace_unchecked() }, 5);
    /// ```
    ///
    /// # Complexity
    /// For an `n` * `n` matrix:
    /// Time: O(n)
    /// Space: O(1)
    ///
    pub unsafe fn trace_unchecked(&self) -> K {
        self.trace_internal()
    }

    #[inline(always)]
    fn trace_internal(&self) -> K {
        let mut accumulator = self.content.get(0).unwrap().clone();
        for i in 1..self.dimensions.height {
            accumulator += self.get(i, i).unwrap()
        }
        accumulator
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let res = u.trace().unwrap();
            assert_eq!(res, 2.0);
            println!("trace({}) = {}", u, res);
        }
        {
            let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
            let res = u.trace().unwrap();
            assert_eq!(res, 9.0);
            println!("trace({}) = {}", u, res);
        }
        {
            let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
            let res = u.trace().unwrap();
            assert_eq!(res, -21.0);
            println!("trace({}) = {}", u, res);
        }
    }
}
