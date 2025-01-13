mod add;
mod remove;

use add::add;
use clap::Subcommand;
use colored::Colorize;
use remove::remove;
use semver::Version;

use crate::mc;

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
}

impl Commands {
    pub fn exec(&self) -> anyhow::Result<()> {
        match self {
            Commands::List => list(),
            Commands::Add { mod_id, version } => add(mod_id, version),
            Commands::Remove { mod_id, version } => remove(mod_id, version),
        }
    }
}

fn list() -> anyhow::Result<()> {
    println!("{}", "Minecraft Installations:".bold().italic());
    for (name, inst) in mc::retrieve()? {
        println!("  {name:?} - {}", inst.version);
    }
    Ok(())
}
