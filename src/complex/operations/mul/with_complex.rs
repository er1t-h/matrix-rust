use std::ops::{AddAssign, Mul, MulAssign, SubAssign};

use crate::Complex;

impl<T> Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_mul_assign(&mut self, rhs: &Self) {
        let mut tmp_1a = self.real.clone();
        let mut tmp_1b = self.imaginary.clone();
        tmp_1a *= &rhs.real;
        tmp_1b *= &rhs.imaginary;
        tmp_1a -= &tmp_1b;

        let mut tmp_2a = self.real.clone();
        let mut tmp_2b = self.imaginary.clone();
        tmp_2a *= &rhs.imaginary;
        tmp_2b *= &rhs.real;
        tmp_2a += &tmp_2b;

        (self.real, self.imaginary) = (tmp_1a, tmp_2a);
    }
}
impl<T> Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_mul(&self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.default_mul_assign(rhs);
        tmp
    }
}

impl<T> MulAssign<&Self> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.default_mul_assign(rhs);
    }
}

impl<T> MulAssign<Self> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.default_mul_assign(&rhs);
    }
}

impl<T> Mul<Self> for &Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.default_mul(rhs)
    }
}

impl<T> Mul<Complex<T>> for &Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        self.default_mul(&rhs)
    }
}

impl<T> Mul<&Complex<T>> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn mul(self, rhs: &Complex<T>) -> Self::Output {
        self.default_mul(rhs)
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        self.default_mul(&rhs)
    }
}

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
        assert_eq!(nb, Complex::new(18496, 0))
    }
}
