use std::fmt::Display;

use super::Complex;

impl<T> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Self {
        Complex { real, imaginary }
    }
}

impl<T> From<T> for Complex<T>
where
    T: Default,
{
    fn from(nb: T) -> Self {
        Self {
            real: nb,
            imaginary: T::default(),
        }
    }
}

impl<T> Display for Complex<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}i", &self.real, &self.imaginary)
    }
}
