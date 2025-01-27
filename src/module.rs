use std::collections::HashMap;

use semver::VersionReq;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use url::Url;

use crate::{op_inst, ModAPI};

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
    pub api_dep: (ModAPI, VersionReq),
}

impl Module {
    pub fn check_inst(&self) -> bool {
        let inst = op_inst();
        self.mc_dep.matches(&inst.version)
            && inst
                .api
                .get(&self.api_dep.0)
                .filter(|x| self.api_dep.1.matches(x))
                .is_some()
    }
}
