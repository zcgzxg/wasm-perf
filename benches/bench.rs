use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn b64_100_1000(c: &mut Criterion) {
    c.bench_function("b64_100_1000", |b| {
        b.iter(|| b64_bench::bench(black_box(100), 1000))
    });
}

fn b64_10000_1000(c: &mut Criterion) {
    c.bench_function("b64_10000_1000", |b| {
        b.iter(|| b64_bench::bench(black_box(10000), 1000))
    });
}

criterion_group!(bench, b64_100_1000, b64_10000_1000);
criterion_main!(bench);
