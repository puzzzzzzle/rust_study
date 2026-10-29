use futures::executor::block_on;

async fn hello(name : &str,_data:[u8;100]) -> String {
    format!("Hello, {}!", name)
}
async fn simple() -> String {
    "Hello, World!".to_string()
}
async fn run() {
    let name = "World";
    let data:[u8;100] = [0;100];
    // 数据直接存储在future中, 例如拷贝进去的data, 之将在future中分配内存
    // size of greeting: 120
    let greeting= hello(name,data);
    println!("size of greeting: {}",std::mem::size_of_val(&greeting));
    println!("{}",greeting.await);
    println!("size of simple: {}",std::mem::size_of_val(&simple()));
}

fn main() {
    block_on(run())
}
