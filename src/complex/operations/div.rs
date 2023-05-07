use std::ops::{AddAssign, Div, DivAssign, MulAssign, SubAssign};

use crate::Complex;

impl<T> Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    // (A + B * I) / (C + D * I) =
    // (A * C + B * D) / (C ^ 2 + D ^ 2) + ((-A * D + B * C) * i) / (C ^ 2 + D ^ 2)
    #[inline(always)]
    fn default_div_assign(&mut self, rhs: &Self) {
        let mut acbd = {
            let ac = {
                let mut tmp = self.real.clone();
                tmp *= &rhs.real;
                tmp
            };
            let bd = {
                let mut tmp = self.imaginary.clone();
                tmp *= &rhs.imaginary;
                tmp
            };
            let mut tmp = ac;
            tmp += &bd;
            tmp
        };
        let c2d2 = {
            let c2 = {
                let mut tmp = rhs.real.clone();
                tmp *= &rhs.real;
                tmp
            };
            let d2 = {
                let mut tmp = rhs.imaginary.clone();
                tmp *= &rhs.imaginary;
                tmp
            };
            let mut tmp = c2;
            tmp += &d2;
            tmp
        };
        let mut adbc = {
            let ad = {
                let mut tmp = self.real.clone();
                tmp *= &rhs.imaginary;
                tmp
            };
            let bc = {
                let mut tmp = self.imaginary.clone();
                tmp *= &rhs.real;
                tmp
            };
            let mut tmp = bc;
            tmp -= &ad;
            tmp
        };

        acbd /= &c2d2;
        adbc /= &c2d2;

        (self.real, self.imaginary) = (acbd, adbc);
    }
}

impl<T> Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    #[inline(always)]
    fn default_div(&self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.default_div_assign(rhs);
        tmp
    }
}

impl<T> DivAssign<&Self> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    fn div_assign(&mut self, rhs: &Self) {
        self.default_div_assign(rhs);
    }
}

impl<T> DivAssign<Self> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    fn div_assign(&mut self, rhs: Self) {
        self.default_div_assign(&rhs);
    }
}

impl<T> Div<Self> for &Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn div(self, rhs: Self) -> Self::Output {
        self.default_div(rhs)
    }
}

impl<T> Div<Complex<T>> for &Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Self::Output {
        self.default_div(&rhs)
    }
}

impl<T> Div<&Self> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Self;
    fn div(self, rhs: &Self) -> Self::Output {
        self.default_div(rhs)
    }
}

impl<T> Div<Self> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + DivAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T> + Clone,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.default_div(&rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::Complex;

    #[test]
    fn mul() {
        let nb = Complex::new(10.0, 6.0);
        assert_eq!(nb / nb, Complex::new(1.0, 0.0));
        let other = Complex::new(-10.0, 6.0);
        assert_eq!(&nb / &other, Complex::new(-(8.0 / 17.0), -(15.0 / 17.0)));
        assert_eq!(&nb / other, Complex::new(-(8.0 / 17.0), -(15.0 / 17.0)));
        assert_eq!(nb / &other, Complex::new(-(8.0 / 17.0), -(15.0 / 17.0)));
    }
}
