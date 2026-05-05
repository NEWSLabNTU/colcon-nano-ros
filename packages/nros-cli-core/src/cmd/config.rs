//! `nros config show` / `nros config check` — Phase 111.A.6.

use clap::Subcommand;
use eyre::{Result, eyre};

#[derive(Debug, Subcommand)]
pub enum Args {
    /// Print the resolved configuration (config.toml + Kconfig + Cargo
    /// features merged into one view)
    Show,
    /// Validate the configuration: catches mismatched RMW × platform ×
    /// ROS edition combinations and missing required keys.
    Check,
}

pub fn run(args: Args) -> Result<()> {
    match args {
        Args::Show => Err(eyre!(
            "`nros config show` is not implemented yet (Phase 111.A.6)."
        )),
        Args::Check => Err(eyre!(
            "`nros config check` is not implemented yet (Phase 111.A.6)."
        )),
    }
}
