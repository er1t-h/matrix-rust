use std::ops::{Mul, MulAssign};

use crate::Complex;

impl<T> Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_mul_assign_t(&mut self, rhs: &T) {
        self.real *= rhs;
        self.imaginary *= rhs;
    }
}
impl<T> Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_mul_t(&self, rhs: &T) -> Self {
        let mut tmp = self.clone();
        tmp.default_mul_assign_t(rhs);
        tmp
    }
}

impl<T> MulAssign<&T> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    fn mul_assign(&mut self, rhs: &T) {
        self.default_mul_assign_t(rhs);
    }
}

impl<T> MulAssign<T> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.default_mul_assign_t(&rhs);
    }
}

impl<T> Mul<&T> for &Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn mul(self, rhs: &T) -> Self::Output {
        self.default_mul_t(rhs)
    }
}

impl<T> Mul<T> for &Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.default_mul_t(&rhs)
    }
}

impl<T> Mul<&T> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    type Output = Self;
    fn mul(self, rhs: &T) -> Self::Output {
        self.default_mul_t(rhs)
    }
}

impl<T> Mul<T> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + Clone,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.default_mul_t(&rhs)
    }
}

#[allow(clippy::op_ref, clippy::misrefactored_assign_op)]
#[cfg(test)]
mod test {
    use crate::Complex;

    #[test]
    fn mul() {
        let mut nb = Complex::new(10, 6);
        assert_eq!(nb * nb, Complex::new(64, 120));
        let other = Complex::new(-10, 6);
        assert_eq!(&nb * &other, Complex::new(-136, 0));
        assert_eq!(nb * &other, Complex::new(-136, 0));
        assert_eq!(&nb * other, Complex::new(-136, 0));
        nb *= other * nb;
        nb *= &other;
        assert_eq!(nb, Complex::new(18496, 0));
    }
}
