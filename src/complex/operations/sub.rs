use std::ops::{Sub, SubAssign};

use crate::Complex;

impl<T> Complex<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    #[inline(always)]
    fn default_sub_assign(&mut self, rhs: &Self) {
        self.real -= &rhs.real;
        self.imaginary -= &rhs.imaginary;
    }
}
impl<T> Complex<T>
where
    for<'a> T: SubAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_sub(&self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.default_sub_assign(rhs);
        tmp
    }
}

impl<T> SubAssign<&Self> for Complex<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.default_sub_assign(rhs);
    }
}

impl<T> SubAssign<Self> for Complex<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.default_sub_assign(&rhs);
    }
}

impl<T> Sub<Self> for &Complex<T>
where
    for<'a> T: SubAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.default_sub(rhs)
    }
}

impl<T> Sub<Complex<T>> for &Complex<T>
where
    for<'a> T: SubAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Self::Output {
        self.default_sub(&rhs)
    }
}

impl<T> Sub<&Self> for Complex<T>
where
    for<'a> T: SubAssign<&'a T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        self.default_sub(rhs)
    }
}

impl<T> Sub<Self> for Complex<T>
where
    for<'a> T: SubAssign<&'a T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.default_sub(&rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::Complex;

    #[test]
    fn sub() {
        let mut nb = Complex::new(10, 6);
        assert_eq!(nb - nb, Complex::new(0, 0));
        let other = Complex::new(-10, 6);
        assert_eq!(&nb - &other, Complex::new(20, 0));
        assert_eq!(nb - &other, Complex::new(20, 0));
        assert_eq!(&nb - other, Complex::new(20, 0));
        nb -= other - nb;
        nb -= &other;
        assert_eq!(nb, Complex::new(40, 0))
    }
}
