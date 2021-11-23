use log4rs;
use std::env;

#[derive(Debug)]
pub struct LogPath {
    path: String,
}

impl Default for LogPath {
    fn default() -> Self {
        let mut curr_path = dbg!(env::current_dir().unwrap());
        let conf_relative_path = "data/configure/log4rs.yaml";
        loop {
            // 先看看当前路径下有没有配置文件
            let mut curr_check_conf = curr_path.clone();
            curr_check_conf.push(conf_relative_path);
            if curr_check_conf.as_path().exists() {
                // 找到了就返回
                break LogPath {
                    path: String::from(dbg!(curr_check_conf.as_os_str().to_str().unwrap())),
                };
            }
            // 没找到就更新
            if curr_path.parent().is_some() {
                curr_path = curr_path.parent().unwrap().to_path_buf()
            } else {
                // 到头了, 返回失败吧
                break LogPath {
                    path: String::from(dbg!("")),
                };
            };
        }
    }
}

pub fn init_logger(path: LogPath) -> anyhow::Result<()> {
    log4rs::init_file(path.path, Default::default())?;
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use crate::init_logger;
    use log::*;
    use std::path::PathBuf;
    #[test]
    fn test_init() {
        init_logger::init_logger(Default::default()).unwrap();
        info!("log started!")
    }
}
