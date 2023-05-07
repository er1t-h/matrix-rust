use std::ops::{AddAssign, MulAssign, SubAssign};

use crate::error::LinearInterpolationError;

///
/// Returns the linear interpolation between 2 numbers
///
/// # Errors
/// Returns a [`RatioOffBound`](LinearInterpolationError::RatioOffBound) if ratio is not between 0 and 1
///
pub fn lerp<V, Ratio>(u: &V, v: &V, ratio: &Ratio) -> Result<V, LinearInterpolationError>
where
    Ratio: BetweenZeroAndOne,
    for<'a> V: Clone + AddAssign<&'a V> + SubAssign<&'a V> + MulAssign<&'a Ratio>,
{
    if ratio.is_between_zero_and_one() {
        let mut accumulator = v.clone();
        accumulator -= u;
        accumulator *= ratio;
        accumulator += u;
        Ok(accumulator)
    } else {
        Err(LinearInterpolationError::RatioOffBound)
    }
}

pub trait BetweenZeroAndOne {
    fn is_between_zero_and_one(&self) -> bool;
}

impl BetweenZeroAndOne for f32 {
    fn is_between_zero_and_one(&self) -> bool {
        (0. ..=1.).contains(self)
    }
}

impl BetweenZeroAndOne for f64 {
    fn is_between_zero_and_one(&self) -> bool {
        (0. ..=1.).contains(self)
    }
}

#[cfg(test)]
mod test {
    use crate::complex::cpl;
    use crate::utils::ex02::lerp;
    use crate::{error::LinearInterpolationError, Matrix, Vector};
    use pretty_assertions::assert_eq;

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

    #[test]
    fn complex() {
        let res = lerp(&cpl!(1., 3.), &cpl!(0., 0.), &0.5_f64).unwrap();
        assert_eq!(res, cpl!(0.5, 1.5));
        let res = lerp(&cpl!(1., 3.), &cpl!(0., 0.), &0.).unwrap();
        assert_eq!(res, cpl!(1., 3.));
        let res = lerp(&cpl!(1., 3.), &cpl!(0., 0.), &1.).unwrap();
        assert_eq!(res, cpl!(0., 0.));
    }
}
