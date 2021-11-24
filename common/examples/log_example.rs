use common::init_logger::init_logger;
use log::*;
fn main() {
    init_logger(Default::default()).unwrap();
    info!("log started")
}
