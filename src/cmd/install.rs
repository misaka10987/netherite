use std::{ops::Deref, path::Path};

use anyhow::bail;
use pubgrub::{solver::resolve, version::SemanticVersion};
use symlink::symlink_file;
use tracing::info;

use crate::{mc_dir, mod_storage, to_semver, MCInst, REGISTRY};

use super::add;

pub fn install(mod_id: &str, mc_inst: Option<impl AsRef<Path>>) -> anyhow::Result<()> {
    let path = match mc_inst {
        Some(x) => x.as_ref().to_owned(),
        None => {
            let insts = mc_dir()
                .read_dir()
                .unwrap()
                .map(|x| x.unwrap().file_name().to_str().unwrap().to_owned())
                .collect();
            inquire::Select::new(
                &format!("Select a minecraft instance to install {mod_id}:"),
                insts,
            )
            .prompt()?
            .into()
        }
    };
    let mut inst = MCInst::open(&path)?;
    if inst.modlist.contains_key(mod_id) {
        info!("{mod_id} is already installed for {path:?}");
        return Ok(());
    }
    let sol = resolve(
        REGISTRY.deref(),
        mod_id.into(),
        SemanticVersion::new(1, 2, 3),
    );
    let sol = match sol {
        Ok(x) => x,
        Err(e) => bail!("{e:?}"),
    };
    for (mod_id, version) in &sol {
        let version = to_semver(&version);
        add(&mod_id, &Some(version.clone()))?;
        symlink_file(
            mod_storage(&mod_id, &version),
            path.join("mods").join(mod_id),
        )?;
    }
    inst.modlist
        .insert(mod_id.into(), to_semver(sol.get(mod_id).unwrap()));
    inst.save(&path)?;
    Ok(())
}
