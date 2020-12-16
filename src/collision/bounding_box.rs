use crate::base::*;
use crate::collision::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoundingBox {
    pub center: Point,
    pub half: Size,
}

impl BoundingBox {
    pub fn new(center: Point, half: Size) -> Self {
        Self { center, half }
    }
    pub fn from_edges(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        assert!(x1 < x2 && y1 < y2);
        let half = Size::new(x2 - x1, y2 - y1) / 2.0;
        let center = Point::new(x1 + half.w, y1 + half.h);
        Self { center, half }
    }
    pub fn x1(&self) -> f32 {
        self.center.x - self.half.w
    }
    pub fn x2(&self) -> f32 {
        self.center.x + self.half.w
    }
    pub fn y1(&self) -> f32 {
        self.center.y - self.half.h
    }
    pub fn y2(&self) -> f32 {
        self.center.y + self.half.h
    }
    pub fn min_x1(&self, other: BoundingBox) -> f32 {
        self.x1().min(other.x1())
    }
    pub fn min_x2(&self, other: BoundingBox) -> f32 {
        self.x2().min(other.x2())
    }
    pub fn max_x1(&self, other: BoundingBox) -> f32 {
        self.x1().max(other.x1())
    }
    pub fn max_x2(&self, other: BoundingBox) -> f32 {
        self.x2().max(other.x2())
    }
    pub fn min_y1(&self, other: BoundingBox) -> f32 {
        self.y1().min(other.y1())
    }
    pub fn min_y2(&self, other: BoundingBox) -> f32 {
        self.y2().min(other.y2())
    }
    pub fn max_y1(&self, other: BoundingBox) -> f32 {
        self.y1().max(other.y1())
    }
    pub fn max_y2(&self, other: BoundingBox) -> f32 {
        self.y2().max(other.y2())
    }
    pub fn is_inside(&self, point: Point) -> bool {
        point.x > self.x1() && point.x < self.x2() && point.y > self.y1() && point.y < self.y2()
    }
    pub fn overlaps_x(&self, other: BoundingBox) -> bool {
        other.x2() > self.x1() && other.x1() < self.x2()
    }
    pub fn overlaps_y(&self, other: BoundingBox) -> bool {
        other.y2() > self.y1() && other.y1() < self.y2()
    }
    pub fn is_above(&self, other: BoundingBox) -> bool {
        self.y1() > other.y2()
    }
    pub fn is_below(&self, other: BoundingBox) -> bool {
        self.y2() < other.y1()
    }
    pub fn is_left(&self, other: BoundingBox) -> bool {
        self.x2() < other.x1()
    }
    pub fn is_right(&self, other: BoundingBox) -> bool {
        self.x1() > other.x2()
    }
    pub fn distance(&self, other: BoundingBox) -> f32 {
        if self.overlaps_x(other) {
            if self.is_above(other) {
                return self.y1() - other.y2();
            } else if self.is_below(other) {
                return other.y1() - self.y2();
            } else {
                return 0.0; // intersecting
            }
        } else if self.overlaps_y(other) {
            if self.is_left(other) {
                return other.x1() - self.x2();
            } else if self.is_right(other) {
                return self.x1() - other.x2();
            } else {
                return 0.0; // intersecting
            }
        } else {
            return ((self.max_x1(other) - self.min_x2(other)).powf(2.0)
                + (self.max_y1(other) - self.min_y2(other)).powf(2.0))
            .sqrt();
        }
    }
    pub fn hit_point(&self, point: Point) -> Option<Hit> {
        let dx = point.x - self.center.x;
        let px = self.half.w - dx.abs();
        if px <= 0.0 {
            return None;
        }
        let dy = point.y - self.center.y;
        let py = self.half.h - dy.abs();
        if py <= 0.0 {
            return None;
        }
        if px < py {
            Some(Hit::new(
                Point::new(self.center.x + self.half.w * dx.signum(), point.y),
                Vector::new(dx.signum(), 0.0),
                Vector::new(px * dx.signum(), 0.0),
            ))
        } else {
            Some(Hit::new(
                Point::new(point.x, self.center.y + self.half.h * dy.signum()),
                Vector::new(0.0, dy.signum()),
                Vector::new(0.0, dy * dy.signum()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collision::BoundingBox;

    #[test]
    fn test_distance() {
        let bb_ref = BoundingBox::from_edges(1.0, 1.0, 5.0, 3.0);

        assert_eq!(bb_ref.distance(bb_ref), 0.0, "Testing with self");
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(2.0, -2.0, 3.0, 2.0)),
            0.0,
            "Testing with intersecting box"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(-2.0, 0.0, 0.0, 2.0)),
            1.0,
            "Testing with box left"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(7.0, 0.0, 9.0, 2.0)),
            2.0,
            "Testing with right box right"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(2.0, 4.0, 4.0, 5.0)),
            1.0,
            "Testing with box above"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(-1.0, -3.0, 3.0, -1.0)),
            2.0,
            "Testing with box below"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(6.0, 4.0, 8.0, 10.0)),
            2f32.sqrt(),
            "Testing with box above right"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(-4.0, 4.0, 0.0, 10.0)),
            2f32.sqrt(),
            "Testing with box above left"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(6.0, -2.0, 8.0, 0.0)),
            2f32.sqrt(),
            "Testing with box below right"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::from_edges(-4.0, -3.0, 0.0, 0.0)),
            2f32.sqrt(),
            "Testing with box below left"
        );
    }

    #[test]
    fn test_overlaps_x() {
        let bb_ref = BoundingBox::from_edges(1.0, 1.0, 5.0, 3.0);

        let y1 = -2.0;
        let y2 = 0.0;

        // test with selfq
        assert!(bb_ref.overlaps_x(bb_ref) == true);
        // test overlapping left side
        assert!(bb_ref.overlaps_x(BoundingBox::from_edges(-1.0, y1, 3.0, y2)) == true);
        // test overlapping right side
        assert!(bb_ref.overlaps_x(BoundingBox::from_edges(3.0, y1, 7.0, y2)) == true);
        // test not overlapping left side
        assert!(bb_ref.overlaps_x(BoundingBox::from_edges(-5.0, y1, 0.0, y2)) == false);
        // test not overlapping right side
        assert!(bb_ref.overlaps_x(BoundingBox::from_edges(6.0, y1, 11.0, y2)) == false);
        // test overlapping with smaller box
        assert!(bb_ref.overlaps_x(BoundingBox::from_edges(2.0, y1, 4.0, y2)) == true);
        // test overlapping with bigger box
        assert!(bb_ref.overlaps_x(BoundingBox::from_edges(0.0, y1, 6.0, y2)) == true);
    }

    #[test]
    fn test_overlaps_y() {
        let bb_ref = BoundingBox::from_edges(1.0, 1.0, 3.0, 5.0);

        let x1 = -2.0;
        let x2 = 0.0;

        // test with self
        assert!(bb_ref.overlaps_y(bb_ref) == true);
        // test overlapping top side
        assert!(bb_ref.overlaps_y(BoundingBox::from_edges(x1, -1.0, x2, 3.0)) == true);
        // test overlapping bottom side
        assert!(bb_ref.overlaps_y(BoundingBox::from_edges(x1, 3.0, x2, 7.0)) == true);
        // test not overlapping left side
        assert!(bb_ref.overlaps_y(BoundingBox::from_edges(x1, -5.0, x2, 0.0)) == false);
        // test not overlapping right side
        assert!(bb_ref.overlaps_y(BoundingBox::from_edges(x1, 6.0, x2, 11.0)) == false);
        // test overlapping with smaller box
        assert!(bb_ref.overlaps_y(BoundingBox::from_edges(x1, 2.0, x2, 4.0)) == true);
        // test overlapping with bigger box
        assert!(bb_ref.overlaps_y(BoundingBox::from_edges(x1, 0.0, x2, 6.0)) == true);
    }
}
