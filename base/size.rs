use crate::geometry::base::{Scale, Vector};

#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[allow(dead_code)]
impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
    pub fn zero() -> Self {
        Self { w: 0.0, h: 0.0 }
    }
    pub fn to_vector(&self) -> Vector {
        Vector::new(self.w, self.h)
    }
    pub fn max(&self) -> f32 {
        self.w.max(self.h)
    }
    pub fn min(&self) -> f32 {
        self.w.min(self.h)
    }
}

impl std::ops::Mul<Scale> for Size {
    type Output = Self;
    fn mul(self, other: Scale) -> Self {
        Self {
            w: self.w * other.sx,
            h: self.h * other.sy,
        }
    }
}

impl std::ops::Mul<f32> for Size {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            w: self.w * other,
            h: self.h * other,
        }
    }
}

impl std::ops::Div<Scale> for Size {
    type Output = Self;
    fn div(self, other: Scale) -> Self {
        Self {
            w: self.w / other.sx,
            h: self.h / other.sy,
        }
    }
}

impl std::ops::Div<f32> for Size {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            w: self.w / other,
            h: self.h / other,
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        float_eq::FloatEq::eq_abs(&self.w, &other.w, &10e-6)
            && float_eq::FloatEq::eq_abs(&self.h, &other.h, &10e-6)
    }
}
