pub mod get_run_dir;
pub mod init_logger;

use crate::get_run_dir::get_run_dir;
use crate::init_logger::{LogConfName, LogPath};
use anyhow::Result;
use std::env;

pub fn init_env() -> Result<()> {
    let run_dir = get_run_dir(LogConfName::default().name.as_str())?;
    env::set_current_dir(run_dir)?;
    init_logger::init_logger(LogPath::default().path.unwrap())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use log::*;
    #[test]
    fn it_works() {
        crate::init_env().unwrap();
        info!("started");
        assert_eq!(2 + 2, 4);
    }
}
