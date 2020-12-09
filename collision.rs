use crate::geometry::base::{Point, Vector};
use crate::geometry::shape::shape::*;
use crate::geometry::shape::Polygon;

pub fn are_close(shape_a: &dyn Shape, shape_b: &dyn Shape, margin: f32) -> bool {
    shape_a.center().distance_to(shape_b.center()) + margin
        < shape_a.enclosing_radius() + shape_b.enclosing_radius()
}
pub fn distance_closest_points(shape_a: &dyn Shape, shape_b: &dyn Shape) -> (f32, Point, Point) {
    // TODO: optimize
    let poly_a = shape_a.to_polygon();
    let poly_b = shape_b.to_polygon();
    let mut best = std::f32::INFINITY;
    let mut best_point_a = Point::zero();
    let mut best_point_b = Point::zero();
    for &point in poly_a.vertices.iter() {
        let closest = shape_b.closest_point(point);
        let candidate = closest.distance_to(point);
        if candidate < best {
            best = candidate;
            best_point_a = point;
            best_point_b = closest;
        };
    }
    for &point in poly_b.vertices.iter() {
        let closest = shape_a.closest_point(point);
        let candidate = closest.distance_to(point);
        if candidate < best {
            best = candidate;
            best_point_a = closest;
            best_point_b = point;
        };
    }
    (best, best_point_a, best_point_b)
}

pub fn ball_bounce(ball: &dyn Shape, ball_direction: Vector, object: &dyn Shape) -> Option<Vector> {
    assert_eq!(
        ball.get_type(),
        ShapeType::Circle,
        "Expected ball to be somewhat circular"
    );

    let contact = object.contact_point(ball.center(), ball_direction);
    if let Some(point) = contact {
        if let Some(vector) = object.get_normal_vector_at(point) {
            return Some(vector);
        }
    }
    None
}
