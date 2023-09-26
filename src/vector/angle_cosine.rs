//!
//! Angle Cos
//!

use std::{
    iter::Sum,
    ops::{Div, Mul},
};

use crate::{
    error::VectorOperationError,
    traits::{Divisor, Sqrt},
    Vector,
};

impl<K> Vector<K>
where
    K: Clone + Sum + Sqrt + Divisor + Mul<Output = K> + Div<Output = K> + PartialEq<K>,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    ///
    /// Returns the cosine of the angle formed by the two vectors.
    ///
    /// Consider using [`angle_cos_unchecked`](Vector#method.angle_cos_unchecked) if you're sure that your inputs are correct.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::error::VectorOperationError;
    ///
    /// let v1 = Vector::from([1.0, 0.0]);
    /// let v2 = Vector::from([0.0, 0.0]);
    /// assert_eq!(Vector::angle_cos(&v1, &v2), Err(VectorOperationError::ZeroVector));
    /// ```
    ///
    /// # Errors
    /// If the two vectors does not have the same len, returns [`NotSameSize`](VectorOperationError::NotSameSize)
    /// If the product of the norm of the two vector is zero, returns [`ZeroVector`](VectorOperationError::ZeroVector)
    ///
    /// # Complexity:
    /// Linear: O(n) with `n` the total number of coordinates
    ///
    pub fn angle_cos(u: &Self, v: &Self) -> Result<K, VectorOperationError> {
        if u.len() != v.len() {
            return Err(VectorOperationError::NotSameSize(u.len(), v.len()));
        }
        let norm_product = u.norm() * v.norm();
        if !norm_product.can_be_divisor() {
            return Err(VectorOperationError::ZeroVector);
        }
        Ok(unsafe { u.dot_unchecked(v) } / norm_product)
    }

    ///
    /// Returns the cosine of the angle formed by the two vectors.
    ///
    /// # Safety
    /// A  zero vector or size difference between the vectors can cause undefined
    /// behaviour or can panic the program.
    /// Consider using [`angle_cos`](crate::Vector#method.angle_cos) if you're not sure that your inputs are
    /// correct.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::error::VectorOperationError;
    ///
    /// let v1 = Vector::from([1.0, 0.0]);
    /// let v2 = Vector::from([1.0, 0.0]);
    /// assert_eq!(unsafe { Vector::angle_cos_unchecked(&v1, &v2) }, 1.0);
    /// ```
    ///
    /// # Complexity:
    /// Linear: O(n) with `n` the total number of coordinates
    ///
    #[must_use]
    pub unsafe fn angle_cos_unchecked(u: &Self, v: &Self) -> K {
        let tmp = u.dot_internal(v);
        tmp / (u.norm() * v.norm())
    }
}

#[cfg(test)]
mod test {
    use crate::error::VectorOperationError;
    use crate::{assert_eq_float, Vector};

    #[test]
    fn example() {
        {
            let u = Vector::from([1., 0.]);
            let v = Vector::from([1., 0.]);
            let res = Vector::angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 1.0);
            println!("cos({u}, {v}) = {res}");
            // 1.0
        }
        {
            let u = Vector::from([1., 0.]);
            let v = Vector::from([0., 1.]);
            let res = Vector::angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 0.0);
            println!("cos({u}, {v}) = {res}");
            // 0.0
        }
        {
            let u = Vector::from([-1., 1.]);
            let v = Vector::from([1., -1.]);
            let res = Vector::angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, -1.0);
            println!("cos({u}, {v}) = {res}");
            // -1.0
        }
        {
            let u = Vector::from([2., 1.]);
            let v = Vector::from([4., 2.]);
            let res = Vector::angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 1.0);
            println!("cos({u}, {v}) = {res}");
            // 1.0
        }
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            let res = Vector::angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 0.974_631_846);
            println!("cos({u}, {v}) = {res}");
            // 0.974631846
        }
    }

    #[test]
    fn errors() {
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5.]);
            assert_eq!(
                Vector::angle_cos(&u, &v),
                Err(VectorOperationError::NotSameSize(3, 2))
            );
        }
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([0., 0., 0.]);
            assert_eq!(
                Vector::angle_cos(&u, &v),
                Err(VectorOperationError::ZeroVector)
            );
        }
    }

    // * Can't adapt to complex: Needs sqrt, which have multiple possibilities
}
