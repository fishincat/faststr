use criterion::{criterion_group, criterion_main, Criterion};
use faststr::FastStr;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let s = FastStr::empty();
    c.bench_function("quick empty faststr", |b| b.iter(|| black_box(s.clone())));

    let s = FastStr::from("Hello, world!");
    c.bench_function("quick static faststr", |b| b.iter(|| black_box(s.clone())));

    #[allow(deprecated)]
    let s = FastStr::new_inline("Hello, world!");
    c.bench_function("quick inline faststr", |b| b.iter(|| black_box(s.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
