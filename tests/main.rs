#![feature(div_duration)]
mod benchmark;
use std::time::Duration;

use benchmark::benchmark;
use clnooms::{extended::f80, half::f16};
use rand::random;
const WARMUP : Duration = Duration::from_secs(3);
const BENCH : Duration = Duration::from_secs(5);

#[test]
fn f16_add () {
    let alpha : f32 = random();
    let beta : f32 = random();

    let _alpha = f16::from(alpha);
    let _beta = f16::from(beta);

    let single = benchmark(WARMUP, BENCH, || alpha + beta);
    let half = benchmark(WARMUP, BENCH, || _alpha + _beta);
    
    println!("{:?}\n{:?}", single.mean, half.mean)
}

#[test]
fn f80_add () {
    let alpha : f64 = random();
    let beta : f64 = random();

    let _alpha = f80::from(alpha);
    let _beta = f80::from(beta);

    let double = benchmark(WARMUP, BENCH, || alpha + beta);
    let extended = benchmark(WARMUP, BENCH, || _alpha + _beta);
    
    println!("{:?}\n{:?}", double.mean, extended.mean)
}