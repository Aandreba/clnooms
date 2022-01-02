#![feature(div_duration)]
use std::{time::{Duration, Instant}};

#[derive(Debug)]
pub struct Benchmark {
    entries: Vec<Duration>,
    pub mean: Duration
}

pub fn benchmark<T, F: Fn() -> T> (warmup: Duration, bench: Duration, fun: F) -> Benchmark {
    // WARMUP
    println!("Wrming up for {:?}", warmup);
    let mut _warmup = Duration::ZERO;
    let mut count : u32 = 0;
    while _warmup <= warmup {
        _warmup += time(&fun);
        count += 1;
    }

    // RUN
    let delta = bench.div_duration_f64(_warmup);


    println!("Benchmarking for {:?} (approx. {} iters)", bench, (count as f64 * delta).round());

    let mut entries : Vec<Duration> = Vec::new();
    let mut _bench = Duration::ZERO;
    while _bench <= bench {
        let time = time(&fun);
        _bench += time;
        entries.push(time);
    }

    let mean = _bench / entries.len() as u32;
    Benchmark {
        entries,
        mean
    }
}

#[inline(always)]
pub fn time<T, F: Fn() -> T> (fun: &F) -> Duration {
    let start = Instant::now();
    black_box(fun());
    let end = Instant::now();

    end.duration_since(start)
}

pub fn black_box<T> (dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}