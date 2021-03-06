use crate::base::{Angle, Line, Point, Vector};
use crate::collision::BoundingBox;
pub use crate::shape::shape::*;
use crate::shape::Polygon;
use std::cell::RefCell;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f32,
    _polygon: RefCell<Option<Polygon>>,
    _bounding_box: RefCell<Option<BoundingBox>>,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Self {
            center,
            radius,
            _polygon: RefCell::new(None),
            _bounding_box: RefCell::new(None),
        }
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn is_inslide(&self, point: Point) -> bool {
        self.center.distance_to(point) < self.radius
    }
    fn invalidate(&self) {
        *self._polygon.borrow_mut() = None;
        *self._bounding_box.borrow_mut() = None;
    }
    fn create_polygon(&self) {
        // determine number of polygon vertices from radius
        let n_vertices = 4 + (4.0 * self.radius.sqrt().floor()) as usize;
        let mut vertices = Vec::new();
        vertices.reserve(n_vertices);
        let angle_step = (360.0 / n_vertices as f32).to_radians();
        for i in 0..n_vertices {
            vertices.push(
                self.center
                    + Vector::new(
                        self.radius * ((i as f32) * angle_step).cos(),
                        self.radius * ((i as f32) * angle_step).sin(),
                    ),
            )
        }
        *self._polygon.borrow_mut() = Some(Polygon { vertices });
    }
    fn create_bounding_box(&self) {
        *self._bounding_box.borrow_mut() = Some(BoundingBox::from_edges(
            self.center().x - self.radius,
            self.center().y + self.radius,
            self.center().x - self.radius,
            self.center().y + self.radius,
        ));
    }
}

impl Shape for Circle {
    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }
    fn center(&self) -> Point {
        self.center
    }
    fn enclosing_radius(&self) -> f32 {
        self.radius
    }
    fn translate(&mut self, vector: Vector) {
        self.center = self.center + vector;
        self.invalidate();
    }
    fn move_to(&mut self, point: Point) {
        self.center = point;
        self.invalidate();
    }
    fn rotate(&mut self, _theta: Angle) {
        // does nothing
    }
    fn rotate_to(&mut self, _phi: Angle) {
        // does nothing
    }
    fn rotate_about(&mut self, point: Point, theta: Angle) {
        self.center.rotate_about(point, theta);
        self.invalidate();
    }
    fn polygon(&self) -> Polygon {
        if *self._polygon.borrow() == None {
            self.create_polygon();
        }
        (*self._polygon.borrow()).clone().unwrap()
    }
    fn bounding_box(&self) -> BoundingBox {
        if *self._bounding_box.borrow() == None {
            self.create_bounding_box();
        }
        (*self._bounding_box.borrow()).clone().unwrap()
    }
    fn closest_point(&self, point: Point) -> Point {
        let v = Vector::from_points(self.center, point).get_unit_vector();
        self.center + (v * self.radius).to_point()
    }
    fn contact_point(&self, origin: Point, direction: Vector) -> Option<Point> {
        let extended =
            direction.get_unit_vector() * (origin.distance_to(self.center) + self.radius);
        let line = Line::from_vector(origin, extended);
        let (ia, ib) = line.intersection_circle(self);
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
        None // No contact
    }
    fn get_normal_vector_at(&self, point: Point) -> Option<Vector> {
        if point.distance_to(self.center) == self.radius {
            return Some(Vector::from_points(self.center, point).get_unit_vector());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::base::{Point, Vector};
    use crate::shape::{shape::Shape, Circle};

    #[test]
    fn test_translate() {
        let mut circle = Circle::new(Point::new(10.0, -5.0), 10.0);
        circle.translate(Vector::new(-2.0, 1.0));
        let expected = Circle::new(Point::new(8.0, -4.0), 10.0);
        assert_eq!(circle, expected);
    }
    #[test]
    fn test_move_to() {
        let mut circle = Circle::new(Point::new(10.0, -5.0), 10.0);
        circle.move_to(Point::new(-2.0, 1.0));
        let expected = Circle::new(Point::new(-2.0, 1.0), 10.0);
        assert_eq!(circle, expected);
    }
    #[test]
    fn test_to_polygon() {
        let circle = Circle::new(Point::new(10.0, -5.0), 10.0);
        let poly = circle.polygon();
        let length = poly.vertices.len();
        assert!(length % 4 == 0);
        let vert_rightmost = Point::new(20.0, -5.0);
        let vert_bottommost = Point::new(10.0, 5.0);
        let vert_leftmost = Point::new(0.0, -5.0);
        let vert_topmost = Point::new(10.0, -15.0);
        assert!(
            poly.vertices[0] == vert_rightmost
                && poly.vertices[length / 4] == vert_bottommost
                && poly.vertices[2 * length / 4] == vert_leftmost
                && poly.vertices[3 * length / 4] == vert_topmost,
            "{}, {}, {}, {} == {}, {}, {}, {}",
            poly.vertices[0],
            poly.vertices[length / 4],
            poly.vertices[2 * length / 4],
            poly.vertices[3 * length / 4],
            vert_rightmost,
            vert_bottommost,
            vert_leftmost,
            vert_topmost
        );
    }
    #[test]
    fn test_closest_point() {
        let circle = Circle::new(Point::new(0.0, 0.0), 10.0);
        let point = Point::new(15.0, -15.0);
        let result = circle.closest_point(point);
        let expected = Point::new(10.0 / 2f32.sqrt(), -10.0 / 2f32.sqrt());
        assert_eq!(result, expected);
    }
}
