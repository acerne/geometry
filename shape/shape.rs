use crate::geometry::base::{Angle, Point, Vector};
use crate::geometry::shape::Polygon;

pub trait Shape {
    fn center(&self) -> Point;
    fn enclosing_radius(&self) -> f32;
    fn translate(&mut self, vector: Vector);
    fn move_to(&mut self, point: Point);
    fn rotate(&mut self, theta: Angle);
    fn rotate_to(&mut self, phi: Angle);
    fn rotate_about(&mut self, point: Point, theta: Angle);
    fn to_polygon(&self) -> Polygon;
    fn closest_point(&self, point: Point) -> Point;
    //fn to_bounding_box(&self) -> Box;
    //fn to_enclosing_circle(&self) -> Circle;
    //fn center_of_gravity(&self) -> Point
}
