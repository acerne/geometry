use float_eq::FloatEq;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

impl FloatEq for Angle {
    type Epsilon = f64;

    fn eq_abs(&self, other: &Self, max_diff: &f64) -> bool {
        self.deg.eq_abs(&other.deg, max_diff)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &f64) -> bool {
        self.deg.eq_rmax(&other.deg, max_diff)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &f64) -> bool {
        self.deg.eq_rmin(&other.deg, max_diff)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &f64) -> bool {
        self.deg.eq_r1st(&other.deg, max_diff)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &f64) -> bool {
        self.deg.eq_r2nd(&other.deg, max_diff)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &float_eq::UlpsEpsilon<f64>) -> bool {
        self.deg.eq_ulps(&other.deg, max_diff)
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}°)", self.deg)
    }
}
