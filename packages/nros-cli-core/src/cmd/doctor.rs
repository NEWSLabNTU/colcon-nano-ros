//! `nros doctor` — Phase 111.A.7. Aggregates per-platform doctors.

use clap::Args as ClapArgs;
use eyre::{Result, eyre};

#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Restrict the check to one platform (`zephyr`, `freertos`, …)
    #[arg(long)]
    pub platform: Option<String>,
}

pub fn run(_args: Args) -> Result<()> {
    Err(eyre!(
        "`nros doctor` is not implemented yet (Phase 111.A.7). \
         Use `just doctor` or the per-platform `packages/scripts/doctor-*.sh` \
         scripts for now."
    ))
}
