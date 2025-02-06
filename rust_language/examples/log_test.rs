use common;
use log::*;
use std::env;
use anyhow::{Result, anyhow};

fn main() -> Result<(), > {
    common::init_env()?;
    info!("hello info");
    info!(
        "cwd {}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    );
    let curr_dir = env::current_dir()?
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow!("cannot convert to str"))?
        .to_string();
    info!("cwd {curr_dir}" );
    Ok(())
}
