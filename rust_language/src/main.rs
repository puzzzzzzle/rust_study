use anyhow::{anyhow, Result};
use std::env;

// 所有的main函数开始的执行路径都在项目最根的cargo所在的目录下
fn main() -> Result<()> {
    println!("{}", env::current_dir()?.to_str().ok_or(anyhow!(""))?);
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use anyhow::{anyhow, Result};
    use std::env;
    #[test]
    // 所有单圈测试类的执行路径都在当前文件所在的cargo目录级别下
    fn test_init() -> Result<()> {
        println!("{}", env::current_dir()?.to_str().ok_or(anyhow!(""))?);
        Ok(())
    }
}
