use std::ops::Neg;

use crate::Complex;

impl<T> Neg for Complex<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            real: -&self.real,
            imaginary: -&self.imaginary,
        }
    }
}

impl<T> Neg for &Complex<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Self::Output {
            real: -&self.real,
            imaginary: -&self.imaginary,
        }
    }
}
