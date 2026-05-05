//! `nros monitor` — Phase 111.A.11.

use clap::Args as ClapArgs;
use eyre::{Result, eyre};

#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Named target/env (matches `nros run --env`)
    #[arg(long)]
    pub env: Option<String>,
}

pub fn run(_args: Args) -> Result<()> {
    Err(eyre!(
        "`nros monitor` is not implemented yet (Phase 111.A.11)."
    ))
}
