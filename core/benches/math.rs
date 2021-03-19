use criterion::{criterion_group, criterion_main, Criterion};
use mc_core::math;

static TO_TEST_ANGLES: [f32; 3] = [
    0.0,
    math::JAVA_PI as f32,
    math::JAVA_PI as f32 * 2.0
];

fn mc_sin_benchmark(c: &mut Criterion) {
    c.bench_function("mc_sin", |b| {
        for &angle in &TO_TEST_ANGLES {
            b.iter(|| math::mc_sin(angle));
        }
    });
}

fn mc_cos_benchmark(c: &mut Criterion) {
    c.bench_function("mc_cos", |b| {
        for &angle in &TO_TEST_ANGLES {
            b.iter(|| math::mc_cos(angle));
        }
    });
}

criterion_group!(benches, mc_sin_benchmark, mc_cos_benchmark);
criterion_main!(benches);
