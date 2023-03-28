use std::ops::{AddAssign, MulAssign, SubAssign};

use crate::error::LinearInterpolationError;

///
/// Returns the linear interpolation of `u` and `v` with ration `t`.
///
/// If the ratio is bigger than `1` or less than `0`, returns a `LinearInterpolationError`
/// See [lerp_unchecked] if your input is already verified.
///
/// # Example
/// ```
/// use matrix::utils::lerp;
/// use matrix::error::LinearInterpolationError;
///
/// let res = lerp(&1.0, &2.0, &1.01);
/// assert_eq!(res, Err(LinearInterpolationError::RatioOffBound));
/// ```
///
/// # Complexity:
/// Same as the complexity of `+=`, `-=` and `*=` for `V`
///
pub fn lerp<V, Ratio>(u: &V, v: &V, ratio: &Ratio) -> Result<V, LinearInterpolationError>
where
    Ratio: Into<f64> + Clone,
    V: Clone,
    for<'a> V: AddAssign<&'a V> + SubAssign<&'a V> + MulAssign<&'a Ratio>,
{
    let bounds: f64 = ratio.clone().into();
    if !(0. ..=1.).contains(&bounds) {
        Err(LinearInterpolationError::RatioOffBound)
    } else {
        Ok(lerp_internal(u, v, ratio))
    }
}

///
/// Returns the linear interpolation of `u` and `v` with ration `t`.
///
/// This function allows you to use ratios above 1 or below 0. For a safe
/// alternative, use [lerp]
///
/// # Safety
/// Make sure that the ration is not above 1 or below 0.
///
/// # Example
/// ```
/// use matrix::utils::lerp_unchecked;
/// use matrix::error::LinearInterpolationError;
///
/// let res = unsafe { lerp_unchecked(&1.0, &2.0, &0.5) };
/// assert_eq!(res, 1.5);
/// ```
///
/// # Complexity
/// Same as the complexity of `+=`, `-=` and `*=` for `V`
///
pub unsafe fn lerp_unchecked<V, Ratio>(u: &V, v: &V, ratio: &Ratio) -> V
where
    V: Clone,
    for<'a> V: AddAssign<&'a V> + SubAssign<&'a V> + MulAssign<&'a Ratio>,
{
    lerp_internal(u, v, ratio)
}

#[inline(always)]
fn lerp_internal<V, Ratio>(u: &V, v: &V, ratio: &Ratio) -> V
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

    use super::lerp;
    use crate::{error::LinearInterpolationError, Matrix, Vector};

    #[test]
    fn example() {
        {
            let res = lerp(&0., &1., &0.).unwrap();
            assert_eq!(res, 0.);
            println!("{}", res);
        }
        {
            let res = lerp(&0., &1., &1.).unwrap();
            assert_eq!(res, 1.);
            println!("{}", res);
        }
        {
            let res = lerp(&0., &1., &0.5).unwrap();
            assert_eq!(res, 0.5);
            println!("{}", res);
        }
        {
            let res = lerp(&21., &42., &0.3).unwrap();
            assert_eq!(res, 27.3);
            println!("{}", res);
        }
        {
            let res = lerp(&Vector::from([2., 1.]), &Vector::from([4., 2.]), &0.3).unwrap();
            assert_eq!(res, [2.6, 1.3]);
            println!("{}", res);
        }
        {
            let res = lerp(
                &Matrix::from([[2., 1.], [3., 4.]]),
                &Matrix::from([[20., 10.], [30., 40.]]),
                &0.5,
            )
            .unwrap();
            assert_eq!(res, [[11., 5.5], [16.5, 22.]]);
            println!("{}", res);
        }
    }

    #[test]
    fn errors() {
        {
            let res = lerp(&1.0, &2.0, &1.01);
            assert_eq!(res, Err(LinearInterpolationError::RatioOffBound))
        }
        {
            let res = lerp(&1.0, &2.0, &-0.01);
            assert_eq!(res, Err(LinearInterpolationError::RatioOffBound))
        }
    }
}
