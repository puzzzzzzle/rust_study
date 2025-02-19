use anyhow::{Result, anyhow};
use reqwest;
use tokio::time::{Duration, sleep};

async fn get_url(url: &str) -> Result<String> {
    println!("start req:{}", url);
    // 发送GET请求
    let response = reqwest::get(url).await?;

    // 确保请求成功
    if response.status().is_success() {
        println!("got url end");
        // 获取响应文本
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(anyhow!(
            "got fail: {}, {:?}",
            response.status(),
            response.error_for_status()
        ))
    }
}
// 一个异步函数，模拟一个耗时操作
async fn do_something(time: u64){
    println!("start... time: {}", time);
    sleep(Duration::from_secs(time)).await; // 模拟一个耗时2秒的操作
    println!("end! time: {}", time);
}
async fn do_something_ret(time: u64) -> u64 {
    println!("start... time: {}", time);
    sleep(Duration::from_secs(time)).await; // 模拟一个耗时2秒的操作
    println!("end! time: {}", time);
    time
}
#[tokio::main]
async fn main() {
    // 启动多个异步任务
    let task1 = do_something(1);
    let task2 = do_something_ret(2);
    let get_url_task = get_url("https://www.baidu.com/");

    // 并行执行任务
    // 有结果的都在这里,
    let (_,ret2,url_ret) = tokio::join!(task1, task2, get_url_task);

    println!("get url result: {}", ret2);
    println!("do_something_ret: {:?}", url_ret);

    println!("all task done!");
}
