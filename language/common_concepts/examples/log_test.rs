use log::*;
use log4rs;
use std::env;

fn main() {
    log4rs::init_file("data/configure/log4rs.yaml", Default::default()).unwrap();
    info!("hello info");
    println!(
        "{}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    )
}
