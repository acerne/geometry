use crate::base::Point;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoundingBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl BoundingBox {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        assert!(x1 < x2 && y1 < y2);
        Self { x1, y1, x2, y2 }
    }
    pub fn min_x1(&self, other: BoundingBox) -> f32 {
        self.x1.min(other.x1)
    }
    pub fn min_x2(&self, other: BoundingBox) -> f32 {
        self.x2.min(other.x2)
    }
    pub fn max_x1(&self, other: BoundingBox) -> f32 {
        self.x1.max(other.x1)
    }
    pub fn max_x2(&self, other: BoundingBox) -> f32 {
        self.x2.max(other.x2)
    }
    pub fn min_y1(&self, other: BoundingBox) -> f32 {
        self.y1.min(other.y1)
    }
    pub fn min_y2(&self, other: BoundingBox) -> f32 {
        self.y2.min(other.y2)
    }
    pub fn max_y1(&self, other: BoundingBox) -> f32 {
        self.y1.max(other.y1)
    }
    pub fn max_y2(&self, other: BoundingBox) -> f32 {
        self.y2.max(other.y2)
    }
    // pub fn is_under(&self, point: Point) -> bool {
    //     // Should I use screen coordinate system (x right, y down) or normal coordinate system (x right, y up)?
    //     // From perspective of a game "is_above" must be as seen on display
    //     point.x > self.x1 && point.x < self.x2 && point.y < self.y1
    // }
    // pub fn is_above(&self, point: Point) -> bool {
    //     point.x > self.x1 && point.x < self.x2 && point.y > self.y2
    // }
    // pub fn is_left(&self, point: Point) -> bool {
    //     point.y > self.y1 && point.y < self.y2 && point.x < self.x1
    // }
    // pub fn is_right(&self, point: Point) -> bool {
    //     point.y > self.y1 && point.y < self.y2 && point.x > self.x2
    // }
    pub fn is_inside(&self, point: Point) -> bool {
        point.x > self.x1 && point.x < self.x2 && point.y > self.y1 && point.y < self.y2
    }
    pub fn overlaps_x(&self, other: BoundingBox) -> bool {
        other.x2 > self.x1 && other.x1 < self.x2
    }
    pub fn overlaps_y(&self, other: BoundingBox) -> bool {
        other.y2 > self.y1 && other.y1 < self.y2
    }
    pub fn is_above(&self, other: BoundingBox) -> bool {
        self.y1 > other.y2
    }
    pub fn is_below(&self, other: BoundingBox) -> bool {
        self.y2 < other.y1
    }
    pub fn is_left(&self, other: BoundingBox) -> bool {
        self.x2 < other.x1
    }
    pub fn is_right(&self, other: BoundingBox) -> bool {
        self.x1 > other.x2
    }
    pub fn distance(&self, other: BoundingBox) -> f32 {
        if self.overlaps_x(other) {
            if self.is_above(other) {
                return self.y1 - other.y2;
            } else if self.is_below(other) {
                return other.y1 - self.y2;
            } else {
                return 0.0; // intersecting
            }
        } else if self.overlaps_y(other) {
            if self.is_left(other) {
                return other.x1 - self.x2;
            } else if self.is_right(other) {
                return self.x1 - other.x2;
            } else {
                return 0.0; // intersecting
            }
        } else {
            return ((self.max_x1(other) - self.min_x2(other)).powf(2.0)
                + (self.max_y1(other) - self.min_y2(other)).powf(2.0))
            .sqrt();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collision::BoundingBox;

    #[test]
    fn test_distance() {
        let bb_ref = BoundingBox::new(1.0, 1.0, 5.0, 3.0);

        assert_eq!(bb_ref.distance(bb_ref), 0.0, "Testing with self");
        assert_eq!(
            bb_ref.distance(BoundingBox::new(2.0, -2.0, 3.0, 2.0)),
            0.0,
            "Testing with intersecting box"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(-2.0, 0.0, 0.0, 2.0)),
            1.0,
            "Testing with box left"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(7.0, 0.0, 9.0, 2.0)),
            2.0,
            "Testing with right box right"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(2.0, 4.0, 4.0, 5.0)),
            1.0,
            "Testing with box above"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(-1.0, -3.0, 3.0, -1.0)),
            2.0,
            "Testing with box below"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(6.0, 4.0, 8.0, 10.0)),
            2f32.sqrt(),
            "Testing with box above right"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(-4.0, 4.0, 0.0, 10.0)),
            2f32.sqrt(),
            "Testing with box above left"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(6.0, -2.0, 8.0, 0.0)),
            2f32.sqrt(),
            "Testing with box below right"
        );
        assert_eq!(
            bb_ref.distance(BoundingBox::new(-4.0, -3.0, 0.0, 0.0)),
            2f32.sqrt(),
            "Testing with box below left"
        );
    }

    #[test]
    fn test_overlaps_x() {
        let bb_ref = BoundingBox::new(1.0, 1.0, 5.0, 3.0);

        let y1 = -2.0;
        let y2 = 0.0;

        // test with self
        assert!(bb_ref.overlaps_x(bb_ref) == true);
        // test overlapping left side
        assert!(bb_ref.overlaps_x(BoundingBox::new(-1.0, y1, 3.0, y2)) == true);
        // test overlapping right side
        assert!(bb_ref.overlaps_x(BoundingBox::new(3.0, y1, 7.0, y2)) == true);
        // test not overlapping left side
        assert!(bb_ref.overlaps_x(BoundingBox::new(-5.0, y1, 0.0, y2)) == false);
        // test not overlapping right side
        assert!(bb_ref.overlaps_x(BoundingBox::new(6.0, y1, 11.0, y2)) == false);
        // test overlapping with smaller box
        assert!(bb_ref.overlaps_x(BoundingBox::new(2.0, y1, 4.0, y2)) == true);
        // test overlapping with bigger box
        assert!(bb_ref.overlaps_x(BoundingBox::new(0.0, y1, 6.0, y2)) == true);
    }

    #[test]
    fn test_overlaps_y() {
        let bb_ref = BoundingBox::new(1.0, 1.0, 3.0, 5.0);

        let x1 = -2.0;
        let x2 = 0.0;

        // test with self
        assert!(bb_ref.overlaps_y(bb_ref) == true);
        // test overlapping top side
        assert!(bb_ref.overlaps_y(BoundingBox::new(x1, -1.0, x2, 3.0)) == true);
        // test overlapping bottom side
        assert!(bb_ref.overlaps_y(BoundingBox::new(x1, 3.0, x2, 7.0)) == true);
        // test not overlapping left side
        assert!(bb_ref.overlaps_y(BoundingBox::new(x1, -5.0, x2, 0.0)) == false);
        // test not overlapping right side
        assert!(bb_ref.overlaps_y(BoundingBox::new(x1, 6.0, x2, 11.0)) == false);
        // test overlapping with smaller box
        assert!(bb_ref.overlaps_y(BoundingBox::new(x1, 2.0, x2, 4.0)) == true);
        // test overlapping with bigger box
        assert!(bb_ref.overlaps_y(BoundingBox::new(x1, 0.0, x2, 6.0)) == true);
    }
}
