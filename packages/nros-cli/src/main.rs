//! The `nros` standalone binary — Phase 111.A.2.
//!
//! Pure clap dispatch shell. All real work lives in `nros-cli-core` so
//! the `cargo nano-ros` adapter can share it byte-for-byte.

use clap::Parser;
use eyre::Result;
use nros_cli_core::cmd::Cmd;

#[derive(Parser, Debug)]
#[command(
    name = "nros",
    about = "nano-ros user CLI",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    nros_cli_core::run(cli.command)
}
