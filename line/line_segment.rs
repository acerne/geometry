use crate::geometry::base::{Point, Vector};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LineSegment {
    pub origin: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(origin: Point, end: Point) -> Self {
        Self { origin, end }
    }
    pub fn from_vector(point: Point, vector: Vector) -> Self {
        Self {
            origin: point,
            end: point + vector,
        }
    }
    pub fn to_vector(&self) -> Vector {
        let diff = self.origin - self.end;
        Vector {
            dx: diff.x,
            dy: diff.y,
        }
    }
    pub fn closest_point(&self, point: Point) -> Point {
        let ab = Vector::from_points(self.origin, self.end);
        let ap = Vector::from_points(self.origin, point);
        let t = (ap.dx * ab.dx + ap.dy * ab.dy) / (ab.dx.powf(2.0) + ab.dy.powf(2.0));
        let t_constr = t.min(1.0).max(0.0);
        return self.origin + ab * t_constr;
    }
    pub fn is_on_segment(&self, point: Point) -> bool {
        point.distance_to(self.origin) + point.distance_to(self.end)
            == self.origin.distance_to(self.end)
    }
    fn intersection(&self, other: LineSegment) -> Option<Point> {
        let a1 = self.end.y - self.origin.y;
        let b1 = self.origin.x - self.end.x;
        let c1 = a1 * self.origin.x + b1 * self.origin.y;
        let a2 = other.end.y - other.origin.y;
        let b2 = other.origin.x - other.end.x;
        let c2 = a2 * other.origin.x + b2 * other.origin.y;
        let delta = a1 * b2 - a2 * b1;
        if delta == 0.0 {
            return None;
        }
        let intersection = Point {
            x: (b2 * c1 - b1 * c2) / delta,
            y: (a1 * c2 - a2 * c1) / delta,
        };
        if !self.is_on_segment(intersection) {
            return None;
        }
        Some(intersection)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::base::{Point, Vector};
    use crate::geometry::line::LineSegment;

    #[test]
    fn test_from_vector() {
        let point_a = Point::new(1.0, 1.0);
        let point_b = Point::new(-1.0, -1.0);
        let vector = Vector::from_points(point_a, point_b);
        let line = LineSegment::from_vector(point_a, vector);
        assert!(line.origin == point_a, "{} == {}", line.origin, point_a);
        assert!(line.end == point_b, "{} == {}", line.origin, point_b);
    }
    #[test]
    fn test_closest_point() {
        let point_a = Point::new(2.0, 0.0);
        let point_b = Point::new(0.0, 2.0);
        let line = LineSegment::new(point_a, point_b);
        // test middle of line segment
        let point = Point::new(2.0, 2.0);
        let result = line.closest_point(point);
        let expected = Point::new(1.0, 1.0);
        assert_eq!(result, expected);
        // test outside of line segment origin
        let point = Point::new(3.0, 0.0);
        let result = line.closest_point(point);
        let expected = Point::new(2.0, 0.0);
        assert_eq!(result, expected);
        // test outside of line segment end
        let point = Point::new(0.0, 3.0);
        let result = line.closest_point(point);
        let expected = Point::new(0.0, 2.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_is_on_segment() {
        let point_a = Point::new(1.0, 1.0);
        let point_b = Point::new(-1.0, -1.0);
        let line = LineSegment::new(point_a, point_b);
        // test point on line segment
        let test_point = Point::zero();
        assert!(line.is_on_segment(test_point));
        // test point not on line segment
        let test_point = Point::new(1.0, -1.0);
        assert!(!line.is_on_segment(test_point));
        // test point on line, but not on line segment
        let test_point = Point::new(2.0, 2.0);
        assert!(!line.is_on_segment(test_point));
    }
    #[test]
    fn test_intersection() {
        // test intersecting line segments
        let line_a = LineSegment::new(Point::new(1.0, 1.0), Point::new(-1.0, -1.0));
        let line_b = LineSegment::new(Point::new(1.0, -1.0), Point::new(-1.0, 1.0));
        let intersection = line_a.intersection(line_b);
        assert!(!intersection.is_none());
        assert_eq!(intersection.unwrap(), Point::zero());

        // test parallel line segments
        let line_a = LineSegment::new(Point::new(1.0, 1.0), Point::new(1.0, -1.0));
        let line_b = LineSegment::new(Point::new(-1.0, 1.0), Point::new(-1.0, -1.0));
        let intersection = line_a.intersection(line_b);
        assert!(intersection.is_none());

        // test intersecting lines, but not line segments
        let line_a = LineSegment::new(Point::new(1.0, 1.0), Point::new(2.0, -1.0));
        let line_b = LineSegment::new(Point::new(-1.0, 1.0), Point::new(-2.0, -1.0));
        let intersection = line_a.intersection(line_b);
        assert!(intersection.is_none());
    }
}
