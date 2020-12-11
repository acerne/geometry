use criterion::{black_box, criterion_group, Criterion};
use geometry::base::*;
use geometry::shape::*;

pub fn hexagon_modification_benchmark(c: &mut Criterion) {
    let mut hex = Hexagon::new(Point::new(200.0, 100.0), 100.0, Angle::zero());

    c.bench_function("hexagon modification", |b| {
        b.iter(|| modify_hexagon(black_box(&mut hex)))
    });
}

pub fn hexagon_polygon_access_benchmark(c: &mut Criterion) {
    let hex = Hexagon::new(Point::new(200.0, 100.0), 100.0, Angle::zero());

    c.bench_function("hexagon polygon access", |b| {
        b.iter(|| access_hexagon_polygon(black_box(&hex)))
    });
}

pub fn modify_hexagon(hex: &mut Hexagon) {
    let rot = Angle::new(1.0);
    for _i in 0..100 {
        hex.rotate(rot);
    }
}

pub fn access_hexagon_polygon(hex: &Hexagon) {
    for _i in 0..100 {
        let polygon = hex.polygon();
        let _point = polygon.vertices.first();
    }
}

criterion_group!(
    polygons,
    hexagon_modification_benchmark,
    hexagon_polygon_access_benchmark
);
