use criterion::{black_box, criterion_group, criterion_main, Criterion};
use triangle_number::create_triangle_number_series;

pub fn criterion_benchmark(c: &mut Criterion) {
    const SIDE_LENGTH: u64 = 67_108_864;

    c.bench_function("write", |b| b.iter(|| create_triangle_number_series(black_box(SIDE_LENGTH))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
