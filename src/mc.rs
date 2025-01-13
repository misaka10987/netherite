use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self},
};

use dirs::home_dir;
use semver::Version;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::ModAPI;

#[derive(Clone, Serialize, Deserialize)]
pub struct MCInst {
    pub version: Version,
    pub api: HashMap<ModAPI, Version>,
    pub modlist: HashMap<String, Version>,
    pub comment: Option<String>,
}

impl MCInst {
}

/// Retrieve the minecraft instance description file `mcinst.toml` from all installations under `.minecraft` under home directory.
pub fn retrieve() -> anyhow::Result<HashMap<OsString, MCInst>> {
    let path = home_dir().unwrap().join(".minecraft").join("versions");
    let mut map = HashMap::new();
    for i in path.read_dir()? {
        let dir = i?;
        let name = dir.file_name();
        let path = dir.path().join("mcinst.toml");
        if !path.exists() {
            debug!("skip manually managed installation {:?}", dir.path());
            continue;
        }
        let s = fs::read_to_string(path)?;
        let value = toml::from_str(&s)?;
        map.insert(name, value);
    }
    Ok(map)
}
