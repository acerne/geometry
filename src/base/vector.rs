use crate::base::{Angle, Point, Scale, Size};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32,
}

#[allow(dead_code)]
impl Vector {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn from_points(point_a: Point, point_b: Point) -> Self {
        let diff = point_b - point_a;
        Self {
            dx: diff.x,
            dy: diff.y,
        }
    }
    pub fn from_magnitude(magnitude: f32, orientation: Angle) -> Self {
        let dx = magnitude * orientation.cos() as f32;
        let dy = magnitude * orientation.sin() as f32;
        Self { dx, dy }
    }
    pub fn from_orientation(orientation: Angle) -> Self {
        let dx = orientation.cos() as f32;
        let dy = orientation.sin() as f32;
        Self { dx, dy }
    }
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.dx = self.dx / mag;
        self.dy = self.dy / mag;
    }
    pub fn rotate(&mut self, phi: f32) {
        let x1 = self.dx;
        let y1 = self.dy;
        self.dx = x1 * phi.cos() - y1 * phi.sin();
        self.dy = x1 * phi.sin() + y1 * phi.cos();
    }
    pub fn dot(self, other: Self) -> f32 {
        self.dx * other.dx + self.dy * other.dy
    }
    pub fn cross(self, other: Self) -> f32 {
        self.dx * other.dy - self.dy * other.dx
    }
    pub fn magnitude(&self) -> f32 {
        (self.dx.powf(2.0) + self.dy.powf(2.0)).sqrt()
    }
    pub fn squared_magnitude(&self) -> f32 {
        self.dx.powf(2.0) + self.dy.powf(2.0)
    }
    pub fn orientation(&self) -> Angle {
        let rad = self.dy.atan2(self.dx);
        Angle::new((rad as f64).to_degrees())
    }
    pub fn get_normal_vector(&self) -> Vector {
        Vector {
            dx: -self.dy,
            dy: self.dx,
        }
    }
    pub fn get_unit_vector(self) -> Vector {
        let mag = self.magnitude();
        self / mag
    }
    pub fn to_point(self) -> Point {
        Point {
            x: self.dx,
            y: self.dy,
        }
    }
    pub fn to_size(self) -> Size {
        Size {
            w: self.dx,
            h: self.dy,
        }
    }
    pub fn to_scale(self) -> Scale {
        Scale {
            sx: self.dx,
            sy: self.dy,
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

impl std::ops::Add<f32> for Vector {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            dx: self.dx + other,
            dy: self.dy + other,
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            dx: self.dx - other.dx,
            dy: self.dy - other.dy,
        }
    }
}

impl std::ops::Sub<f32> for Vector {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            dx: self.dx - other,
            dy: self.dy - other,
        }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            dx: self.dx * other,
            dy: self.dy * other,
        }
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            dx: self.dx / other,
            dy: self.dy / other,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        float_eq::FloatEq::eq_abs(&self.dx, &other.dx, &10e-6)
            && float_eq::FloatEq::eq_abs(&self.dy, &other.dy, &10e-6)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::{Angle, Vector};

    #[test]
    fn test_normalize() {
        let mut vector_a = Vector::new(5.0, -5.0);
        vector_a.normalize();
        let vector_b = Vector::new(1.0 / 2f32.sqrt(), -1.0 / 2f32.sqrt());
        assert!(vector_a == vector_b, "{} == {}", vector_a, vector_b);
    }
    #[test]
    fn test_rotate() {
        let mut vector_a = Vector::new(1.0, 1.0);
        vector_a.rotate(90f32.to_radians());
        let vector_b = Vector::new(-1.0, 1.0);
        assert!(vector_a == vector_b, "{} == {}", vector_a, vector_b);
    }
    #[test]
    fn test_dot() {
        let vector_a = Vector::new(5.0, 3.0);
        let vector_b = Vector::new(-1.0, 5.0);
        let result = vector_a.dot(vector_b);
        assert_eq!(result, 10f32);
    }
    #[test]
    fn test_cross() {
        let vector_a = Vector::new(5.0, 3.0);
        let vector_b = Vector::new(-1.0, 5.0);
        let result = vector_a.cross(vector_b);
        assert_eq!(result, 28f32);
    }
    #[test]
    fn test_magnitude() {
        let vector_a = Vector::new(1.0, -1.0);
        let result = vector_a.magnitude();
        assert_eq!(result, 2f32.sqrt());
    }
    #[test]
    fn test_get_orientation() {
        let vector_a = Vector::new(1.0, -1.0);
        let result = vector_a.orientation();
        let expected = Angle::new(-45f64);
        assert!(result == expected, "{} == {}", result, expected);
    }
}
