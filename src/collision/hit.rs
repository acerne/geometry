use crate::base::*;
use crate::collision::*;
use crate::shape::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hit {
    pub contact: Point,
    pub normal: Vector,
    pub delta: Vector,
    pub time: f32,
}

impl Hit {
    pub fn new(contact: Point, normal: Vector, delta: Vector) -> Self {
        Self {
            contact,
            normal,
            delta,
            time: 0.0,
        }
    }
    pub fn new_time(contact: Point, normal: Vector, delta: Vector, time: f32) -> Self {
        Self {
            contact,
            normal,
            delta,
            time,
        }
    }
    pub fn zero() -> Self {
        Self {
            contact: Point::zero(),
            normal: Vector::zero(),
            delta: Vector::zero(),
            time: 0.0,
        }
    }
}
