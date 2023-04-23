use std::ops::{AddAssign, Mul};

use crate::{error::LinearCombinationError, Vector};

///
/// Returns the linear combination of the `vectors` with each of the `coefficients`
/// or a [LinearCombinationError] if the parameters don't allow for such operation.
/// See [linear_combination_unchecked] for more informations.
///
/// # Example:
/// ```
/// use matrix::vector::linear_combination;
/// use matrix::Vector;
/// use matrix::error::LinearCombinationError;
///
/// let vec1 = Vector::from([1, 2]);
/// let vec2 = Vector::from([1, 2, 3]);
/// assert_eq!(linear_combination(&[vec1, vec2], &[1, 2]), Err(LinearCombinationError::VectorSizeMismatch(2, 3)));
/// ```
///
/// # Complexity:
/// Linear: O(n) with `n` the total number inside the vectors
///
pub fn linear_combination<K>(
    vectors: &[Vector<K>],
    coefficients: &[K],
) -> Result<Vector<K>, LinearCombinationError>
where
    K: Clone,
    for<'a> Vector<K>: AddAssign<&'a Vector<K>>,
    for<'a> &'a Vector<K>: Mul<&'a K, Output = Vector<K>>,
{
    if coefficients.len() != vectors.len() {
        return Err(LinearCombinationError::VectorsAndCoefficientSizeDifference(
            vectors.len(),
            coefficients.len(),
        ));
    }
    if vectors.is_empty() {
        return Err(LinearCombinationError::VectorArrayIsEmpty);
    }
    let mut iter = vectors.iter();
    let first_size = iter.next().unwrap().len();
    for elt in iter {
        if first_size != elt.len() {
            return Err(LinearCombinationError::VectorSizeMismatch(
                first_size,
                elt.len(),
            ));
        }
    }
    Ok(linear_combination_internal(vectors, coefficients))
}

///
/// Returns the linear combination of the `vectors` with each of the `coefficients`
/// The linear combination is the sum of the multiplication of each vectors by a coefficient.
///
/// Using [linear_combination] returns a [Result], whereas this function will return
/// a wrong answer in case of a bad input.
///
/// # Safety
/// Make sure that `u` and `coefs` have the same size, and that each vector of
/// `u` has the same size, or a non-sensical result might be returned.
///
/// # Example:
/// ```
/// use matrix::vector::linear_combination_unchecked;
/// use matrix::Vector;
///
/// let vec1 = Vector::from([1, 2, 0]);
/// let vec2 = Vector::from([0, 1, 2]);
/// let res = unsafe { linear_combination_unchecked(&[vec1, vec2], &[1, 2]) };
/// assert_eq!(res, [1, 4, 4]);
/// ```
///
/// # Complexity:
/// Linear: O(n) with `n` the total number of coordinates inside the vectors.
///
pub unsafe fn linear_combination_unchecked<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Clone,
    for<'a> Vector<K>: AddAssign<&'a Vector<K>>,
    for<'a> &'a Vector<K>: Mul<&'a K, Output = Vector<K>>,
{
    linear_combination_internal(u, coefs)
}

#[inline(always)]
fn linear_combination_internal<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Clone,
    for<'a> Vector<K>: AddAssign<&'a Vector<K>>,
    for<'a> &'a Vector<K>: Mul<&'a K, Output = Vector<K>>,
{
    let mut return_vector;
    if let (Some(vec), Some(coef)) = (u.get(0), coefs.get(0)) {
        return_vector = vec * coef;
    } else {
        return Vector::new();
    }
    for (vec, coef) in u.iter().zip(coefs.iter()).skip(1) {
        return_vector += &(vec * coef);
    }
    return_vector
}

#[cfg(test)]
mod test {
    use crate::{complex::cpl, error::LinearCombinationError, Vector};
    use pretty_assertions::assert_eq;

    use super::linear_combination;

    #[test]
    fn example() {
        {
            let e1 = Vector::from([1., 0., 0.]);
            let e2 = Vector::from([0., 1., 0.]);
            let e3 = Vector::from([0., 0., 1.]);

            let res = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]).unwrap();
            assert_eq!(res, [10., -2., 0.5]);
            println!("{}", res);
        }
        {
            let v1 = Vector::from([1., 2., 3.]);
            let v2 = Vector::from([0., 10., -100.]);

            let res = linear_combination(&[v1, v2], &[10., -2.]).unwrap();
            assert_eq!(res, [10., 0., 230.]);
            println!("{}", res);
        }
    }

    #[test]
    fn errors() {
        {
            let e1 = Vector::from([1., 0., 0.]);
            let e2 = Vector::from([0., 1., 0.]);
            let e3 = Vector::from([0., 0., 1.]);

            let res = linear_combination(&[e1, e2, e3], &[10., 0.5]);
            assert_eq!(
                res,
                Err(LinearCombinationError::VectorsAndCoefficientSizeDifference(
                    3, 2
                ))
            );
        }
        {
            let res = linear_combination::<u32>(&[], &[]);
            assert_eq!(res, Err(LinearCombinationError::VectorArrayIsEmpty));
        }
        {
            let e1 = Vector::from([1., 0., 0.]);
            let e2 = Vector::from([0., 1., 0.]);
            let e3 = Vector::from([0., 0.]);

            let res = linear_combination(&[e1, e2, e3], &[10., 0.5, 5.2]);
            assert_eq!(res, Err(LinearCombinationError::VectorSizeMismatch(3, 2)));
        }
    }

    #[test]
    fn normal_test() {
        let e1 = Vector::from([1., 2., 0.]);
        let e2 = Vector::from([0., -1., 2.]);
        let e3 = Vector::from([1., 0., 1.]);

        let coefs = [5., 3., 6.];
        let res = linear_combination(&[e1, e2, e3], &coefs).unwrap();
        assert_eq!(res, [11., 7., 12.])
    }

    #[test]
    fn with_complex() {
        let e1 = Vector::from([cpl!(1. + 0. i), cpl!(2. + 3. i), cpl!(2. + 5. i)]);
        let e2 = Vector::from([cpl!(0., 0.), cpl!(-1. + 1. i), cpl!(0. - 2. i)]);
        let e3 = Vector::from([cpl!(1. + 10. i), cpl!(0., 0.), cpl!(-5. - 3. i)]);

        let coefs = [cpl!(1. + 0. i), cpl!(4. + 1. i), cpl!(0. + 3. i)];
        let res = linear_combination(&[e1, e2, e3], &coefs).unwrap();
        assert_eq!(
            res,
            Vector::from([cpl!(-29. + 3. i), cpl!(-3. + 6. i), cpl!(13. - 18. i)])
        );
    }
}
