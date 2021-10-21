use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;

async fn hello() -> String {
    String::from("hello co")
}
async fn hello_1() -> String {
    String::from("hello_1 co")
}
async fn hello_1_2() -> String {
    let s1 = hello().await;
    let s2 = hello_1().await;
    String::from(format!("hello_1_2 co : ({} {})", s1, s2))
}
async fn hello_latter() -> String {
    sleep(Duration::from_secs(1));
    String::from("hello_latter co")
}
#[test]
fn test_1() -> () {
    let future = hello();
    let ret = block_on(future);
    println!("{}", ret);
    let ret = block_on(hello_1_2());
    println!("{}", ret);
    let ret = block_on(hello_latter());
    println!("{}", ret);
}
