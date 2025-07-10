use criterion::{criterion_group, criterion_main, Criterion};
use log::info;

fn log_info() {
    info!("one log ");
}
fn bench_log(c: &mut Criterion) {
    common::init_env().unwrap();

    info!("start bench log");
    c.bench_function("bench_log", |b| b.iter(|| log_info()));
}

criterion_group!(benches, bench_log);
criterion_main!(benches);
