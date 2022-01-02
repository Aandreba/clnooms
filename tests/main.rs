#![feature(div_duration)]
mod benchmark;
use std::time::Duration;

use benchmark::benchmark;
use clnooms::extended::f80;
use rand::random;
const WARMUP : Duration = Duration::from_secs(3);
const BENCH : Duration = Duration::from_secs(5);

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