use criterion::{criterion_group, criterion_main, Criterion};
use faststr::FastStr;
use std::{hint::black_box, ptr, time::Duration};

fn criterion_benchmark(c: &mut Criterion) {
    // batch test
    let s = vec![FastStr::empty(); 64];
    c.bench_function("batch empty faststr", |b| {
        // b.iter(|| black_box(vec_faststr_clone(&s)))
        b.iter_custom(|iters| iter_cunstom(iters, &s))
    });

    let s = vec![FastStr::from("Hello, world!"); 64];
    c.bench_function("batch static faststr", |b| {
        // b.iter(|| black_box(vec_faststr_clone(&s)))
        b.iter_custom(|iters| iter_cunstom(iters, &s))
    });

    #[allow(deprecated)]
    let s = vec![FastStr::new_inline("Hello, world!"); 64];
    c.bench_function("batch inline faststr", |b| {
        // b.iter(|| black_box(vec_faststr_clone(&s)))
        b.iter_custom(|iters| iter_cunstom(iters, &s))
    });
}

fn iter_cunstom(iters: u64, ss: &Vec<FastStr>) -> Duration {
    let mut total = std::time::Duration::new(0, 0);

    for _ in 0..iters {
        flush_cache();

        let start = std::time::Instant::now();
        vec_faststr_clone(ss);
        total += start.elapsed();
    }

    total
}

fn vec_faststr_clone(ss: &Vec<FastStr>) {
    let cloned: Vec<FastStr> = ss.iter().map(|item| black_box(item.clone())).collect();
    black_box(cloned);
}

pub fn flush_cache() {
    const SIZE: usize = 12 * 1024 * 1024;
    let buffer = vec![0u8; SIZE];
    let ptr = buffer.as_ptr();

    unsafe {
        for i in (0..SIZE).step_by(64) {
            let _ = ptr::read_volatile(ptr.add(i));
        }

        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
