use anyhow::{anyhow, Result};
use common;
use log::*;

#[test]
fn handle_option_question_mark() -> Result<()> {
    common::init_env()?;
    let s: Option<String> = None;
    // s? 返回None, 不是Error
    // s?;
    assert!(s.ok_or(anyhow!("None")).is_err());
    Ok(())
}
#[test]
fn handle_option_match() -> Result<()> {
    common::init_env()?;
    let s: Option<String> = None;
    let ret = match s {
        None => -1,
        Some(_) => 0,
    };
    assert_eq!(ret, -1);
    Ok(())
}
// map 系列 如果有值, 调用f 后 自动包装Some
// 这回导致无法主动返回None
#[test]
fn handle_option_map() -> Result<()> {
    common::init_env()?;
    let s: Option<String> = Some("hello".to_string());
    // map系列无法主动返回None
    // let _no: Option<String> = s.map(|s| None);
    // 注意:closure 会移动 s 的所有权, 这里使用ref 转为借用
    let _len = s.as_ref().map(|s| s.len());
    let ret = s.as_ref().map(|s| s.clone() + " ...");
    assert!(ret.is_some());
    assert_eq!(ret.ok_or(anyhow!(""))?, "hello ...");
    Ok(())
}
// and_then 系列知识简单的调用, 不会自动包装
#[test]
fn handle_option_and_then() -> Result<()> {
    common::init_env()?;
    let mut s: Option<String> = Some("hello".to_string());
    let _none: Option<String> = s.as_ref().and_then(|_| None);
    let _len = s.as_ref().and_then(|para| Some(para.len()));

    let ret: Option<String> = s
        .as_mut()
        // String 的 Add 符号, 会直接在第一个string 中 调用 push_str, 必须获取多有权, 这里直接clone了, 在这之后和原来的没关系了
        .and_then(|para| Some(para.clone() + " ..."))
        .and_then(|_| None);
    // s 在这之后还可用
    info!("{}", s.as_ref().ok_or(anyhow!(""))?);
    assert_eq!(s.as_ref().ok_or(anyhow!(""))?, "hello");
    assert!(ret.is_none());

    // 直接使用s的值得话 会导致s被获取所有权
    let ret = s.and_then(|para| Some(para + " ..."));
    // s 在这之后不可用
    // info!("{}", s.as_ref().ok_or(anyhow!(""))?);     //ERROR
    // assert_eq!(s.as_ref().ok_or(anyhow!(""))?, "hello");     // ERROR
    info!("{}", ret.as_ref().ok_or(anyhow!(""))?);
    assert_eq!(ret.as_ref().ok_or(anyhow!(""))?, "hello ...");

    Ok(())
}
fn main() {}
