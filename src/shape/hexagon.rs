use crate::base::{Angle, Point, Vector};
use crate::collision::BoundingBox;
use crate::shape::{shape::*, Polygon};
use std::cell::RefCell;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Hexagon {
    center: Point,
    side: f32,
    phi: Angle,
    _polygon: RefCell<Option<Polygon>>,
    _bounding_box: RefCell<Option<BoundingBox>>,
}

impl Hexagon {
    pub fn new(center: Point, side: f32, phi: Angle) -> Self {
        Self {
            center,
            side,
            phi,
            _polygon: RefCell::new(None),
            _bounding_box: RefCell::new(None),
        }
    }
    fn invalidate(&self) {
        *self._polygon.borrow_mut() = None;
        *self._polygon.borrow_mut() = None;
    }
    fn create_polygon(&self) {
        let mut vertices = Vec::new();
        vertices.reserve(6);
        for i in 0..6 {
            let theta = self.phi + Angle::new(i as f64 * 60f64);
            let xh = self.center.x + (theta.cos() as f32) * self.side;
            let yh = self.center.y + (theta.sin() as f32) * self.side;
            vertices.push(Point::new(xh, yh));
        }
        *self._polygon.borrow_mut() = Some(Polygon { vertices });
    }
    fn create_bounding_box(&self) {
        *self._bounding_box.borrow_mut() = Some(self.polygon().to_bounding_box());
    }
}

impl Shape for Hexagon {
    fn get_type(&self) -> ShapeType {
        ShapeType::Hexagon
    }
    fn center(&self) -> Point {
        self.center
    }
    fn enclosing_radius(&self) -> f32 {
        self.side
    }
    fn translate(&mut self, vector: Vector) {
        self.center = self.center + vector;
        self.invalidate();
    }
    fn move_to(&mut self, point: Point) {
        self.center = point;
        self.invalidate();
    }
    fn rotate(&mut self, theta: Angle) {
        self.phi = self.phi + theta;
        self.invalidate();
    }
    fn rotate_to(&mut self, phi: Angle) {
        self.phi = phi;
        self.invalidate();
    }
    fn rotate_about(&mut self, point: Point, theta: Angle) {
        self.center.rotate_about(point, theta);
        self.phi = self.phi + theta;
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
            self.create_polygon();
        }
        (*self._bounding_box.borrow()).clone().unwrap()
    }
    fn closest_point(&self, point: Point) -> Point {
        let polygon = self.polygon();
        polygon.closest_point(point)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::{Angle, Point, Vector};
    use crate::shape::{shape::Shape, Hexagon};

    #[test]
    fn test_translate() {
        let mut hexagon = Hexagon::new(Point::new(10.0, -5.0), 10.0, Angle::zero());
        hexagon.translate(Vector::new(-2.0, 1.0));
        let expected = Hexagon::new(Point::new(8.0, -4.0), 10.0, Angle::zero());
        assert_eq!(hexagon, expected);
    }
    #[test]
    fn test_move_to() {
        let mut hexagon = Hexagon::new(Point::new(10.0, -5.0), 10.0, Angle::zero());
        hexagon.move_to(Point::new(-2.0, 1.0));
        let expected = Hexagon::new(Point::new(-2.0, 1.0), 10.0, Angle::zero());
        assert_eq!(hexagon, expected);
    }
    #[test]
    fn test_rotate() {
        let mut hexagon = Hexagon::new(Point::new(10.0, -5.0), 10.0, Angle::zero());
        hexagon.rotate(Angle::new(45f64));
        let expected = Hexagon::new(Point::new(10.0, -5.0), 10.0, Angle::new(45f64));
        assert_eq!(hexagon, expected);
    }
    #[test]
    fn test_to_polygon_flat_topped() {
        let hexagon = Hexagon::new(Point::new(10.0, -5.0), 2.0, Angle::zero());
        let poly = hexagon.polygon();
        let vert_a = Point::new(12.0, -5.0);
        let vert_b = Point::new(11.0, -5.0 + 3.0f32.sqrt());
        let vert_c = Point::new(9.0, -5.0 + 3.0f32.sqrt());
        let vert_d = Point::new(8.0, -5.0);
        let vert_e = Point::new(9.0, -5.0 - 3.0f32.sqrt());
        let vert_f = Point::new(11.0, -5.0 - 3.0f32.sqrt());
        assert!(
            poly.vertices[0] == vert_a
                && poly.vertices[1] == vert_b
                && poly.vertices[2] == vert_c
                && poly.vertices[3] == vert_d
                && poly.vertices[4] == vert_e
                && poly.vertices[5] == vert_f,
            "{} == {}, {}, {}, {}, {}, {}",
            poly,
            vert_a,
            vert_b,
            vert_c,
            vert_d,
            vert_e,
            vert_f
        );
    }
    #[test]
    fn test_to_polygon_pointy_topped() {
        let hexagon = Hexagon::new(Point::new(10.0, -5.0), 2.0, Angle::new(90f64));
        let poly = hexagon.polygon();
        let vert_a = Point::new(10.0, -3.0);
        let vert_b = Point::new(10.0 - 3.0f32.sqrt(), -4.0);
        let vert_c = Point::new(10.0 - 3.0f32.sqrt(), -6.0);
        let vert_d = Point::new(10.0, -7.0);
        let vert_e = Point::new(10.0 + 3.0f32.sqrt(), -6.0);
        let vert_f = Point::new(10.0 + 3.0f32.sqrt(), -4.0);
        assert!(
            poly.vertices[0] == vert_a
                && poly.vertices[1] == vert_b
                && poly.vertices[2] == vert_c
                && poly.vertices[3] == vert_d
                && poly.vertices[4] == vert_e
                && poly.vertices[5] == vert_f,
            "{} == {}, {}, {}, {}, {}, {}",
            poly,
            vert_a,
            vert_b,
            vert_c,
            vert_d,
            vert_e,
            vert_f
        );
    }
    #[test]
    fn test_closest_point() {
        let hexagon = Hexagon::new(Point::new(0.0, 0.0), 10.0, Angle::zero());
        let point = Point::new(0.0, -15.0);
        let result = hexagon.closest_point(point);
        let expected = Point::new(0.0, -10.0 * 3.0f32.sqrt() / 2.0);
        assert_eq!(result, expected);
    }
}
