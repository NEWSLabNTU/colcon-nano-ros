//! `nros run` — build → flash → monitor. Phase 111.A.10.

use clap::Args as ClapArgs;
use eyre::{Result, eyre};

#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Named target/env to run (matches a `[env.<name>]` section in the
    /// project config). Optional when the project has only one target.
    #[arg(long)]
    pub env: Option<String>,
}

pub fn run(_args: Args) -> Result<()> {
    Err(eyre!("`nros run` is not implemented yet (Phase 111.A.10)."))
}
