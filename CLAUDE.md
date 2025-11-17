# colcon-cargo-ros2: Development Guide

**Build Rust ROS 2 packages with automatic message binding generation.**

This file contains development instructions for Claude Code. User documentation is in [README.md](README.md).

## Repository Structure

```
colcon-cargo-ros2/  (THIS REPOSITORY)
├── .envrc                        # Automatic ROS 2 environment setup (direnv)
├── CLAUDE.md                     # This file (development instructions)
├── README.md                     # User-facing documentation
├── justfile                      # Build automation (dual workspace + Python)
│
├── # USER-FACING LIBRARIES (requires ROS 2)
├── user-libs/
│   ├── Cargo.toml                # Workspace manifest
│   ├── rclrs/                    # ROS 2 client library for Rust
│   └── rosidl-runtime-rs/        # Runtime library for ROS messages
│
├── # BUILD INFRASTRUCTURE (no ROS required)
├── packages/                     # Renamed from build-tools/
│   ├── Cargo.toml                # Workspace manifest
│   ├── rosidl-parser/            # ROS IDL parser (.msg, .srv, .action)
│   ├── rosidl-codegen/           # Code generator with Askama templates
│   ├── rosidl-bindgen/           # Binding generator (embeds user-libs)
│   ├── cargo-ros2/               # Build orchestrator (pre-build + post-build)
│   └── colcon-cargo-ros2/        # Python/PyO3 colcon extension
│       ├── colcon_cargo_ros2/    # Python module
│       ├── cargo-ros2-py/        # Rust library exposed to Python
│       └── test/                 # Python tests
│
└── .github/workflows/            # CI for both workspaces
```

## Development Workflows

### Environment Setup

**Automatic (Recommended)**:
```bash
# Install direnv: https://direnv.net/
direnv allow  # Once - permits .envrc to run
# ROS 2 environment now automatically loads when entering directory
```

**Manual**:
```bash
source /opt/ros/jazzy/setup.bash  # Or humble, iron, etc.
```

### Quick Build Commands

```bash
# Build everything (both workspaces + Python wheel)
just build

# Test everything
just test

# Format and lint all code
just format
just check

# Full quality check (format + lint + test)
just quality

# Clean everything
just clean
```

### Workspace-Specific Commands

```bash
# Build-tools/packages workspace (no ROS required)
just build-build-tools
just test-build-tools
just clean-build-tools

# User-libs workspace (requires ROS)
just build-user-libs
just test-user-libs
just clean-user-libs

# Python package
just build-python
just install-python
just install  # Install from wheel
```

### Development Cycle

**CRITICAL**: After modifying code, rebuild and reinstall to see changes:

```bash
# 1. Make changes to code/templates

# 2. Clean and rebuild (REQUIRED for template changes)
just clean-build-tools   # Only needed if templates changed
just build-python        # Rebuild wheel (includes all Rust tools)
just install             # Install updated wheel

# 3. Test in a workspace
cd ~/test_workspace
rm -rf build/.colcon build/ros2_bindings
colcon build --packages-select <package>
```

**Why**: Templates are embedded at compile time. Python wheel bundles Rust binaries. Must reinstall to use updated tools.

## Key Development Guidelines

### File Manipulation Rules

**CRITICAL: ALWAYS use Write/Edit tools for file operations**

- ❌ **NEVER**: `cat > file`, `echo > file`, `cat <<EOF`, or any shell redirection
- ✅ **ALWAYS**: Use `Write` tool to create files
- ✅ **ALWAYS**: Use `Edit` tool to modify files
- ✅ **ALWAYS**: Use `Read` tool to view files

**Exception**: Bash is only for system commands (git, cargo, colcon, etc.), not file I/O.

### Temporary Files

**All temporary files MUST be created in `$PROJECT_ROOT/tmp/` using Write tool:**

```bash
# ✅ CORRECT
Write: tmp/test_data.json
Content: {"key": "value"}

Write: tmp/build_script.sh
Content: |
  #!/bin/bash
  cargo build --release

Bash: chmod +x tmp/build_script.sh && tmp/build_script.sh

# ❌ WRONG
Bash: echo '{"key": "value"}' > /tmp/test_data.json
Bash: cat > tmp/test.sh <<'EOF'
  #!/bin/bash
  ...
EOF
```

### Code Quality

**REQUIRED before committing**:
```bash
just quality  # Format + lint + test
```

