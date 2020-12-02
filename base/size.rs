use crate::geometry::base::Scale;
use float_eq::FloatEq;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
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

impl FloatEq for Size {
    type Epsilon = f32;

    fn eq_abs(&self, other: &Self, max_diff: &f32) -> bool {
        self.w.eq_abs(&other.w, max_diff) && self.h.eq_abs(&other.h, max_diff)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &f32) -> bool {
        self.w.eq_rmax(&other.w, max_diff) && self.h.eq_rmax(&other.h, max_diff)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &f32) -> bool {
        self.w.eq_rmin(&other.w, max_diff) && self.h.eq_rmin(&other.h, max_diff)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &f32) -> bool {
        self.w.eq_r1st(&other.w, max_diff) && self.h.eq_r1st(&other.h, max_diff)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &f32) -> bool {
        self.w.eq_r2nd(&other.w, max_diff) && self.h.eq_r2nd(&other.h, max_diff)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &float_eq::UlpsEpsilon<f32>) -> bool {
        self.w.eq_ulps(&other.w, max_diff) && self.h.eq_ulps(&other.h, max_diff)
    }
}
