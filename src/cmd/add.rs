use std::process::exit;

use anyhow::bail;
use semver::Version;
use tracing::debug;

use crate::{check_md5, download, inst_dir, registry::Reg, REGISTRY};

pub fn add(mod_id: &str, version: &Option<Version>) -> anyhow::Result<()> {
    let all = match REGISTRY.query(mod_id) {
        Some(x) => x,
        None => bail!("mod {mod_id} not found in registry"),
    };
    let v = match version {
        Some(v) => v.clone(),
        None => inquire::Select::new(
            &format!("Select a version of {mod_id}:"),
            all.keys().rev().collect(),
        )
        .prompt()?
        .clone(),
    };
    let info = match all.get(&v) {
        Some(x) => x,
        None => bail!("version {v} for {mod_id} not found in registry"),
    };
    if info.url.is_empty() {
        bail!("no download URL for {mod_id}")
    }
    let save = inst_dir().join("mod").join(&info.md5);
    if save.exists() {
        debug!("found {save:?}");
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
            &format!("Select a download source for {mod_id} {v}"),
            info.url.clone(),
        )
        .prompt()?;
        download(url, &save)?;
    }
    check_md5(&save, &info.md5)?;
    Ok(())
}
