#[derive(Clone, Copy, Debug, Default)]
pub struct Angle {
    deg: f64,
}

impl Angle {
    pub fn new(deg: f64) -> Self {
        let deg = wrap(deg);
        Self { deg }
    }
    pub fn zero() -> Self {
        Self { deg: 0f64 }
    }
    pub fn from_rad(rad: f64) -> Self {
        let deg = wrap(rad.to_degrees());
        Self { deg }
    }
    pub fn pi() -> Self {
        Self { deg: 180f64 }
    }
    pub fn to_rad64(&self) -> f64 {
        self.deg.to_radians()
    }
    pub fn to_rad32(&self) -> f32 {
        self.deg.to_radians() as f32
    }
    pub fn sin(&self) -> f64 {
        self.deg.to_radians().sin()
    }
    pub fn cos(&self) -> f64 {
        self.deg.to_radians().cos()
    }
}

fn wrap(deg: f64) -> f64 {
    deg - (deg / 360f64).floor() * 360f64
}

impl std::ops::Add<Angle> for Angle {
    type Output = Self;
    fn add(self, other: Angle) -> Self {
        Self {
            deg: wrap(self.deg + other.deg),
        }
    }
}

impl std::ops::Sub<Angle> for Angle {
    type Output = Self;
    fn sub(self, other: Angle) -> Self {
        Self {
            deg: wrap(self.deg - other.deg),
        }
    }
}

impl std::ops::Mul<Angle> for Angle {
    type Output = Self;
    fn mul(self, other: Angle) -> Self {
        Self {
            deg: wrap(self.deg * other.deg),
        }
    }
}

impl std::ops::Div<Angle> for Angle {
    type Output = Self;
    fn div(self, other: Angle) -> Self {
        Self {
            deg: wrap(self.deg / other.deg),
        }
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        float_eq::FloatEq::eq_abs(&self.deg, &other.deg, &10e-6)
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}°)", self.deg)
    }
}
