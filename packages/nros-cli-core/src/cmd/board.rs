//! `nros board list` — Phase 111.A.8.

use clap::Subcommand;
use eyre::{Result, eyre};

#[derive(Debug, Subcommand)]
pub enum Args {
    /// Enumerate every supported board (chip / flash / ram /
    /// supported_rmw)
    List,
}

pub fn run(args: Args) -> Result<()> {
    match args {
        Args::List => Err(eyre!(
            "`nros board list` is not implemented yet (Phase 111.A.8)."
        )),
    }
}
