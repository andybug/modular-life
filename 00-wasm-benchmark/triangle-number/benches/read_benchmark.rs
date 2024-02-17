use criterion::{criterion_group, criterion_main, Criterion};
use triangle_number::{create_triangle_number_series, sum_series};

pub fn criterion_benchmark(c: &mut Criterion) {
    const SIDE_LENGTH: u64 = 67_108_864;

    let series = create_triangle_number_series(SIDE_LENGTH);
    c.bench_function("read", |b| b.iter(|| sum_series(&series)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
