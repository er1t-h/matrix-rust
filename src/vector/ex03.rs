use std::{iter::Sum, ops::Mul};

use crate::{error::VectorOperationError, Vector};

impl<K> Vector<K>
where
    K: Clone + Sum,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    pub fn safe_dot(&self, v: &Self) -> Result<K, VectorOperationError> {
        if self.len() != v.len() {
            Err(VectorOperationError::NotSameSize(self.len(), v.len()))
        } else {
            Ok(self.dot_internal(v))
        }
    }

    ///
    /// Returns the dot product of the the two vectors.
    ///
    /// Using the [safe_dot](Vector::safe_dot) method, you get a [Result], whereas this function
    /// will return a wrong answer in case of a bad input
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec1 = Vector::from([4, 2, 3]);
    /// let vec2 = Vector::from([4, 2, 3]);
    /// assert_eq!(vec1.dot(&vec2), 29);
    /// ```
    ///
    /// Complexity:
    /// Linear: O(n) with `n` the total number of coordinates of the vectors.
    ///
    pub fn dot(&self, v: &Self) -> K {
        self.dot_internal(v)
    }

    #[inline(always)]
    fn dot_internal(&self, v: &Self) -> K {
        self.content
            .iter()
            .zip(v.content.iter())
            .map(|(lhs, rhs)| lhs * rhs)
            .sum()
    }
}
