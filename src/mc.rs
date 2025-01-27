use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
    sync::{LazyLock, RwLock},
};

use semver::Version;
use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::ModAPI;

#[derive(Clone, Serialize, Deserialize)]
pub struct MCInst {
    pub version: Version,
    pub api: HashMap<ModAPI, Version>,
    pub modlist: HashMap<String, Version>,
    pub comment: Option<String>,
}

impl MCInst {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        trace!("reading minecraft instance at {path:?}",);
        let s = fs::read_to_string(path)?;
        let value: MCInst = toml::from_str(&s)?;
        let mut lock = OP_INST.write().unwrap();
        *lock = Some(value.clone());
        Ok(value)
    }
    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let s = toml::to_string(self)?;
        fs::write(path.as_ref(), s)?;
        Ok(())
    }
}

static OP_INST: LazyLock<RwLock<Option<MCInst>>> = LazyLock::new(|| RwLock::new(None));

/// The operating minecraft version.
/// This exists for the use of integration of PubGrub SAT solver.
pub fn op_inst() -> MCInst {
    OP_INST.read().unwrap().clone().unwrap()
}
