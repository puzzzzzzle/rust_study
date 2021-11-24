use anyhow;
use anyhow::Result;
use std::env;

pub fn get_run_dir(conf_relative_path: &str) -> Result<String> {
    let mut curr_path = env::current_dir().unwrap();
    loop {
        // 先看看当前路径下有没有配置文件
        let mut curr_check_conf = curr_path.clone();
        curr_check_conf.push(conf_relative_path);
        if curr_check_conf.as_path().exists() {
            // 找到了就返回
            break Ok(String::from(curr_path.as_os_str().to_str().unwrap()));
        }
        // 没找到就更新
        if curr_path.parent().is_some() {
            curr_path = curr_path.parent().unwrap().to_path_buf()
        } else {
            // 到头了, 返回失败吧
            break Err(anyhow::anyhow!("cannot find config file path"));
        };
    }
}
