use advent_of_code::day10_expand;
use criterion::{Criterion, criterion_group, criterion_main};

use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 10 expand", |b| {
        b.iter(|| day10_expand(black_box("1321131112")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
