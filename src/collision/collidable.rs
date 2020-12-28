use crate::base::*;
use crate::collision::*;
use crate::shape::*;

pub trait Collidable {
    fn hit_point(&self, point: Point) -> Option<Hit>;
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit>;
    fn hit_circle(&self, circle: &Circle) -> Option<Hit>;
}

impl Collidable for Ray {
    fn hit_point(&self, point: Point) -> Option<Hit> {
        let dir = self.vector();
        let closest = Line::from_vector(self.origin, dir).closest_point(point);
        let delta = Vector::from_points(self.origin + dir, point);
        if closest.distance_to(point) < 0.5 {
            return Some(Hit::new_time(
                point,
                delta.get_unit_vector(),
                delta,
                1.0 - delta.magnitude() / dir.magnitude(),
            ));
        }
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
        let time = near;
        if time >= 0.0 && time <= 1.0 {
            if near_x > near_y {
                return Some(Hit::new_time(
                    self.origin + dir * time,
                    Vector::new(-dir.dx.signum(), 0.0),
                    -dir * (1.0 - time),
                    time,
                ));
            } else {
                return Some(Hit::new_time(
                    self.origin + dir * time,
                    Vector::new(0.0, -dir.dy.signum()),
                    -dir * (1.0 - time),
                    time,
                ));
            }
        }
        None
    }
    fn hit_circle(&self, circle: &Circle) -> Option<Hit> {
        let dir = self.vector();
        let dist = Vector::from_points(circle.center(), self.origin);

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(dist);
        let c = dist.dot(dist) - circle.radius().powf(2.0);

        let det = b * b - 4.0 * a * c;
        if det >= 0.0 {
            let t1 = (-b + det.sqrt()) / (2.0 * a);
            let t2 = (-b - det.sqrt()) / (2.0 * a);
            let time = t1.min(t2);
            if time >= 0.0 && time <= 1.0 {
                let contact = self.origin + dir * time;
                let normal = Vector::from_points(circle.center(), contact).get_unit_vector();
                return Some(Hit::new_time(contact, normal, -dir * (1.0 - time), time));
            }
        }
        None
    }
}

impl Collidable for BoundingBox {
    fn hit_point(&self, point: Point) -> Option<Hit> {
        let dx = self.center.x - point.x;
        let px = self.half.w - dx.abs();
        if px <= 0.0 {
            return None;
        }
        let dy = self.center.y - point.y;
        let py = self.half.h - dy.abs();
        if py <= 0.0 {
            return None;
        }
        if px < py {
            Some(Hit::new(
                point,
                Vector::new(dx.signum(), 0.0),
                Vector::new(px * dx.signum(), 0.0),
            ))
        } else {
            Some(Hit::new(
                point,
                Vector::new(0.0, dy.signum()),
                Vector::new(0.0, py * dy.signum()),
            ))
        }
    }
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit> {
        let dx = self.center.x - bounding_box.center.x;
        let px = (self.half.w + bounding_box.half.w) - dx.abs();
        if px <= 0.0 {
            return None;
        }
        let dy = self.center.y - bounding_box.center.y;
        let py = (self.half.h + bounding_box.half.h) - dy.abs();
        if py <= 0.0 {
            return None;
        }
        if px < py {
            Some(Hit::new(
                Point::new(
                    bounding_box.center.x + bounding_box.half.w * dx.signum(),
                    self.center.y,
                ),
                Vector::new(dx.signum(), 0.0),
                Vector::new(px * dx.signum(), 0.0),
            ))
        } else {
            Some(Hit::new(
                Point::new(
                    self.center.x,
                    bounding_box.center.y + bounding_box.half.h * dy.signum(),
                ),
                Vector::new(0.0, dy.signum()),
                Vector::new(0.0, py * dy.signum()),
            ))
        }
    }
    fn hit_circle(&self, circle: &Circle) -> Option<Hit> {
        let dist = Vector::from_points(circle.center(), self.center);
        if dist.dx.abs() <= self.half.w + circle.radius()
            && dist.dy.abs() <= self.half.h + circle.radius()
        {
            // possible collision
            if dist.dy.abs() < dist.dx.abs() && dist.dy.abs() < self.half.h {
                // box is on the left/right side the circle
                let sign_x = dist.dx.signum();
                let adjust = Vector::new(sign_x * circle.radius(), 0.0);
                let delta = adjust + Vector::new(self.half.w * sign_x - dist.dx, 0.0);
                return Some(Hit::new(
                    circle.center() + adjust,
                    Vector::new(sign_x, 0.0),
                    delta,
                ));
            } else if dist.dx.abs() < self.half.w {
                // box is on top/bottom the circle
                let sign_y = dist.dy.signum();
                let adjust = Vector::new(0.0, sign_y * circle.radius());
                let delta = adjust + Vector::new(0.0, self.half.h * sign_y - dist.dy);
                return Some(Hit::new(
                    circle.center() + adjust,
                    Vector::new(0.0, sign_y),
                    delta,
                ));
            }
            let deepest_vertex = self.polygon().closest_point(circle.center());
            let vertex_dist = Vector::from_points(circle.center(), deepest_vertex);
            if vertex_dist.magnitude() < circle.radius() {
                // deepest vertex must be pushed out
                let adjust = vertex_dist.get_unit_vector() * circle.radius();
                let delta = adjust - vertex_dist;
                return Some(Hit::new(
                    circle.center() + adjust,
                    delta.get_unit_vector(),
                    delta,
                ));
            }
        }
        None
    }
}

