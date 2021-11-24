use crate::get_run_dir::get_run_dir;
use log4rs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LogConfName {
    pub name: String,
}
impl Default for LogConfName {
    fn default() -> Self {
        let conf_relative_path = "data/configure/log4rs.yaml";
        LogConfName {
            name: conf_relative_path.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct LogPath {
    pub path: Option<String>,
}
impl Default for LogPath {
    fn default() -> Self {
        match get_run_dir(LogConfName::default().name.as_str()) {
            Ok(run_dir) => {
                let mut p = PathBuf::from(run_dir);
                p.push(LogConfName::default().name);
                LogPath {
                    path: Some(p.as_os_str().to_str().unwrap().to_string()),
                }
            }
            Err(_) => LogPath { path: None },
        }
    }
}

pub fn init_logger(path: String) -> anyhow::Result<()> {
    log4rs::init_file(path, Default::default())?;
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
