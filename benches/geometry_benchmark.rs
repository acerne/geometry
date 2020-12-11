use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::collision_functions::collisions,
    benchmarks::polygon_creation::polygons
);
