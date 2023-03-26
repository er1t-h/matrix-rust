use std::ops::{AddAssign, MulAssign, SubAssign};

use crate::error::LinearInterpolationError;

///
/// Returns the linear interpolation of `u` and `v` with ration `t`.
///
/// If the ratio is bigger than `1` or less than `0`, returns a `LinearInterpolationError`
/// See [lerp] if your input is already verified.
///
/// # Example
/// ```
/// use matrix::utils::safe_lerp;
/// use matrix::error::LinearInterpolationError;
///
/// let res = safe_lerp(&1.0, &2.0, &1.01);
/// assert_eq!(res, Err(LinearInterpolationError::RatioOffBound));
/// ```
///
/// # Complexity:
/// Same as the complexity of `+=`, `-=` and `*=` for `V`
///
pub fn safe_lerp<V, Ratio>(u: &V, v: &V, ratio: &Ratio) -> Result<V, LinearInterpolationError>
where
    Ratio: Into<f64> + Clone,
    V: Clone,
    for<'a> V: AddAssign<&'a V> + SubAssign<&'a V> + MulAssign<&'a Ratio>,
{
    let bounds: f64 = ratio.clone().into();
    if bounds > 1. || bounds < 0. {
        Err(LinearInterpolationError::RatioOffBound)
    } else {
        Ok(lerp(u, v, ratio))
    }
}

///
/// Returns the linear interpolation of `u` and `v` with ration `t`.
///
/// This function allows you to use ratios above 1 or below 0. For a safe
/// alternative, use [safe_lerp]
///
/// # Example
/// ```
/// use matrix::utils::safe_lerp;
/// use matrix::error::LinearInterpolationError;
///
/// let res = safe_lerp(&1.0, &2.0, &1.01);
/// assert_eq!(res, Err(LinearInterpolationError::RatioOffBound));
/// ```
///
/// # Complexity:
/// Same as the complexity of `+=`, `-=` and `*=` for `V`
///
pub fn lerp<V, Ratio>(u: &V, v: &V, ratio: &Ratio) -> V
where
    V: Clone,
    for<'a> V: AddAssign<&'a V> + SubAssign<&'a V> + MulAssign<&'a Ratio>,
{
    let mut accumulator = v.clone();
    accumulator -= u;
    accumulator *= ratio;
    accumulator += u;
    accumulator
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::{lerp, safe_lerp};
    use crate::{error::LinearInterpolationError, Matrix, Vector};

    #[test]
    fn example() {
        {
            let res = lerp(&0., &1., &0.);
            assert_eq!(res, 0.);
            println!("{}", res);
        }
        {
            let res = lerp(&0., &1., &1.);
            assert_eq!(res, 1.);
            println!("{}", res);
        }
        {
            let res = lerp(&0., &1., &0.5);
            assert_eq!(res, 0.5);
            println!("{}", res);
        }
        {
            let res = lerp(&21., &42., &0.3);
            assert_eq!(res, 27.3);
            println!("{}", res);
        }
        {
            let res = lerp(&Vector::from([2., 1.]), &Vector::from([4., 2.]), &0.3);
            assert_eq!(res, [2.6, 1.3]);
            println!("{}", res);
        }
        {
            let res = lerp(
                &Matrix::from([[2., 1.], [3., 4.]]),
                &Matrix::from([[20., 10.], [30., 40.]]),
                &0.5,
            );
            assert_eq!(res, [[11., 5.5], [16.5, 22.]]);
            println!("{}", res);
        }
    }

    #[test]
    fn errors() {
        {
            let res = safe_lerp(&1.0, &2.0, &1.01);
            assert_eq!(res, Err(LinearInterpolationError::RatioOffBound))
        }
        {
            let res = safe_lerp(&1.0, &2.0, &-0.01);
            assert_eq!(res, Err(LinearInterpolationError::RatioOffBound))
        }
    }
}
