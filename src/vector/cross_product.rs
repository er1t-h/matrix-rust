use std::ops::{Mul, Sub};

use crate::{error::CrossProductError, Vector};

impl<K> Vector<K>
where
    K: Clone,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K>,
{
    ///
    /// Computes the cross product of two vectors.
    ///
    /// # Panics:
    /// Never.
    ///
    /// # Note:
    /// You can use [`cross_product_unchecked`](crate::Vector#method.cross_product_unchecked) if your inputs are already checked.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::error::CrossProductError;
    ///
    /// let v1 = Vector::from([1, 0, 0]);
    /// let v2 = Vector::from([0, 1, 0, 2]);
    /// assert_eq!(Vector::cross_product(&v1, &v2), Err(CrossProductError::RightVectorShouldBeThreeDimensional));
    /// ```
    ///
    /// # Errors
    /// If len of left vector is not 3, returns a [`LeftVectorShouldBeThreeDimensional`](crate::error::CrossProductError::LeftVectorShouldBeThreeDimensional)
    /// If len of right vector is not 3, returns a [`RightVectorShouldBeThreeDimensional`](crate::error::CrossProductError::RightVectorShouldBeThreeDimensional)
    ///
    /// # Complexity:
    /// Constant
    ///
    pub fn cross_product(u: &Self, v: &Self) -> Result<Self, CrossProductError> {
        if u.len() != 3 {
            Err(CrossProductError::LeftVectorShouldBeThreeDimensional)
        } else if v.len() != 3 {
            Err(CrossProductError::RightVectorShouldBeThreeDimensional)
        } else {
            Ok(Self::cross_product_internal(u, v))
        }
    }

    ///
    /// Computes the cross product of two vectors.
    ///
    /// # Safety
    /// If one of the two vectors is not three dimensional, the behaviour is
    /// undefined.
    /// Please use [`cross_product`](crate::Vector#method.cross_product) for a safe alternative.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([1, 0, 0]);
    /// let v2 = Vector::from([0, 1, 0]);
    /// assert_eq!(unsafe { Vector::cross_product_unchecked(&v1, &v2) }, [0, 0, 1]);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    pub unsafe fn cross_product_unchecked(u: &Self, v: &Self) -> Self {
        Self::cross_product_internal(u, v)
    }

    #[inline(always)]
    fn cross_product_internal(u: &Self, v: &Self) -> Self {
        Self::from([
            &(&u[1] * &v[2]) - &(&u[2] * &v[1]),
            &(&u[2] * &v[0]) - &(&u[0] * &v[2]),
            &(&u[0] * &v[1]) - &(&u[1] * &v[0]),
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::{complex::cpl, error::CrossProductError, Vector};

    #[test]
    fn example() {
        {
            let u = Vector::from([0., 0., 1.]);
            let v = Vector::from([1., 0., 0.]);
            let res = Vector::cross_product(&u, &v).unwrap();
            assert_eq!(res, [0., 1., 0.]);
            println!("cross({u}, {v}) = {res}");
        }
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            let res = Vector::cross_product(&u, &v).unwrap();
            assert_eq!(res, [-3., 6., -3.]);
            println!("cross({u}, {v}) = {res}");
        }
        {
            let u = Vector::from([4., 2., -3.]);
            let v = Vector::from([-2., -5., 16.]);
            let res = Vector::cross_product(&u, &v).unwrap();
            assert_eq!(res, [17., -58., -16.]);
            println!("cross({u}, {v}) = {res}");
        }
    }

    #[test]
    fn errors() {
        let u = Vector::from([2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        let res = Vector::cross_product(&u, &v);
        assert_eq!(
            res,
            Err(CrossProductError::LeftVectorShouldBeThreeDimensional)
        );
        let u = Vector::from([2., -3., 16.]);
        let v = Vector::from([-2., -5., 16., 5.]);
        let res = Vector::cross_product(&u, &v);
        assert_eq!(
            res,
            Err(CrossProductError::RightVectorShouldBeThreeDimensional)
        );
    }

    #[test]
    fn with_complex() {
        let u = Vector::from([cpl!(5. + 2. i), cpl!(3. - 4. i), cpl!(0. + 7. i)]);
        let v = Vector::from([cpl!(8. + 4. i), cpl!(5., 0.), cpl!(-4. - 7. i)]);
        let res = Vector::cross_product(&u, &v).unwrap();
        assert_eq!(
            res,
            [cpl!(-40. - 40. i), cpl!(-22. + 99. i), cpl!(-15. + 30. i)]
        );
    }
}
