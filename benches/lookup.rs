use criterion::{black_box, criterion_group, criterion_main, Criterion};

use epsg::references::{get_crs, get_name};

fn bench_get_crs(c: &mut Criterion) {
    c.bench_function("get_crs", |b| b.iter(|| get_crs(black_box("EPSG:4326"))));
}

fn bench_get_name(c: &mut Criterion) {
    c.bench_function("get_name", |b| b.iter(|| get_name(black_box("EPSG:4326"))));
}

criterion_group!(benches, bench_get_crs, bench_get_name);
criterion_main!(benches);
