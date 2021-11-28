use anyhow::Result;
use common;
use log::*;
use std::env;
fn main() -> Result<()> {
    common::init_env()?;
    info!("hello info");
    info!(
        "cwd {}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    );
    Ok(())
}
