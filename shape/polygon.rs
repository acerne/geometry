use crate::geometry::base::{Angle, Line, Point, Vector};

#[derive(Debug, Default, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: &Vec<Point>) -> Self {
        let p = vertices.clone();
        Self { vertices: p }
    }
    pub fn from_vectors(start: Point, vectors: &Vec<Vector>) -> Self {
        let mut vertices = Vec::new();
        vertices.push(start);
        for vector in vectors.iter() {
            vertices.push(vertices.last().unwrap().clone() + vector.clone());
        }
        Self { vertices }
    }
    pub fn to_lines(&self) -> Vec<Line> {
        let mut lines = Vec::new();
        let n = self.vertices.len();
        for i in 0..n {
            lines.push(Line::new(self.vertices[i], self.vertices[(i + 1) % n]));
        }
        lines
    }
    pub fn closest_point(&self, point: Point) -> Point {
        let sides = self.to_lines();
        let mut best = sides.first().unwrap().closest_point(point);
        for side in sides.iter().skip(1) {
            let candidate = side.closest_point(point);
            if candidate.distance_to(point) < best.distance_to(point) {
                best = candidate;
            }
        }
        best
    }
    pub fn is_inside(&self, point: Point) -> bool {
        let mut angle_sum = Angle::zero();

        let n = self.vertices.len();
        for i in 0..n {
            let angle = Angle::from_vectors(
                Vector::from_points(point, self.vertices[i]),
                Vector::from_points(point, self.vertices[(i + 1) % n]),
            );
            angle_sum = angle_sum + angle;
        }
        float_eq::FloatEq::eq_abs(&angle_sum.deg, &360f64, &10e-3)
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = self.vertices.first();
        if let Some(first) = first {
            write!(f, "{}", first)?;
            for item in self.vertices.iter().skip(1) {
                write!(f, ", {}", item)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::base::{Line, Point, Vector};
    use crate::geometry::shape::Polygon;

    #[test]
    fn test_to_lines() {
        let point_a = Point::new(1.0, 0.0);
        let point_b = Point::new(1.0, 3.0);
        let point_c = Point::new(-2.0, 3.0);
        let points = vec![point_a, point_b, point_c];
        let poly = Polygon::new(&points);
        let lines = poly.to_lines();
        let expected_ab = Line::new(point_a, point_b);
        let expected_bc = Line::new(point_b, point_c);
        let expected_ca = Line::new(point_c, point_a);
        assert_eq!(lines[0], expected_ab);
        assert_eq!(lines[1], expected_bc);
        assert_eq!(lines[2], expected_ca);
    }
    #[test]
    fn test_closest_point() {
        let point_a = Point::new(1.0, 0.0);
        let point_b = Point::new(1.0, 3.0);
        let point_c = Point::new(-2.0, 3.0);
        let points = vec![point_a, point_b, point_c];
        let poly = Polygon::new(&points);
        // test middle of polygon line segment
        let point = Point::new(2.0, 2.0);
        let result = poly.closest_point(point);
        let expected = Point::new(1.0, 2.0);
        assert_eq!(result, expected);
        // test outside of polygon line segment origin
        let point = Point::new(2.0, 4.0);
        let result = poly.closest_point(point);
        let expected = Point::new(1.0, 3.0);
        assert_eq!(result, expected);
        // test outside of polygonline segment end
        let point = Point::new(0.0, -1.0);
        let result = poly.closest_point(point);
        let expected = Point::new(1.0, 0.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_is_inside() {
        let point_a = Point::new(1.0, 0.0);
        let point_b = Point::new(1.0, 3.0);
        let point_c = Point::new(-2.0, 3.0);
        let points = vec![point_a, point_b, point_c];
        let poly = Polygon::new(&points);
        // test inside
        let point = Point::new(0.0, 2.0);
        assert!(poly.is_inside(point) == true, "Point is inside");
        // test on border - not inside!
        let point = Point::new(1.0, 2.0);
        assert!(poly.is_inside(point) == false, "Point is on border");
        // test outside
        let point = Point::new(2.0, 2.0);
        assert!(poly.is_inside(point) == false, "Point is outside");
    }
}
