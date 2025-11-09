# colcon-cargo-ros2: All-in-One ROS 2 Rust Build Tools

**A unified repository containing:**
- **Rust Workspace**: 4 crates for ROS 2 Rust binding generation (`cargo-ros2-bindgen`, `rosidl-parser`, `rosidl-codegen`, `rosidl-runtime-rs`)
- **Python Package**: `colcon-cargo-ros2` colcon extension for seamless workspace integration

Build Rust ROS 2 projects with automatic message binding generation, intelligent caching, and ament-compatible installation.

## Repository Structure

```
colcon-cargo-ros2/  (Standalone Hybrid Repository)
├── Cargo.toml                    # Rust workspace manifest
├── cargo-ros2-bindgen/           # Main binding generator CLI
├── rosidl-parser/                # ROS IDL parser
├── rosidl-codegen/               # Code generator with Askama templates
├── rosidl-runtime-rs/            # Shared runtime library with FFI bindings
│
├── colcon-cargo-ros2/            # Python package subdirectory
│   ├── colcon_cargo_ros2/        # Python module
│   ├── test/                     # Python tests
│   └── setup.py, setup.cfg       # Python package config
│
└── justfile                      # Build automation (Rust + Python)
```

## Installation

### Prerequisites

- Rust toolchain (stable)
- Python 3.8+ with pip
- ROS 2 (Humble, Iron, or Jazzy)
- colcon (for workspace builds)

### Install Both Components

```bash
# Install Rust binaries and Python package
just install

# Or install manually:
cargo install --path cargo-ros2-bindgen
pip3 install -e colcon-cargo-ros2/ --break-system-packages
```

### Install Individual Components

```bash
# Rust only
cargo install --path cargo-ros2-bindgen

# Python only
pip3 install -e colcon-cargo-ros2/ --break-system-packages
```

## Usage

### As a Colcon Extension

Packages need a `package.xml` in addition to `Cargo.toml`. You should see such packages classified as `ament_cargo` in the output of `colcon list`.

Simply list dependencies (other `ament_cargo` packages or message packages) in `Cargo.toml` and `package.xml` as if they were hosted on crates.io. The extension will:
- Discover ROS dependencies via ament_index
- Generate Rust bindings automatically
- Cache generated bindings for fast rebuilds
- Create `.cargo/config.toml` with proper patches
- Install to ament-compatible locations

```bash
# In a colcon workspace
colcon build --symlink-install

# Pass extra cargo arguments
colcon build --cargo-args --release

# After building, run binaries
ros2 run my_package my_binary
```

### As a Standalone Tool

You can use `cargo-ros2-bindgen` directly for binding generation:

```bash
# Generate bindings for a single package
cargo-ros2-bindgen --package std_msgs --output target/ros2_bindings

# With verbose output
cargo-ros2-bindgen --package geometry_msgs --output ./bindings --verbose

# Using direct path (bypasses ament index)
cargo-ros2-bindgen --package my_msgs --output ./out --package-path /path/to/share
```

## Features

- **Automatic Binding Generation**: Generates Rust bindings for all ROS message/service/action types on-demand
- **Smart Caching**: SHA256-based checksums for fast incremental builds
- **Workspace-Level Bindings**: In colcon workspaces, bindings are generated once and shared across all packages
- **Shared Runtime Library**: `rosidl_runtime_rs` provides FFI bindings and idiomatic Rust wrappers
- **Parallel Generation**: Multiple packages generate bindings in parallel using rayon
- **Ament Compatible**: Installs to standard ament locations for seamless ROS 2 integration
- **Progress Indicators**: Beautiful progress bars show generation and build status

## Development

### Quick Start

```bash
# Build Rust workspace
just build

# Run all tests
just test

# Format and lint
just format
just check

# Run full quality checks
just quality

# Note: Python packages don't need building for development.
# Use 'just install-python' to install in development mode.
# Use 'just build-python' only if you need distribution packages (requires 'python3 -m pip install build')
```

### Rust Development

