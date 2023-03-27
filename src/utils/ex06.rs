use std::ops::{Mul, Sub};

use crate::{error::CrossProductError, Vector};

///
/// Computes the cross product of two vectors.
///
/// # Panics:
/// Never.
///
/// # Note:
/// You can use [cross_product] if your inputs are already checked.
///
/// # Example:
/// ```
/// use matrix::utils::safe_cross_product;
/// use matrix::Vector;
/// use matrix::error::CrossProductError;
///
/// let v1 = Vector::from([1, 0, 0]);
/// let v2 = Vector::from([0, 1, 0, 2]);
/// assert_eq!(safe_cross_product(&v1, &v2), Err(CrossProductError::RightVectorShouldBeThreeDimensional));
/// ```
///
/// # Complexity:
/// Constant
///
pub fn safe_cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Result<Vector<K>, CrossProductError>
where
    K: Clone,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K>,
{
    if u.len() != 3 {
        Err(CrossProductError::LeftVectorShouldBeThreeDimensional)
    } else if v.len() != 3 {
        Err(CrossProductError::RightVectorShouldBeThreeDimensional)
    } else {
        Ok(cross_product_internal(u, v))
    }
}

///
/// Computes the cross product of two vectors.
///
/// # Panics:
/// If one of the two vectors is not three dimensional, the behaviour is undefined.
/// Please use [safe_cross_product] if you're not sure of your inputs.
///
/// # Example:
/// ```
/// use matrix::utils::cross_product;
/// use matrix::Vector;
///
/// let v1 = Vector::from([1, 0, 0]);
/// let v2 = Vector::from([0, 1, 0]);
/// assert_eq!(cross_product(&v1, &v2), [0, 0, 1]);
/// ```
///
/// # Complexity:
/// Constant
///
pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Clone,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K>,
{
    cross_product_internal(u, v)
}

#[inline(always)]
fn cross_product_internal<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Clone,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K>,
{
    Vector::from([
        &(&u[1] * &v[2]) - &(&u[2] * &v[1]),
        &(&u[2] * &v[0]) - &(&u[0] * &v[2]),
        &(&u[0] * &v[1]) - &(&u[1] * &v[0]),
    ])
}

#[cfg(test)]
mod test {
    use crate::{error::CrossProductError, utils::safe_cross_product, Vector};

    #[test]
    fn example() {
        {
            let u = Vector::from([0., 0., 1.]);
            let v = Vector::from([1., 0., 0.]);
            let res = safe_cross_product(&u, &v).unwrap();
            assert_eq!(res, [0., 1., 0.]);
            println!("cross({}, {}) = {}", u, v, res);
        }
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            let res = safe_cross_product(&u, &v).unwrap();
            assert_eq!(res, [-3., 6., -3.]);
            println!("cross({}, {}) = {}", u, v, res);
        }
        {
            let u = Vector::from([4., 2., -3.]);
            let v = Vector::from([-2., -5., 16.]);
            let res = safe_cross_product(&u, &v).unwrap();
            assert_eq!(res, [17., -58., -16.]);
            println!("cross({}, {}) = {}", u, v, res);
        }
    }

    #[test]
    fn errors() {
        let u = Vector::from([2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        let res = safe_cross_product(&u, &v);
        assert_eq!(
            res,
            Err(CrossProductError::LeftVectorShouldBeThreeDimensional)
        );
        let u = Vector::from([2., -3., 16.]);
        let v = Vector::from([-2., -5., 16., 5.]);
        let res = safe_cross_product(&u, &v);
        assert_eq!(
            res,
            Err(CrossProductError::RightVectorShouldBeThreeDimensional)
        );
    }
}
