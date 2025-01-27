use std::process::exit;

use anyhow::bail;
use semver::Version;
use tracing::debug;

use crate::{check_md5, download, mod_storage, registry::Reg, REGISTRY};

pub fn add(mod_id: &str, version: &Option<Version>) -> anyhow::Result<()> {
    let all = match REGISTRY.query(mod_id) {
        Some(x) => x,
        None => bail!("mod {mod_id} not found in registry"),
    };
    let version = match version {
        Some(v) => v.clone(),
        None => inquire::Select::new(
            &format!("Select a version of {mod_id}:"),
            all.keys().rev().collect(),
        )
        .prompt()?
        .clone(),
    };
    let info = match all.get(&version) {
        Some(x) => x,
        None => bail!("version {version} for {mod_id} not found in registry"),
    };
    if info.url.is_empty() {
        bail!("no download URL for {mod_id}")
    }
    let path = mod_storage(mod_id, &version);
    if path.exists() {
        debug!("found {path:?}");
    } else {
        let accept = inquire::Confirm::new(&format!(
            "Mod {mod_id} is licensed under {}, accept?",
            info.spdx
        ))
        .prompt()?;
        if !accept {
            exit(0)
        }
        let url = inquire::Select::new(
            &format!("Select a download source for {mod_id} {version}:"),
            info.url.clone(),
        )
        .prompt()?;
        download(url, &path)?;
    }
    check_md5(&path, &info.md5)?;
    Ok(())
}
