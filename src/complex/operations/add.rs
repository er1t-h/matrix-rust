use std::ops::{Add, AddAssign};

use crate::Complex;

impl<T> Complex<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    #[inline(always)]
    fn default_add_assign(&mut self, rhs: &Self) {
        self.real += &rhs.real;
        self.imaginary += &rhs.imaginary;
    }
}
impl<T> Complex<T>
where
    for<'a> T: AddAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_add(&self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.default_add_assign(rhs);
        tmp
    }
}

impl<T> AddAssign<&Self> for Complex<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.default_add_assign(rhs);
    }
}

impl<T> AddAssign<Self> for Complex<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.default_add_assign(&rhs);
    }
}

impl<T> Add<Self> for &Complex<T>
where
    for<'a> T: AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        self.default_add(rhs)
    }
}

impl<T> Add<Complex<T>> for &Complex<T>
where
    for<'a> T: AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        self.default_add(&rhs)
    }
}

impl<T> Add<&Self> for Complex<T>
where
    for<'a> T: AddAssign<&'a T> + Clone,
{
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        self.default_add(rhs)
    }
}

impl<T> Add<Self> for Complex<T>
where
    for<'a> T: AddAssign<&'a T> + Clone,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.default_add(&rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::Complex;

    #[test]
    fn add() {
        let mut nb = Complex::new(10, 6);
        assert_eq!(nb + nb, Complex::new(20, 12));
        let other = Complex::new(-10, 6);
        assert_eq!(&nb + &other, Complex::new(0, 12));
        assert_eq!(nb + &other, Complex::new(0, 12));
        assert_eq!(&nb + other, Complex::new(0, 12));
        nb += other + nb;
        nb += &other;
        assert_eq!(nb, Complex::new(0, 24))
    }
}
