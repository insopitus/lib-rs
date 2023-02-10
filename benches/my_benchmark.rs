use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

use lib_rs::base64::base64_encode;

fn criterion_benchmark(c: &mut Criterion) {
    let data = fs::read("./assets/img.jpg").unwrap();
    c.bench_function("base64 encode", |b| b.iter(|| base64_encode(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);