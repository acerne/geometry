use crate::base::*;
use crate::collision::*;
use crate::shape::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Angle,
    pub length: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Angle, length: f32) -> Self {
        Self {
            origin,
            direction,
            length,
        }
    }
    pub fn vector(&self) -> Vector {
        Vector::new(
            self.direction.cos() as f32 * self.length,
            self.direction.sin() as f32 * self.length,
        )
    }
}