Ensures:
- Code formatted with nightly rustfmt
- All clippy warnings fixed (`-D warnings`)
- All tests pass (Rust + Python)
- Zero warnings

## Recent Architectural Improvements

### Workspace-Local Library Linking Fix (2025-11-18)

**Problem**: Rust binaries failed to link against workspace-local ROS interface libraries:
```
rust-lld: error: unable to find library -lsplat_msgs__rosidl_typesupport_c
```

**Root Cause**: Cargo `build.rs` linker search paths (`cargo:rustc-link-search`) don't propagate to downstream binaries.

**Solution**: Added `[build]` rustflags to `ros2_cargo_config.toml`:
```toml
[build]
rustflags = [
    "-L", "native=/path/to/install/package/lib",
    "-L", "native=/opt/ros/jazzy/lib"
]
```

**Files Modified**: `packages/colcon-cargo-ros2/colcon_cargo_ros2/workspace_bindgen.py`

**Impact**: Enables workspaces with custom interface packages used by Rust packages.

---

### `[package.metadata.ros]` Installation Support (2025-11-17)

Implemented support for installing additional files (launch, config, URDF, etc.):

```toml
[package.metadata.ros]
install_to_share = ["launch", "config", "README.md"]  # Directories and files
install_to_include = ["include"]
install_to_lib = ["scripts"]
```

**Semantics**:
- **Directories**: Copied recursively with name preserved
- **Individual files**: Filename preserved (parent path dropped)
- **Missing paths**: Build fails with clear error

100% backward compatible with cargo-ament-build.

---

### WString Array/Sequence Support (2025-11-17)

Added complete WString support across all idiomatic templates to fix type mismatches in generated bindings.

---

### `--cargo-args` Support (2025-11-17)

Added ability to pass arguments to Cargo:
```bash
colcon build --cargo-args --release
colcon build --cargo-args --profile dev-release
```

---

### Ruff Linter Migration (2025-11-17)

Migrated from flake8 to ruff (10-100x faster, written in Rust, no plugin dependencies).

---

### Dual Workspace Architecture (2025-11-11)

Split into two independent workspaces:
- **user-libs/**: Requires ROS 2 environment (`rclrs`, `rosidl-runtime-rs`)
- **packages/**: No ROS required (build infrastructure)

Benefits:
- Can develop build tools without ROS installed
- Faster CI for build tools
- Clear separation of concerns

---

### Workspace-Level Shared Bindings (2025-11-07)

Bindings generated once at `build/ros2_bindings/`, shared by all packages:
```
build/ros2_bindings/std_msgs/      # Generated once
src/pkg1/.cargo/config.toml → ../../build/ros2_bindings/*
src/pkg2/.cargo/config.toml → ../../build/ros2_bindings/*
```

Benefits:
- No duplication (std_msgs generated once, not per-package)
- Faster builds
- Smaller workspace

## Testing

```bash
# All tests
just test

# Workspace-specific
just test-build-tools  # No ROS required
just test-user-libs    # Requires ROS

# Specific package
cd packages/rosidl-codegen && cargo test
```

## CI/CD

GitHub Actions workflows:
- **wheels.yml**: Production builds (triggered by tags, publishes to PyPI)
- **test-build.yml**: Quick validation on PRs

Builds 31 artifacts (30 wheels + sdist) for Linux/macOS/Windows × Python 3.8-3.13.

## Status

**Version**: v0.3.0 Released (2025-11-18)
**Progress**: 15/20 subphases (75%) | 206 tests passing (203 Rust + 3 Python) | Zero warnings
**Latest**: Workspace-local library linking fix ✅, `[package.metadata.ros]` installation support ✅
**Testing**: Validated with autoware_carla_bridge (118 packages) ✅, splat-drive workspace (6 packages + custom interfaces) ✅

**Versions**:
- Rust workspace: v0.2.0 (rosidl-parser, rosidl-codegen, rosidl-bindgen, cargo-ros2)
- Python package: v0.3.0 (colcon-cargo-ros2)

**PyPI**: 31 artifacts for Linux/macOS/Windows × Python 3.8-3.13

**Architecture**:
- Two independent workspaces (user-libs + packages)
- Workspace-level binding generation via colcon's package discovery
- Rustflags-based linker search paths for workspace libraries
- Complete ament layout installation

**Next**: Phase 3.4 - Enhanced Testing & Documentation
