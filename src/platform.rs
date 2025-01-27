use std::{fs, path::PathBuf};

use dirs::{data_dir, home_dir};
use semver::Version;

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

pub fn mod_storage(mod_id: &str, version: &Version) -> PathBuf {
    inst_dir()
        .join("mod")
        .join(mod_id)
        .join(version.to_string())
}

pub fn mc_dir() -> PathBuf {
    home_dir().unwrap().join(".minecraft").join("versions")
}
