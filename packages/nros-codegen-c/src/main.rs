//! nros-codegen: Generate C bindings for ROS 2 interface files.
//!
//! Usage: nros-codegen --args-file <path> [--verbose]

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "nros-codegen")]
#[command(about = "Generate C bindings for ROS 2 interface files")]
struct Cli {
    /// Path to the JSON arguments file
    #[arg(long)]
    args_file: PathBuf,

    /// Verbose output
    #[arg(long)]
    verbose: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let config = cargo_nano_ros::GenerateCConfig {
        args_file: cli.args_file,
        verbose: cli.verbose,
    };

    match cargo_nano_ros::generate_c_from_args_file(config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("nros-codegen: {e:#}");
            ExitCode::FAILURE
        }
    }
}
