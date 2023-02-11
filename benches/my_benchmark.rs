use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};


use lib_rs::base64::base64_encode;

fn criterion_benchmark(c: &mut Criterion) {
    let data = fs::read("./assets/img.jpg").unwrap();
    c.bench_function("base64 encode", |b| b.iter(|| base64_encode(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);