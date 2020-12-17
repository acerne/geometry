use crate::base::*;
use crate::collision::*;
use crate::shape::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hit {
    pub position: Point,
    pub normal: Vector,
    pub delta: Vector,
}

impl Hit {
    pub fn new(position: Point, normal: Vector, delta: Vector) -> Self {
        Self {
            position,
            normal,
            delta,
        }
    }
    pub fn zero() -> Self {
        Self {
            position: Point::zero(),
            normal: Vector::zero(),
            delta: Vector::zero(),
        }
    }
}
