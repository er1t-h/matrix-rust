#[derive(Debug, PartialEq, Clone, Copy, Default, PartialOrd)]
pub struct Degree(pub f32);
#[derive(Debug, PartialEq, Clone, Copy, Default, PartialOrd)]
pub struct Radian(pub f32);

impl From<Radian> for Degree {
    fn from(value: Radian) -> Self {
        Self(value.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degree> for Radian {
    fn from(value: Degree) -> Self {
        Self(value.0 * std::f32::consts::PI / 180.0)
    }
}

impl Radian {
    pub fn sin(self) -> f32 {
        self.0.sin()
    }

    pub fn cos(self) -> f32 {
        self.0.cos()
    }

    pub fn sin_cos(self) -> (f32, f32) {
        self.0.sin_cos()
    }
}

impl Degree {
    pub fn sin(self) -> f32 {
        Radian::from(self).sin()
    }

    pub fn cos(self) -> f32 {
        Radian::from(self).cos()
    }

    pub fn sin_cos(self) -> (f32, f32) {
        Radian::from(self).sin_cos()
    }
}
