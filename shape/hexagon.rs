use crate::geometry::base::{Angle, Point, Vector};
use crate::geometry::shape::{shape::Shape, Polygon};
use float_eq::FloatEq;

pub struct Hexagon {
    pub center: Point,
    pub side: f32,
    pub phi: Angle,
}

impl Hexagon {
    fn new(center: Point, side: f32, phi: Angle) -> Self {
        Self { center, side, phi }
    }
}

impl Shape for Hexagon {
    fn translate(&mut self, vector: Vector) {
        self.center = self.center + vector;
    }
    fn move_to(&mut self, point: Point) {
        self.center = point;
    }
    fn rotate(&mut self, theta: Angle) {
        self.phi = self.phi + theta;
    }
    fn rotate_to(&mut self, phi: Angle) {
        self.phi = phi;
    }
    fn to_polygon(&self) -> Polygon {
        let mut vertices = Vec::new();
        vertices.reserve(6);
        for i in 0..6 {
            let theta = self.phi + Angle::new(i as f64 * 60f64);
            let xh = self.center.x + (theta.cos() as f32) * self.side;
            let yh = self.center.y + (theta.sin() as f32) * self.side;
            vertices.push(Point::new(xh, yh));
        }
        Polygon { vertices }
    }
}

impl PartialEq for Hexagon {
    fn eq(&self, other: &Self) -> bool {
        self.center.eq_abs(&other.center, &10e-6)
            && float_eq::float_eq!(self.side, other.side, abs <= 10e-6)
            && self.phi.eq_abs(&other.phi, &10e-6)
    }
}

impl std::fmt::Debug for Hexagon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Hexagon")
            .field("center", &self.center)
            .field("side", &self.side)
            .field("phi", &self.phi)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::base::{Angle, Point, Vector};
    use crate::geometry::shape::{shape::Shape, Hexagon};
    use float_eq::FloatEq;
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
        let poly = hexagon.to_polygon();
        let vert_a = Point::new(12.0, -5.0);
        let vert_b = Point::new(11.0, -5.0 + 3.0f32.sqrt());
        let vert_c = Point::new(9.0, -5.0 + 3.0f32.sqrt());
        let vert_d = Point::new(8.0, -5.0);
        let vert_e = Point::new(9.0, -5.0 - 3.0f32.sqrt());
        let vert_f = Point::new(11.0, -5.0 - 3.0f32.sqrt());
        assert!(
            poly.vertices[0].eq_abs(&vert_a, &10e-6)
                && poly.vertices[1].eq_abs(&vert_b, &10e-6)
                && poly.vertices[2].eq_abs(&vert_c, &10e-6)
                && poly.vertices[3].eq_abs(&vert_d, &10e-6)
                && poly.vertices[4].eq_abs(&vert_e, &10e-6)
                && poly.vertices[5].eq_abs(&vert_f, &10e-6),
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
        let poly = hexagon.to_polygon();
        let vert_a = Point::new(10.0, -3.0);
        let vert_b = Point::new(10.0 - 3.0f32.sqrt(), -4.0);
        let vert_c = Point::new(10.0 - 3.0f32.sqrt(), -6.0);
        let vert_d = Point::new(10.0, -7.0);
        let vert_e = Point::new(10.0 + 3.0f32.sqrt(), -6.0);
        let vert_f = Point::new(10.0 + 3.0f32.sqrt(), -4.0);
        assert!(
            poly.vertices[0].eq_abs(&vert_a, &10e-6)
                && poly.vertices[1].eq_abs(&vert_b, &10e-6)
                && poly.vertices[2].eq_abs(&vert_c, &10e-6)
                && poly.vertices[3].eq_abs(&vert_d, &10e-6)
                && poly.vertices[4].eq_abs(&vert_e, &10e-6)
                && poly.vertices[5].eq_abs(&vert_f, &10e-6),
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
}
