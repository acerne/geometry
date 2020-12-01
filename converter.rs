use crate::geometry::base::Point;
use crate::geometry::shape::Polygon;
use ggez::*;

pub fn convert_to_point(point: &Point) -> mint::Point2<f32> {
    mint::Point2 {
        x: point.x,
        y: point.y,
    }
}

pub fn convert_to_points(polygon: &Polygon) -> Vec<mint::Point2<f32>> {
    let mut points = Vec::new();
    for vert in polygon.vertices.iter() {
        points.push(convert_to_point(vert));
    }
    points
}
