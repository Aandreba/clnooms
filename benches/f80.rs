use std::{ptr::addr_of, arch::asm};

use clnooms::extended::f80;
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::random;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("f80 addition", |b| {
        let alpha : f80 = random::<f64>().into();
        let beta : f80 = random::<f64>().into();

        unsafe {
            b.iter(|| {
                let ptr1 = addr_of!(alpha.0);
                let ptr2 = addr_of!(beta.0);
                let mut out = &[0u8;10] as *const [u8;10];

                asm!(
                    "fld tbyte ptr [{0}]",
                    "fld tbyte ptr [{1}]",
                    "faddp st(1), st",
                    "fstp tbyte ptr [{2}]",
                    in(reg) ptr1,
                    in(reg) ptr2,
                    out(reg) out
                );

                f80(*out)
            })
        }
    });

    c.bench_function("f64 addition", |b| {
        let alpha : f64 = random();
        let beta : f64 = random();

        b.iter(|| alpha + beta)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);