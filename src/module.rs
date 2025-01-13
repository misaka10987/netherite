use std::collections::HashMap;

use semver::VersionReq;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use url::Url;

use crate::mc::MCInst;

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub spdx: spdx::Expression,
    pub url: Vec<Url>,
    pub md5: String,
    #[serde(default)]
    pub dep: HashMap<String, VersionReq>,
    #[serde(default, rename = "minecraft")]
    pub mc_dep: VersionReq,
}

impl Module {
    pub fn check_mc(&self, inst: MCInst) -> bool {
        return false;
    }
}
