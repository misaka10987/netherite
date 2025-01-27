mod add;
mod install;
mod list;
mod prelude;
mod remove;

use clap::Subcommand;
use colored::Colorize;
use semver::Version;
use std::path::PathBuf;

pub use prelude::*;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all installed packages.
    List,
    /// Download a package to the local storage.
    Add {
        /// Name of the package.
        #[arg(value_name = "MODID")]
        mod_id: String,
        /// The version of package to install, would prompt later if not supplied.
        #[arg(short, long)]
        version: Option<Version>,
    },
    /// Remove a package from the local storage.
    #[command(name = "remove", visible_alias = "rm")]
    Remove {
        /// Name of the package.
        #[arg(value_name = "MODID")]
        mod_id: String,
        /// The version of package to remove, remove all versions if not supplied.
        #[arg(short, long)]
        version: Option<Version>,
    },
    /// Install a package to a minecraft instance.
    Install {
        /// Name ot the package.
        #[arg(value_name = "MODID")]
        mod_id: String,
        #[arg(value_name = "MCINST", default_value = ".")]
        mc_inst: PathBuf,
    },
}

impl Commands {
    pub fn exec(&self) -> anyhow::Result<()> {
        match self {
            Commands::List => list(),
            Commands::Add { mod_id, version } => add(mod_id, version),
            Commands::Remove { mod_id, version } => remove(mod_id, version),
            Commands::Install { mod_id, mc_inst } => install(mod_id, Some(mc_inst)),
        }?;
        println!("{}", "Success".bold().green());
        Ok(())
    }
}
