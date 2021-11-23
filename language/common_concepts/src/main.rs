use std::env;

// 所有的main函数开始的执行路径都在项目最根的cargo所在的目录下
fn main() {
    println!(
        "{}",
        env::current_dir().unwrap().as_os_str().to_str().unwrap()
    );
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::env;
    #[test]
    // 所有单圈测试类的执行路径都在当前文件所在的cargo目录级别下
    fn test_init() {
        println!(
            "{}",
            env::current_dir().unwrap().as_os_str().to_str().unwrap()
        );
    }
}
