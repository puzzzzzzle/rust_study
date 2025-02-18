use anyhow::{Result, anyhow};
use common;
use log::*;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    common::init_env()?;
    info!("hello info");
    let curr_dir = env::current_dir()
        .unwrap_or(PathBuf::from("get cwd fail"))
        .to_str()
        .unwrap_or("get cwd fail")
        .to_string(); // PathBuf::from("get cwd fail") 是临时变量, 必须最终转火所有权
    let curr_dir = env::current_dir()
        .map(|p| p.to_string_lossy().into_owned()) // 自动处理非 UTF-8 路径
        .unwrap_or_else(|_| "get_cwd_failed".to_string());
    info!("cwd {curr_dir}");
    Ok(())
}