impl Collidable for Circle {
    fn hit_point(&self, point: Point) -> Option<Hit> {
        let dist = Vector::from_points(point, self.center());
        if dist.magnitude() <= self.radius() {
            let normal = dist.get_unit_vector();
            return Some(Hit::new(
                point,
                normal,
                normal * (self.radius() - dist.magnitude()),
            ));
        }
        None
    }
    fn hit_bounding_box(&self, bounding_box: BoundingBox) -> Option<Hit> {
        let dist = Vector::from_points(bounding_box.center, self.center());
        if dist.dx.abs() <= bounding_box.half.w + self.radius()
            && dist.dy.abs() <= bounding_box.half.h + self.radius()
        {
            // possible collision
            if dist.dy.abs() <= bounding_box.half.h && dist.dx.abs() < bounding_box.half.w {
                // circle center is inside the box, select closest
                let sign_x = dist.dx.signum();
                let sign_y = dist.dy.signum();
                let adjust_x = Vector::new(sign_x * bounding_box.half.w, 0.0);
                let adjust_y = Vector::new(0.0, sign_y * bounding_box.half.h);
                let delta_x = adjust_x + Vector::new(self.radius() * sign_x - dist.dx, 0.0);
                let delta_y = adjust_y + Vector::new(0.0, self.radius() * sign_y - dist.dy);
                if delta_x.magnitude() < delta_y.magnitude() {
                    return Some(Hit::new(
                        Point::new(bounding_box.center.x, self.center().y) + adjust_x,
                        Vector::new(sign_x, 0.0),
                        delta_x,
                    ));
                } else {
                    return Some(Hit::new(
                        Point::new(self.center().x, bounding_box.center.y) + adjust_y,
                        Vector::new(0.0, sign_y),
                        delta_y,
                    ));
                }
            } else if dist.dy.abs() <= bounding_box.half.h {
                // circle is on the left/right side the box
                let sign_x = dist.dx.signum();
                let adjust = Vector::new(sign_x * bounding_box.half.w, 0.0);
                let delta = adjust + Vector::new(self.radius() * sign_x - dist.dx, 0.0);
                return Some(Hit::new(
                    Point::new(bounding_box.center.x, self.center().y) + adjust,
                    Vector::new(sign_x, 0.0),
                    delta,
                ));
            } else if dist.dx.abs() < bounding_box.half.w {
                // circle is on top/bottom the box
                let sign_y = dist.dy.signum();
                let adjust = Vector::new(0.0, sign_y * bounding_box.half.h);
                let delta = adjust + Vector::new(0.0, self.radius() * sign_y - dist.dy);
                return Some(Hit::new(
                    Point::new(self.center().x, bounding_box.center.y) + adjust,
                    Vector::new(0.0, sign_y),
                    delta,
                ));
            }
            let deepest_vertex = bounding_box.polygon().closest_point(self.center());
            let vertex_dist = Vector::from_points(deepest_vertex, self.center());
            if vertex_dist.magnitude() < self.radius() {
                // pushed out by the deepest vertex
                let delta = vertex_dist.get_unit_vector() * self.radius() - vertex_dist;
                return Some(Hit::new(deepest_vertex, delta.get_unit_vector(), delta));
            }
        }
        None
    }
    fn hit_circle(&self, circle: &Circle) -> Option<Hit> {
        let dist = Vector::from_points(circle.center(), self.center());
        if dist.magnitude() <= circle.radius() + self.radius() {
            let normal = dist.get_unit_vector();
            let contact = circle.center() + normal * circle.radius();
            return Some(Hit::new(
                contact,
                normal,
                normal * (circle.radius() + self.radius() - dist.magnitude()),
            ));
        }
        None
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
        // let vertices_inside_a = self.polygon().vertices_inside(&bounding_box.polygon());
        let vertices_inside_b = bounding_box.polygon().vertices_inside(&self.polygon());
        // TODO: does not work yet
        if let Some(indices) = vertices_inside_b {
            let mut best_distance = -std::f32::INFINITY;
            let mut best_point = Point::zero();
            let mut best_vertex = Point::zero();
            for &idx in indices.iter() {
                let vertex = self.polygon().vertices[idx];
                let canditate = bounding_box.polygon().closest_point(vertex);
                let distance = vertex.distance_to(canditate);
                if distance > best_distance {
                    best_distance = distance;
                    best_point = canditate;
                    best_vertex = vertex;
                }
            }
            let delta = Vector::from_points(best_point, best_vertex);
            return Some(Hit::new(best_point, delta.get_normal_vector(), delta));
        }
        None
    }
    fn hit_circle(&self, circle: &Circle) -> Option<Hit> {
        None
    }
}
