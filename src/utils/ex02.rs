use std::ops::{AddAssign, MulAssign, SubAssign};

use crate::{error::LinearInterpolationError, Complex, Matrix, Vector};

///
/// Trait to implement a linear interpolation between two same objects
///
pub trait LinearInterpolation<Ratio>: Sized {
    ///
    /// Returns the linear interpolation of `u` and `v` with ration `t`.
    ///
    /// If the ratio is bigger than `1` or less than `0`, returns a `LinearInterpolationError`
    /// See [lerp_unchecked] if your input is already verified.
    ///
    fn lerp(left: &Self, right: &Self, ratio: &Ratio) -> Result<Self, LinearInterpolationError>;
}

macro_rules! impl_lerp {
    ($current: ident, $($next: ident),+) => {
        impl_lerp!($current);
        impl_lerp!($($next),+);
    };
    ($current: ident) => {
        impl LinearInterpolation<Self> for $current {
            fn lerp(left: &Self, right: &Self, ratio: &Self) -> Result<Self, LinearInterpolationError> {
                if !(0. ..=1.).contains(ratio) {
                    Err(LinearInterpolationError::RatioOffBound)
                } else {
                    Ok((right - left) * ratio + left)
                }
            }
        }
    };
}
impl_lerp!(f32, f64);

macro_rules! impl_lerp_vec_mat {
    ($name: ident, $type: ident, $($next: ident),+) => {
        impl_lerp_vec_mat!($name, $type);
        impl_lerp_vec_mat!($name, $($next),+);
    };
    ($name: ident, $current: ident) => {
        impl <K> LinearInterpolation<$current> for $name<K>
        where
            K: Clone,
            for<'a> K: AddAssign<&'a K> + SubAssign<&'a K>,
            for<'a> $name<K>: MulAssign<&'a $current>
        {
            fn lerp(left: &Self, right: &Self, ratio: &$current) -> Result<Self, LinearInterpolationError> {
                if !(0. ..=1.).contains(ratio) {
                    Err(LinearInterpolationError::RatioOffBound)
                } else {
                    let mut accumulator = right.clone();
                    accumulator -= left;
                    accumulator *= ratio;
                    accumulator += left;
                    Ok(accumulator)
                }
            }
        }
    };
}
impl_lerp_vec_mat!(Vector, f32, f64);
impl_lerp_vec_mat!(Matrix, f32, f64);

impl<K> LinearInterpolation<f64> for Complex<K>
where
    K: LinearInterpolation<f64>,
{
    fn lerp(left: &Self, right: &Self, ratio: &f64) -> Result<Self, LinearInterpolationError> {
        let real = K::lerp(left.re(), right.re(), ratio)?;
        let imaginary = K::lerp(left.im(), right.im(), ratio)?;
        Ok(Self::new(real, imaginary))
    }
}

#[cfg(test)]
mod test {
    use crate::complex::cpl;
    use crate::utils::ex02::LinearInterpolation;
    use crate::Complex;
    use crate::{error::LinearInterpolationError, Matrix, Vector};
    use pretty_assertions::assert_eq;

    #[test]
    fn example() {
        {
            let res = f64::lerp(&0., &1., &0.).unwrap();
            assert_eq!(res, 0.);
            println!("{}", res);
        }
        {
            let res = f64::lerp(&0., &1., &1.).unwrap();
            assert_eq!(res, 1.);
            println!("{}", res);
        }
        {
            let res = f64::lerp(&0., &1., &0.5).unwrap();
            assert_eq!(res, 0.5);
            println!("{}", res);
        }
        {
            let res = f64::lerp(&21., &42., &0.3).unwrap();
            assert_eq!(res, 27.3);
            println!("{}", res);
        }
        {
            let res = Vector::lerp(&Vector::from([2., 1.]), &Vector::from([4., 2.]), &0.3).unwrap();
            assert_eq!(res, [2.6, 1.3]);
            println!("{}", res);
        }
        {
            let res = Matrix::lerp(
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
            let res = f64::lerp(&1.0, &2.0, &1.01);
            assert_eq!(res, Err(LinearInterpolationError::RatioOffBound))
        }
        {
            let res = f64::lerp(&1.0, &2.0, &-0.01);
            assert_eq!(res, Err(LinearInterpolationError::RatioOffBound))
        }
    }

    #[test]
    fn complex() {
        let res = Complex::lerp(&cpl!(1., 3.), &cpl!(0., 0.), &0.5).unwrap();
        assert_eq!(res, cpl!(0.5, 1.5));
        let res = Complex::lerp(&cpl!(1., 3.), &cpl!(0., 0.), &0.).unwrap();
        assert_eq!(res, cpl!(1., 3.));
        let res = Complex::lerp(&cpl!(1., 3.), &cpl!(0., 0.), &1.).unwrap();
        assert_eq!(res, cpl!(0., 0.));
    }
}
