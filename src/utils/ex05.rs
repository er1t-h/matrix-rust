use std::{
    iter::Sum,
    ops::{Div, Mul},
};

use crate::{
    error::VectorOperationError,
    traits::{Divisor, Sqrt},
    Vector,
};

///
/// Returns the cosine of the angle formed by the two vectors.
///
/// Consider using [angle_cos] if you're sure that your inputs are correct.
///
/// # Example:
/// ```
/// use matrix::utils::safe_angle_cos;
/// use matrix::Vector;
/// use matrix::error::VectorOperationError;
///
/// let v1 = Vector::from([1.0, 0.0]);
/// let v2 = Vector::from([0.0, 0.0]);
/// assert_eq!(safe_angle_cos(&v1, &v2), Err(VectorOperationError::ZeroVector));
/// ```
///
/// # Complexity:
/// Linear: O(n) with `n` the total number of coordinates
///
pub fn safe_angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> Result<K, VectorOperationError>
where
    K: Clone + Sum + Sqrt + Divisor + Mul<Output = K> + Div<Output = K> + PartialEq<K>,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    if u.len() != v.len() {
        return Err(VectorOperationError::NotSameSize(u.len(), v.len()));
    }
    let u_norm = u.norm();
    if !u_norm.can_be_divisor() {
        return Err(VectorOperationError::ZeroVector);
    }
    let v_norm = v.norm();
    if !v_norm.can_be_divisor() {
        return Err(VectorOperationError::ZeroVector);
    }
    Ok(u.dot(v) / (u_norm * v_norm))
}

///
/// Returns the cosine of the angle formed by the two vectors.
///
/// Consider using [safe_angle_cos] if you're not sure that your inputs are
/// correct, since a zero vector or size difference between the vectors is
/// undefined behaviour.
///
/// # Example:
/// ```
/// use matrix::utils::angle_cos;
/// use matrix::Vector;
/// use matrix::error::VectorOperationError;
///
/// let v1 = Vector::from([1.0, 0.0]);
/// let v2 = Vector::from([1.0, 0.0]);
/// assert_eq!(angle_cos(&v1, &v2), 1.0);
/// ```
///
/// # Complexity:
/// Linear: O(n) with `n` the total number of coordinates
///
pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> K
where
    K: Clone + Sum + Sqrt + Mul<Output = K> + Div<Output = K>,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    let tmp = u.dot(v);
    tmp / (u.norm() * v.norm())
}

#[cfg(test)]
mod test {
    use crate::error::VectorOperationError;
    use crate::{assert_eq_float, Vector};

    use crate::utils::ex05::safe_angle_cos;

    #[test]
    fn example() {
        {
            let u = Vector::from([1., 0.]);
            let v = Vector::from([1., 0.]);
            let res = safe_angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 1.0);
            println!("cos({}, {}) = {}", u, v, res);
            // 1.0
        }
        {
            let u = Vector::from([1., 0.]);
            let v = Vector::from([0., 1.]);
            let res = safe_angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 0.0);
            println!("cos({}, {}) = {}", u, v, res);
            // 0.0
        }
        {
            let u = Vector::from([-1., 1.]);
            let v = Vector::from([1., -1.]);
            let res = safe_angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, -1.0);
            println!("cos({}, {}) = {}", u, v, res);
            // -1.0
        }
        {
            let u = Vector::from([2., 1.]);
            let v = Vector::from([4., 2.]);
            let res = safe_angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 1.0);
            println!("cos({}, {}) = {}", u, v, res);
            // 1.0
        }
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            let res = safe_angle_cos(&u, &v).unwrap();
            assert_eq_float!(res, 0.974631846);
            println!("cos({}, {}) = {}", u, v, res);
            // 0.974631846
        }
    }

    #[test]
    fn errors() {
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5.]);
            assert_eq!(
                safe_angle_cos(&u, &v),
                Err(VectorOperationError::NotSameSize(3, 2))
            );
        }
        {
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([0., 0., 0.]);
            assert_eq!(
                safe_angle_cos(&u, &v),
                Err(VectorOperationError::ZeroVector)
            );
        }
    }
}
