use criterion::{criterion_group, criterion_main, Criterion};

use rust_ray_tracer_challenge::tuples;

fn bench_tuple(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tuples");
    let p = tuples::point(1.0, 2.0, 3.0);
    group.bench_function("tuple_multiply", |b| b.iter(|| p.multiply(2.0)));
    group.bench_function("tuple_multiply_borrow", |b| {
        b.iter(|| tuples::tuple_multiply_borrow(&p, &2.0))
    });
}

criterion_group!(benches, bench_tuple);
criterion_main!(benches);
