// 开始test功能
#![feature(test)]
// 注意: 目前只有nightly版本才能使用bench

// 内部crate需要extern后才能使用
extern crate test;
use test::Bencher;

use common;
use log::*;

#[bench]
fn bench_log(b: &mut Bencher) {
    common::init_env().unwrap();
    info!("start bench log");
    b.iter(|| {
        info!("one log ");
    });
}
