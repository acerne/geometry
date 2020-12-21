use crate::base::*;
use crate::collision::*;
use crate::shape::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hit {
    pub position: Point,
    pub normal: Vector,
    pub delta: Vector,
    pub time: f32,
}

impl Hit {
    pub fn new(position: Point, normal: Vector, delta: Vector) -> Self {
        Self {
            position,
            normal,
            delta,
            time: 0.0,
        }
    }
    pub fn new_time(position: Point, normal: Vector, delta: Vector, time: f32) -> Self {
        Self {
            position,
            normal,
            delta,
            time,
        }
    }
    pub fn zero() -> Self {
        Self {
            position: Point::zero(),
            normal: Vector::zero(),
            delta: Vector::zero(),
            time: 0.0,
        }
    }
}
