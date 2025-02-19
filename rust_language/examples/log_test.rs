use anyhow::{Result, anyhow, Context};
use common;
use log::*;
use std::env;
use std::path::PathBuf;
#[allow(unused)]
fn find_user(id: u32) -> Option<String> {
    // 模拟查找用户
    Some("Alice".to_string())
}

fn get_user(id: u32) -> Result<String> {
    find_user(id).context("用户不存在") // 将 None 转为错误
}
#[allow(unused)]
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
