use std::{ptr::addr_of, arch::asm};

use clnooms::{extended::f80, half::f16};
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::random;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("f32 addition", |b| {
        let alpha : f32 = random();
        let beta : f32 = random();

        b.iter(|| alpha + beta)
    });
    
    c.bench_function("f16 addition", |b| {
        let alpha : f16 = random::<f32>().into();
        let beta : f16 = random::<f32>().into();

        b.iter(|| alpha + beta)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);