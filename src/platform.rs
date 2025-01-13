use std::{fs, path::PathBuf};

use dirs::data_dir;

pub fn previliged() -> bool {
    #[cfg(unix)]
    return whoami::username() == "root";
    #[cfg(windows)]
    return false;
}

pub fn inst_dir() -> PathBuf {
    if previliged() {
        #[cfg(unix)]
        return "/opt/netherite".into();
        #[cfg(windows)]
        return "C:\\Program Files\\netherite".into();
    }
    let dir = data_dir().unwrap().join("netherite");
    fs::create_dir_all(&dir).unwrap();
    dir
}
