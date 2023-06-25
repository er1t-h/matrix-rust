use std::{iter::Sum, ops::Mul};

use crate::{error::VectorOperationError, Vector};

impl<K> Vector<K>
where
    K: Clone + Sum,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    ///
    /// Returns the dot product of the the two vectors.
    ///
    /// If you're sure that your input is valid, you can use [`dot_unchecked`](Vector#method.dot_unchecked).
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    /// use matrix::error::VectorOperationError;
    ///
    /// let vec1 = Vector::from([4, 2, 3]);
    /// let vec2 = Vector::from([2, 3]);
    /// assert_eq!(vec1.dot(&vec2), Err(VectorOperationError::NotSameSize(3, 2)));
    /// ```
    ///
    /// # Errors
    /// If the len of the two vectors differ, returns a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// Complexity:
    /// Linear: O(n) with `n` the total number of coordinates of the vectors.
    ///
    pub fn dot(&self, v: &Self) -> Result<K, VectorOperationError> {
        if self.len() == v.len() {
            Ok(self.dot_internal(v))
        } else {
            Err(VectorOperationError::NotSameSize(self.len(), v.len()))
        }
    }

    ///
    /// Returns the dot product of the the two vectors.
    ///
    /// Using the [`dot`](Vector#method.dot) method, you get a [Result], whereas this function
    /// will return a wrong answer in case of a bad input.
    ///
    /// # Safety
    /// Make sure both vectors have the same size, or a non-sensical result
    /// might be returned.
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec1 = Vector::from([4, 2, 3]);
    /// let vec2 = Vector::from([4, 2, 3]);
    /// assert_eq!(unsafe { vec1.dot_unchecked(&vec2) }, 29);
    /// ```
    ///
    /// # Complexity:
    /// Linear: O(n) with `n` the total number of coordinates of the vectors.
    ///
    #[must_use]
    pub unsafe fn dot_unchecked(&self, v: &Self) -> K {
        self.dot_internal(v)
    }

    #[inline(always)]
    pub(crate) fn dot_internal(&self, v: &Self) -> K {
        self.content
            .iter()
            .zip(v.content.iter())
            .map(|(lhs, rhs)| lhs * rhs)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::{complex::cpl, Vector};

    #[test]
    fn example() {
        {
            let u = Vector::from([0., 0.]);
            let v = Vector::from([1., 1.]);
            let res = u.dot(&v).unwrap();
            assert_eq!(res, 0.0);
            println!("{}", res);
        }
        {
            let u = Vector::from([1., 1.]);
            let v = Vector::from([1., 1.]);
            let res = u.dot(&v).unwrap();
            assert_eq!(res, 2.0);
            println!("{}", res);
        }
        {
            let u = Vector::from([-1., 6.]);
            let v = Vector::from([3., 2.]);
            let res = u.dot(&v).unwrap();
            assert_eq!(res, 9.0);
            println!("{}", res);
        }
    }

    #[test]
    fn error() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 1.]);
        let res = u.dot(&v);
        assert_eq!(
            res,
            Err(crate::error::VectorOperationError::NotSameSize(3, 2))
        );
    }

    #[test]
    fn with_complex() {
        let u = Vector::from([cpl!(1. + 5. i), cpl!(4. - 2. i), cpl!(-3. - 8. i)]);
        let v = Vector::from([cpl!(4. + 9. i), cpl!(-3. + 8. i), cpl!(0. + 2. i)]);
        let res = u.dot(&v);
        assert_eq!(res, Ok(cpl!(-21. + 61. i)));
    }
}
