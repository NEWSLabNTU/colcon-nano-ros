//! `nros new <name>` — scaffold a new nano-ros project.
//!
//! Phase 111.A.4. Skeleton for now; template expansion lands once the
//! `templates/` tree is in place.

use clap::Args as ClapArgs;
use eyre::{Result, eyre};
use std::path::PathBuf;

#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Project directory to create
    pub name: PathBuf,

    /// Target platform
    #[arg(long, value_parser = ["freertos", "nuttx", "threadx", "zephyr", "esp32", "posix", "baremetal"])]
    pub platform: String,

    /// RMW backend
    #[arg(long, value_parser = ["zenoh", "xrce", "dds"], default_value = "zenoh")]
    pub rmw: String,

    /// Source language
    #[arg(long, value_parser = ["rust", "c", "cpp"], default_value = "rust")]
    pub lang: String,

    /// Use case template
    #[arg(long = "use-case", value_parser = ["talker", "listener", "service", "action"], default_value = "talker")]
    pub use_case: String,

    /// Overwrite an existing directory
    #[arg(long)]
    pub force: bool,
}

pub fn run(_args: Args) -> Result<()> {
    Err(eyre!(
        "`nros new` is not implemented yet (Phase 111.A.4). \
         For now, copy an example from `examples/` as a starting point."
    ))
}
