use std::ops::MulAssign;

use crate::{error::TraceError, Matrix};

impl<K> Matrix<K>
where
    K: Clone,
    for<'a> K: MulAssign<&'a K>,
{
    ///
    /// Returns the product of all diagonal elements of a matrix.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// use matrix::error::TraceError;
    ///
    /// let mat = Matrix::from([[2, 1], [5, 3], [5, 3]]);
    /// assert_eq!(mat.multiplicative_trace(), Err(TraceError::NotSquareMatrix));
    ///
    /// let mat = Matrix::from([[2, 1], [5, 3]]);
    /// assert_eq!(mat.multiplicative_trace(), Ok(6));
    /// ```
    ///
    /// # Complexity
    /// For an `n` * `n` matrix:
    /// Time: O(n)
    /// Space: O(1)
    ///
    pub fn multiplicative_trace(&self) -> Result<K, TraceError> {
        if !self.is_square() {
            return Err(TraceError::NotSquareMatrix);
        }
        Ok(self.multiplicative_trace_internal())
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
    /// assert_eq!(unsafe { mat.multiplicative_trace_unchecked() }, 6);
    /// ```
    ///
    /// # Complexity
    /// For an `n` * `n` matrix:
    /// Time: O(n)
    /// Space: O(1)
    ///
    pub unsafe fn multiplicative_trace_unchecked(&self) -> K {
        self.multiplicative_trace_internal()
    }

    #[inline(always)]
    pub(crate) fn multiplicative_trace_internal(&self) -> K {
        let mut accumulator = self.content.get(0).unwrap().clone();
        for i in 1..self.dimensions.height {
            accumulator *= self.get(i, i).unwrap()
        }
        accumulator
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let res = u.multiplicative_trace().unwrap();
            assert_eq!(res, 1.0);
            println!("multiplicative_trace({}) = {}", u, res);
        }
        {
            let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
            let res = u.multiplicative_trace().unwrap();
            assert_eq!(res, 24.0);
            println!("multiplicative_trace({}) = {}", u, res);
        }
        {
            let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
            let res = u.multiplicative_trace().unwrap();
            assert_eq!(res, 184.0);
            println!("multiplicative_trace({}) = {}", u, res);
        }
    }
}
