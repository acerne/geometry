use crate::base::{Angle, Line, Point, Vector};
use crate::collision::BoundingBox;
use crate::shape::Polygon;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeType {
    Circle,
    Rectangle,
    Hexagon,
}

pub trait Shape {
    fn get_type(&self) -> ShapeType;
    fn center(&self) -> Point;
    fn enclosing_radius(&self) -> f32;
    fn translate(&mut self, vector: Vector);
    fn move_to(&mut self, point: Point);
    fn rotate(&mut self, theta: Angle);
    fn rotate_to(&mut self, phi: Angle);
    fn rotate_about(&mut self, point: Point, theta: Angle);
    fn polygon(&self) -> Polygon;
    fn bounding_box(&self) -> BoundingBox;
    fn closest_point(&self, point: Point) -> Point;
    fn contact_point(&self, origin: Point, direction: Vector) -> Option<Point> {
        let extended = direction.get_unit_vector()
            * (origin.distance_to(self.center()) + self.enclosing_radius());
        let line = Line::from_vector(origin, extended);
        let (ia, ib) = line.intersection_polygon(&self.polygon());
        if let Some(intersection_a) = ia {
            if let Some(intersection_b) = ib {
                // Two intersections
                if origin.distance_to(intersection_a) < origin.distance_to(intersection_b) {
                    return Some(intersection_a);
                } else {
                    return Some(intersection_b);
                }
            } else {
                return Some(intersection_a);
            }
        }
        None
    }
    fn get_normal_vector_at(&self, point: Point) -> Option<Vector> {
        let sides = self.polygon().to_lines();
        for side in sides.iter() {
            if side.is_on_line(point) {
                return Some(side.to_vector().get_normal_vector().get_unit_vector());
            }
        }
        None
    }
    //fn to_enclosing_circle(&self) -> Circle;
    //fn center_of_gravity(&self) -> Point
}
