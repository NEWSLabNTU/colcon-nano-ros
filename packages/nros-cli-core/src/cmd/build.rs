//! `nros build` — Phase 111.A.9.

use clap::Args as ClapArgs;
use eyre::{Result, eyre};

#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Cargo / cmake / west arguments forwarded verbatim to the
    /// underlying tool.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub passthrough: Vec<String>,
}

pub fn run(_args: Args) -> Result<()> {
    Err(eyre!("`nros build` is not implemented yet (Phase 111.A.9)."))
}
