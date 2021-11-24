#![feature(test)]

#[cfg(test)]
mod tests {
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
}
