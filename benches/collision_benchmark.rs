use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geometry::base::*;
use geometry::collision::*;
use geometry::shape::*;

pub fn circle_circle_collision_benchmark(c: &mut Criterion) {
    let shape_a = Circle::new(Point::new(200.0, 100.0), 100.0);
    let shape_b = Circle::new(Point::new(500.0, -200.0), 50.0);

    c.bench_function("circle-circle collision detection", |b| {
        b.iter(|| detection::distance_closest_points(black_box(&shape_a), black_box(&shape_b)))
    });
}

pub fn rectangle_rectangle_collision_benchmark(c: &mut Criterion) {
    let shape_a = Rectangle::new(
        Point::new(200.0, 100.0),
        Size::new(100.0, 20.0),
        Angle::zero(),
    );
    let shape_b = Rectangle::new(
        Point::new(500.0, -200.0),
        Size::new(50.0, 200.0),
        Angle::zero(),
    );

    c.bench_function("rectangle-rectangle collision detection", |b| {
        b.iter(|| detection::distance_closest_points(black_box(&shape_a), black_box(&shape_b)))
    });
}

pub fn hexagon_hexagon_collision_benchmark(c: &mut Criterion) {
    let shape_a = Hexagon::new(Point::new(200.0, 100.0), 100.0, Angle::zero());
    let shape_b = Hexagon::new(Point::new(500.0, -200.0), 50.0, Angle::zero());

    c.bench_function("hexagon-hexagon collision detection", |b| {
        b.iter(|| detection::distance_closest_points(black_box(&shape_a), black_box(&shape_b)))
    });
}

criterion_group!(
    benches,
    circle_circle_collision_benchmark,
    hexagon_hexagon_collision_benchmark,
    rectangle_rectangle_collision_benchmark
);
criterion_main!(benches);
