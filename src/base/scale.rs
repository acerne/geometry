#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Scale {
    pub sx: f32,
    pub sy: f32,
}

impl std::ops::Mul<Scale> for Scale {
    type Output = Self;
    fn mul(self, other: Scale) -> Self {
        Self {
            sx: self.sx * other.sx,
            sy: self.sy * other.sy,
        }
    }
}

impl std::ops::Mul<f32> for Scale {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            sx: self.sx * other,
            sy: self.sy * other,
        }
    }
}
