use common;
use log::*;
use std::env;

fn main() {
    common::init_logger::init_logger(Default::default()).unwrap();
    info!("hello info");
    info!(
        "cwd {}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    );
}
