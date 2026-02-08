//! nano-ros: Standalone build tool for nano-ros
//!
//! This is the standalone binary that can be invoked directly as `nano-ros`.
//! It provides the same functionality as `cargo nano-ros` but without
//! requiring Cargo's subcommand infrastructure.

use cargo_nano_ros::GenerateConfig;
use clap::{Parser, Subcommand};
use eyre::Result;
use std::path::PathBuf;

/// Standalone build tool for nano-ros
#[derive(Debug, Parser)]
#[command(name = "nano-ros")]
#[command(about = "Build tool for nano-ros: generate message bindings from package.xml")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate Rust bindings from package.xml dependencies
    ///
    /// Reads package.xml to discover ROS 2 interface dependencies,
    /// resolves transitive dependencies, and generates nano-ros bindings.
    GenerateRust {
        /// Path to package.xml (default: ./package.xml)
        #[arg(long, default_value = "package.xml")]
        manifest_path: PathBuf,

        /// Output directory for generated bindings (default: ./generated)
        #[arg(long, short, default_value = "generated")]
        output: PathBuf,

        /// Generate .cargo/config.toml with [patch.crates-io] entries
        #[arg(long)]
        config: bool,

        /// Path to nano-ros crates directory (for config patches)
        /// If not specified, nano-ros crates will use crates.io (requires published crates)
        #[arg(long, conflicts_with = "nano_ros_git")]
        nano_ros_path: Option<PathBuf>,

        /// Use nano-ros git repository for config patches
        /// Generates [patch.crates-io] entries pointing to the nano-ros git repository
        #[arg(long, conflicts_with = "nano_ros_path")]
        nano_ros_git: bool,

        /// Overwrite existing bindings
        #[arg(long)]
        force: bool,
    },

    /// (Hidden) Backward-compatible alias for generate-rust
    #[command(hide = true)]
    Generate {
        #[arg(long, default_value = "package.xml")]
        manifest_path: PathBuf,
        #[arg(long, short, default_value = "generated")]
        output: PathBuf,
        #[arg(long)]
        config: bool,
        #[arg(long, conflicts_with = "nano_ros_git")]
        nano_ros_path: Option<PathBuf>,
        #[arg(long, conflicts_with = "nano_ros_path")]
        nano_ros_git: bool,
        #[arg(long)]
        force: bool,
    },

    /// Generate C bindings for interface files (.msg, .srv, .action)
    ///
    /// Generates C code for use with nano-ros-c library. Called by
    /// nano_ros_generate_interfaces() CMake function.
    GenerateC {
        /// Path to JSON arguments file
        #[arg(long)]
        args_file: PathBuf,
    },

    /// Generate bindings for a single ROS 2 package (low-level)
    Bindgen {
        /// ROS package name
        #[arg(long)]
        package: String,

        /// Output directory for generated bindings
        #[arg(long)]
        output: PathBuf,

        /// Direct path to package share directory (bypasses ament index)
        #[arg(long)]
        package_path: Option<PathBuf>,
    },

    /// Clean generated bindings
    Clean {
        /// Output directory to clean (default: ./generated)
        #[arg(long, short, default_value = "generated")]
        output: PathBuf,

        /// Also remove .cargo/config.toml patches
        #[arg(long)]
        config: bool,
    },
}

fn run_generate(
    manifest_path: PathBuf,
    output: PathBuf,
    config: bool,
    nano_ros_path: Option<PathBuf>,
    nano_ros_git: bool,
    force: bool,
    verbose: bool,
) -> Result<()> {
    let cfg = GenerateConfig {
        manifest_path,
        output_dir: output,
        generate_config: config,
        nano_ros_path,
        nano_ros_git,
        force,
        verbose,
    };
    cargo_nano_ros::generate_from_package_xml(cfg)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::GenerateRust {
            manifest_path,
            output,
            config,
            nano_ros_path,
            nano_ros_git,
            force,
        }
        | Command::Generate {
            manifest_path,
            output,
            config,
            nano_ros_path,
            nano_ros_git,
            force,
        } => {
            run_generate(
                manifest_path,
                output,
                config,
                nano_ros_path,
                nano_ros_git,
                force,
                cli.verbose,
            )?;
        }

        Command::GenerateC { args_file } => {
            let cfg = cargo_nano_ros::GenerateCConfig {
                args_file,
                verbose: cli.verbose,
            };
            cargo_nano_ros::generate_c_from_args_file(cfg)?;
            println!("✓ C bindings generated successfully");
        }

        Command::Bindgen {
            package,
            output,
            package_path,
        } => {
            let cfg = cargo_nano_ros::BindgenConfig {
                package_name: package,
                package_path,
                output_dir: output,
                verbose: cli.verbose,
            };
            cargo_nano_ros::generate_bindings(cfg)?;
            println!("✓ Bindings generated successfully");
        }

        Command::Clean { output, config } => {
            cargo_nano_ros::clean_generated(&output, config, cli.verbose)?;
            println!("✓ Cleaned successfully");
        }
    }

    Ok(())
}
