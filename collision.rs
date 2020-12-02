use crate::geometry::base::{Point, Vector};
use crate::geometry::shape::shape::Shape;
use crate::geometry::shape::Polygon;

pub fn are_close(shape_a: &dyn Shape, shape_b: &dyn Shape, margin: f32) -> bool {
    shape_a.center().distance_to(shape_b.center()) + margin
        < shape_a.enclosing_radius() + shape_b.enclosing_radius()
}
pub fn distance(shape_a: &dyn Shape, shape_b: &dyn Shape) -> f32 {
    // TODO: optimize
    let poly_a = shape_a.to_polygon();
    let poly_b = shape_b.to_polygon();
    let mut best = std::f32::INFINITY;
    for &point in poly_a.vertices.iter() {
        let closest = shape_b.closest_point(point);
        let candidate = closest.distance_to(point);
        if candidate < best {
            best = candidate;
        };
    }
    for &point in poly_b.vertices.iter() {
        let closest = shape_a.closest_point(point);
        let candidate = closest.distance_to(point);
        if candidate < best {
            best = candidate;
        };
    }
    best
}
