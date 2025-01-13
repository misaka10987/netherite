mod cmd;
mod mc;
mod module;
mod platform;
mod prelude;
mod registry;

use std::{fs::File, path::Path, process::Command, sync::LazyLock};

use anyhow::bail;
use chksum_md5::chksum;
use clap::Parser;
use cmd::Commands;
use pubgrub::{range::Range, version::SemanticVersion};
use registry::OP_MC_VERSION;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use url::Url;

pub use prelude::*;

fn to_pubgrub_ver(version: &Version) -> SemanticVersion {
    SemanticVersion::new(
        version.major as u32,
        version.minor as u32,
        version.patch as u32,
    )
}

fn to_semver(version: &SemanticVersion) -> Version {
    Version::parse(&format!("{version}")).unwrap()
}

fn to_pubgrub_range(req: &VersionReq) -> Range<SemanticVersion> {
    let mut rng = Range::any();
    for i in &req.comparators {
        let new = if let (Some(minor), Some(patch)) = (i.minor, i.patch) {
            let value = SemanticVersion::new(i.major as u32, minor as u32, patch as u32);
            match i.op {
                semver::Op::Exact => Range::exact(value),
                semver::Op::Greater => panic!("PubGrub solver does not support comparator >"),
                semver::Op::GreaterEq => Range::higher_than(value),
                semver::Op::Less => Range::strictly_lower_than(value),
                semver::Op::LessEq => panic!("PubGrub solver does not support comparator <="),
                semver::Op::Tilde => todo!(),
                semver::Op::Caret => todo!(),
                semver::Op::Wildcard => todo!(),
                _ => todo!(),
            }
        } else {
            todo!()
        };
        rng = rng.intersection(&new);
    }
    rng
}

pub fn download(url: Url, save: impl AsRef<Path>) -> anyhow::Result<()> {
    info!("begin downloading {url} with curl");
    let stat = Command::new("curl")
        .arg("-o")
        .arg(save.as_ref())
        .arg(url.as_str())
        .spawn()?
        .wait()?;
    if !stat.success() {
        bail!("download failure");
    }
    Ok(())
}

pub fn check_md5(file: impl AsRef<Path>, md5sum: &str) -> anyhow::Result<()> {
    info!("checking integrity for {:?}", file.as_ref());
    let sum = chksum(File::open(&file)?)?.to_hex_lowercase();
    if sum != md5sum.to_lowercase() {
        bail!(
            "incorrect checksum {sum} for {:?}, expecting {md5sum}",
            file.as_ref()
        );
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModAPI {
    #[serde(rename = "datapack")]
    DataPack,
    #[serde(rename = "forge")]
    Forge,
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "liteloader")]
    LiteLoader,
}

#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    /// The package registry to use.
    #[arg(long, default_value = "redis://127.0.0.1/")]
    pub registry: Url,
    /// How verbose the logger is going to be.
    #[arg(long, default_value = "TRACE", value_name = "LEVEL")]
    pub log_level: tracing::Level,
    #[command(subcommand)]
    pub cmd: Commands,
}

static ARG: LazyLock<Args> = LazyLock::new(|| Args::parse());

fn main() -> anyhow::Result<()> {
    let f = filter_fn(|x| match x.module_path() {
        Some(x) => !x.starts_with("mio::poll"),
        _ => false,
    });
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(f)
                .with_filter(LevelFilter::from(ARG.log_level)),
        )
        .init();
    *OP_MC_VERSION.write().unwrap() = VersionReq::STAR;
    ARG.cmd.exec()
}
