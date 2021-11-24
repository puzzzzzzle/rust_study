use common;
use log::*;
use std::env;
fn main() {
    common::init_env().unwrap();
    info!("hello info");
    info!(
        "cwd {}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    );
}
