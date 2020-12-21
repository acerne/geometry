use crate::base::*;
use crate::collision::*;
use crate::shape::*;

pub trait Collidable {
    fn hit_point(&self, point: Point) -> Option<Hit>;
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit>;
}

impl Collidable for Ray {
    fn hit_point(&self, point: Point) -> Option<Hit> {
        None
    }
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit> {
        let dir = self.vector();
        let near_x =
            (bounding_box.center.x - dir.dx.signum() * bounding_box.half.w - self.origin.x)
                / dir.dx;
        let near_y =
            (bounding_box.center.y - dir.dy.signum() * bounding_box.half.h - self.origin.y)
                / dir.dy;
        let far_x = (bounding_box.center.x + dir.dx.signum() * bounding_box.half.w - self.origin.x)
            / dir.dx;
        let far_y = (bounding_box.center.y + dir.dy.signum() * bounding_box.half.h - self.origin.y)
            / dir.dy;

        if near_x > far_y || near_y > far_x {
            return None;
        }

        let near = near_x.max(near_y);
        let far = far_x.min(far_y);

        if near >= 1.0 || far <= 0.0 {
            return None;
        }
        let time = near.max(0.0).min(1.0);
        if near < far {
            return Some(Hit::new_time(
                (self.origin + dir) * time,
                Vector::new(-dir.dx.signum(), 0.0),
                -dir * (1.0 - time),
                time,
            ));
        } else {
            return Some(Hit::new_time(
                (self.origin + dir) * time,
                Vector::new(0.0, -dir.dy.signum()),
                -dir * (1.0 - time),
                time,
            ));
        }
    }
}

impl Collidable for BoundingBox {
    fn hit_point(&self, point: Point) -> Option<Hit> {
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
                Vector::new(0.0, py * dy.signum()),
            ))
        }
    }
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit> {
        let dx = bounding_box.center.x - self.center.x;
        let px = (bounding_box.half.w + self.half.w) - dx.abs();
        if px <= 0.0 {
            return None;
        }
        let dy = bounding_box.center.y - self.center.y;
        let py = (bounding_box.half.h + self.half.h) - dy.abs();
        if py <= 0.0 {
            return None;
        }
        if px < py {
            Some(Hit::new(
                Point::new(
                    self.center.x + self.half.w * dx.signum(),
                    bounding_box.center.y,
                ),
                Vector::new(dx.signum(), 0.0),
                Vector::new(px * dx.signum(), 0.0),
            ))
        } else {
            Some(Hit::new(
                Point::new(
                    bounding_box.center.x,
                    self.center.y + self.half.h * dy.signum(),
                ),
                Vector::new(0.0, dy.signum()),
                Vector::new(0.0, py * dy.signum()),
            ))
        }
    }
}

impl Collidable for Rectangle {
    fn hit_point(&self, point: Point) -> Option<Hit> {
        if self.polygon().is_inside(point) {
            let closest = self.closest_point(point);
            let delta = Vector::from_points(point, closest);
            return Some(Hit::new(closest, delta.get_unit_vector(), delta));
        }
        None
    }
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit> {
        None
        // let dx = bounding_box.center.x - self.center.x;
        // let px = (bounding_box.half.w + self.half.w) - dx.abs();
        // if px <= 0.0 {
        //     return None;
        // }
        // let dy = bounding_box.center.y - self.center.y;
        // let py = (bounding_box.half.h + self.half.h) - dy.abs();
        // if py <= 0.0 {
        //     return None;
        // }
        // if px < py {
        //     Some(Hit::new(
        //         Point::new(
        //             self.center.x + self.half.w * dx.signum(),
        //             bounding_box.center.y,
        //         ),
        //         Vector::new(dx.signum(), 0.0),
        //         Vector::new(px * dx.signum(), 0.0),
        //     ))
        // } else {
        //     Some(Hit::new(
        //         Point::new(
        //             bounding_box.center.x,
        //             self.center.y + self.half.h * dy.signum(),
        //         ),
        //         Vector::new(0.0, dy.signum()),
        //         Vector::new(0.0, py * dy.signum()),
        //     ))
        // }
    }
}
