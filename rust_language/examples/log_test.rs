use anyhow::{Result, anyhow};
use common;
use log::*;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    common::init_env()?;
    info!("hello info");
    info!(
        "cwd {}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    );
    let curr_dir = env::current_dir()
        .unwrap_or(PathBuf::from("get cwd fail"))
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow!("cannot convert to str"))?
        .to_string();
    info!("cwd {curr_dir}");
    Ok(())
}
