//! cargo-nano-ros: Standalone build tool for nano-ros
//!
//! Generate ROS 2 message bindings from package.xml dependencies.

use cargo_nano_ros::GenerateConfig;
use clap::{Parser, Subcommand};
use eyre::Result;
use std::path::PathBuf;

/// Standalone build tool for nano-ros
#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    NanoRos(NanoRosArgs),
}

#[derive(Debug, Parser)]
#[command(name = "nano-ros")]
#[command(about = "Standalone build tool for nano-ros", long_about = None)]
struct NanoRosArgs {
    #[command(subcommand)]
    command: NanoRosCommand,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum NanoRosCommand {
    /// Generate Rust bindings from package.xml dependencies
    ///
    /// Reads package.xml to discover ROS 2 interface dependencies,
    /// resolves transitive dependencies, and generates nano-ros bindings.
    Generate {
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
        #[arg(long)]
        nano_ros_path: Option<PathBuf>,

        /// Overwrite existing bindings
        #[arg(long)]
        force: bool,
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

fn main() -> Result<()> {
    let CargoCli::NanoRos(args) = CargoCli::parse();

    match args.command {
        NanoRosCommand::Generate {
            manifest_path,
            output,
            config,
            nano_ros_path,
            force,
        } => {
            let cfg = GenerateConfig {
                manifest_path,
                output_dir: output,
                generate_config: config,
                nano_ros_path,
                force,
                verbose: args.verbose,
            };
            cargo_nano_ros::generate_from_package_xml(cfg)?;
        }

        NanoRosCommand::Bindgen {
            package,
            output,
            package_path,
        } => {
            let cfg = cargo_nano_ros::BindgenConfig {
                package_name: package,
                package_path,
                output_dir: output,
                verbose: args.verbose,
            };
            cargo_nano_ros::generate_bindings(cfg)?;
            println!("✓ Bindings generated successfully");
        }

        NanoRosCommand::Clean { output, config } => {
            cargo_nano_ros::clean_generated(&output, config, args.verbose)?;
            println!("✓ Cleaned successfully");
        }
    }

    Ok(())
}
