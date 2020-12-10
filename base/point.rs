use crate::geometry::base::{Angle, Vector};

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn distance_to(&self, other: Point) -> f32 {
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }
    pub fn squared_distance_to(&self, other: Point) -> f32 {
        (other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)
    }
    pub fn rotate_about(&mut self, point: Point, theta: Angle) {
        let xo = self.x - point.x;
        let yo = self.y - point.y;
        let cos = theta.cos() as f32;
        let sin = theta.sin() as f32;
        self.x = xo * cos - yo * sin + point.x;
        self.y = xo * sin + yo * cos + point.y;
    }
    pub fn to_vector(self) -> Vector {
        Vector {
            dx: self.x,
            dy: self.y,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl std::ops::Add<f32> for Point {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Self;
    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.dx,
            y: self.y - other.dy,
        }
    }
}

impl std::ops::Sub<f32> for Point {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Div<f32> for Point {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        float_eq::FloatEq::eq_abs(&self.x, &other.x, &10e-6)
            && float_eq::FloatEq::eq_abs(&self.y, &other.y, &10e-6)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::base::{Angle, Point};

    #[test]
    fn test_distance_to() {
        let point_a = Point::new(5.0, 5.0);
        let point_b = Point::new(-5.0, 5.0);
        let distance = point_a.distance_to(point_b);
        assert_eq!(distance, 10f32);
    }
    #[test]
    fn test_rotate_about() {
        let mut point_a = Point::new(5.0, -3.0);
        let point_b = Point::new(2.0, 1.0);
        point_a.rotate_about(point_b, Angle::new(90f64));
        let expected = Point::new(6.0, 4.0);
        assert_eq!(point_a, expected);
    }
}
