use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
fn bench_function_2(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

// 下面这两行是必须的
criterion_group!(benches, bench_function_2);
criterion_main!(benches);