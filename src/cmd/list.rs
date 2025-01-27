use colored::Colorize;

use crate::mc_dir;

pub fn list() -> anyhow::Result<()> {
    println!("{}", "Minecraft Instance".bold().italic());
    for path in mc_dir().read_dir()? {
        println!("  {:?}", path?.file_name())
    }
    Ok(())
}
