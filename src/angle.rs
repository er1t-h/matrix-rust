use std::ops::Mul;

#[derive(Debug, PartialEq, Clone, Copy, Default, PartialOrd)]
pub struct Degree(pub f32);
#[derive(Debug, PartialEq, Clone, Copy, Default, PartialOrd)]
pub struct Radian(pub f32);

impl From<Radian> for Degree {
    fn from(value: Radian) -> Self {
        Self(value.0.to_degrees())
    }
}

impl From<Degree> for Radian {
    fn from(value: Degree) -> Self {
        Self(value.0.to_radians())
    }
}

impl Radian {
    #[must_use]
    pub fn sin(self) -> f32 {
        self.0.sin()
    }

    #[must_use]
    pub fn cos(self) -> f32 {
        self.0.cos()
    }

    #[must_use]
    pub fn sin_cos(self) -> (f32, f32) {
        self.0.sin_cos()
    }
}

impl Degree {
    #[must_use]
    pub fn sin(self) -> f32 {
        Radian::from(self).sin()
    }

    #[must_use]
    pub fn cos(self) -> f32 {
        Radian::from(self).cos()
    }

    #[must_use]
    pub fn sin_cos(self) -> (f32, f32) {
        Radian::from(self).sin_cos()
    }
}

impl Mul<f32> for Degree {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