```bash
# Build all Rust crates
cargo build --workspace
# or
just build-rust

# Run tests
cargo test --workspace
# or
just test-rust

# Lint
cargo clippy --workspace -- -D warnings
# or
just check-rust

# Format
cargo +nightly fmt
# or
just format-rust
```

### Python Development

```bash
# Install in development mode
pip3 install -e colcon-cargo-ros2/ --break-system-packages
# or
just install-python

# Run tests
pytest colcon-cargo-ros2/test/
# or
just test-python

# Lint
ruff check colcon-cargo-ros2/colcon_cargo_ros2/ colcon-cargo-ros2/test/
# or
just check-python

# Format
ruff format colcon-cargo-ros2/colcon_cargo_ros2/ colcon-cargo-ros2/test/
# or
just format-python
```

### Testing Changes

After modifying templates or code:

```bash
# Required after template changes
cargo clean && just install

# Test in a workspace
cd testing_workspaces/complex_workspace
rm -rf build install log
colcon build --symlink-install
```

## Documentation

- **CLAUDE.md** - Project instructions and architecture overview
- **docs/ARCH.md** - Detailed architecture documentation
- **docs/DESIGN.md** - Implementation details
- **docs/ROADMAP.md** - Development roadmap

## Architecture Highlights

### Two-Tool Design

1. **`cargo-ros2-bindgen`** - Low-level binding generator
   - Generates Rust bindings for a single ROS interface package
   - Can be used standalone
   - Pure Rust implementation with native IDL parser

2. **`colcon-cargo-ros2`** - High-level colcon integration
   - Wraps `cargo-ros2-bindgen` for workspace builds
   - Manages workspace-level binding cache
   - Handles ament-compatible installation

### Workspace-Level Binding Generation

In colcon workspaces, bindings are generated once at the workspace level:

```
workspace/
├── build/
│   ├── ros2_bindings/            # Generated once, shared by all packages
│   │   ├── std_msgs/
│   │   ├── geometry_msgs/
│   │   └── custom_interfaces/
├── src/
│   ├── robot_controller/
│   │   └── .cargo/config.toml    # Points to ../../build/ros2_bindings/
│   └── robot_driver/
│       └── .cargo/config.toml    # Also points to ../../build/ros2_bindings/
```

**Benefits:**
- No duplication of generated code
- Faster builds
- Smaller disk usage
- Follows ROS conventions

### Shared Runtime Library

`rosidl_runtime_rs` provides:
- FFI bindings to `rosidl_runtime_c`
- Idiomatic Rust wrappers for strings and sequences
- Core traits for messages, services, and actions
- Automatic memory management

This eliminates 100+ lines of duplicated code per generated package.

## Limitations

- `colcon test` is not yet fully supported (use `cargo test` directly)
- The quadratic build cost issue (Cargo rebuilding dependencies) is mitigated but not eliminated

## Contributing

### Code Quality Standards

- Run `just quality` before submitting changes
- All clippy warnings must be fixed (`-D warnings`)
- Code must be formatted with `cargo +nightly fmt`
- Python code must pass `ruff check` and `ruff format`
- All tests must pass

### Development Guidelines

1. **File Operations**: Always use Write/Edit tools, never bash commands (`cat`, `echo`, etc.)
2. **Temporary Files**: Create in `tmp/` directory
3. **Documentation**: Update CLAUDE.md and docs/ when changing architecture
4. **Testing**: Add tests for new features, ensure existing tests pass

## License

MIT OR Apache-2.0 (compatible with ROS 2 ecosystem)

## Related Projects

- **ros2_rust**: Current official Rust bindings (workspace-based approach)
- **r2r**: Alternative bindings (build.rs generation)
- **cargo-ament-build**: Ament layout installer (functionality being absorbed)

---

**Status**: Phase 3 Near Complete - Production Features (2025-11-07)
**Progress**: 14/20 subphases (70%) | 190+ tests passing | Zero warnings
**Latest**: Shared rosidl_runtime_rs ✅, Workspace-aware linking ✅, colcon integration fixed ✅
