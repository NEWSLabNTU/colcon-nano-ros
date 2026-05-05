//! `nros version` — Phase 111.A.12.

use eyre::Result;

pub fn run() -> Result<()> {
    println!("nros {}", env!("CARGO_PKG_VERSION"));
    println!("cargo-nano-ros {}", cargo_nano_ros_version());
    Ok(())
}

fn cargo_nano_ros_version() -> &'static str {
    // `cargo-nano-ros` shares the workspace version, so until it exposes a
    // public constant we read the same env var the workspace populates.
    env!("CARGO_PKG_VERSION")
}
