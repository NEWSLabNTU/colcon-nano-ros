//! nros-codegen: Generate C or C++ bindings for ROS 2 interface files.
//!
//! Usage: nros-codegen --args-file <path> [--language c|cpp] [--verbose]

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "nros-codegen")]
#[command(about = "Generate C or C++ bindings for ROS 2 interface files")]
struct Cli {
    /// Path to the JSON arguments file
    #[arg(long)]
    args_file: PathBuf,

    /// Target language: "c" (default) or "cpp"
    #[arg(long, default_value = "c")]
    language: String,

    /// Verbose output
    #[arg(long)]
    verbose: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.language.as_str() {
        "c" => {
            let config = cargo_nano_ros::GenerateCConfig {
                args_file: cli.args_file,
                verbose: cli.verbose,
            };
            cargo_nano_ros::generate_c_from_args_file(config)
        }
        "cpp" => {
            let config = cargo_nano_ros::GenerateCppConfig {
                args_file: cli.args_file,
                verbose: cli.verbose,
            };
            cargo_nano_ros::generate_cpp_from_args_file(config)
        }
        other => {
            eprintln!("nros-codegen: unsupported language '{other}' (expected 'c' or 'cpp')");
            return ExitCode::FAILURE;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("nros-codegen: {e:#}");
            ExitCode::FAILURE
        }
    }
}
