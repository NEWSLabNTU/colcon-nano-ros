# Combined Rust + Python justfile for colcon-cargo-ros2

# Default recipe - show available commands
default:
    @just --list

# === PACKAGES WORKSPACE ===

# Build packages workspace
build-packages:
    #!/usr/bin/env bash
    set -e
    cd packages
    cargo build \
        --profile dev-release \
        --all-targets

# Test packages workspace
test-packages:
    #!/usr/bin/env bash
    set -e
    cd packages
    cargo nextest run \
        --cargo-profile dev-release \
        --no-fail-fast

# Format packages workspace
format-packages:
    #!/usr/bin/env bash
    set -e
    cd packages
    cargo +nightly fmt

# Check/lint packages workspace
check-packages:
    #!/usr/bin/env bash
    set -e
    cd packages
    cargo clippy --workspace --all-targets -- -D warnings

# Clean packages workspace
clean-packages:
    #!/usr/bin/env bash
    set -e
    cd packages
    cargo clean
    rm -rf colcon-cargo-ros2/dist/ colcon-cargo-ros2/build/ colcon-cargo-ros2/*.egg-info

# === PYTHON COMMANDS ===

# Build Python package (wheel)
build-python:
    #!/usr/bin/env bash
    set -e
    cd packages/colcon-cargo-ros2
    maturin build --profile dev-release

# Install Python package in development mode
install-python:
    pip3 install -e packages/colcon-cargo-ros2/ --break-system-packages

# Test Python code
test-python:
    pytest packages/colcon-cargo-ros2/test/

# Format Python code
format-python:
    #!/usr/bin/env bash
    set -e
    cd packages/colcon-cargo-ros2
    ruff format colcon_cargo_ros2/ test/

# Lint Python code
check-python:
    #!/usr/bin/env bash
    set -e
    cd packages/colcon-cargo-ros2
    ruff check colcon_cargo_ros2/ test/

# === COMBINED COMMANDS ===

# Build packages
build: build-packages build-python

# Install packages
install: install-python

# Test packages + Python
test: test-packages test-python

# Clean packages
clean: clean-packages

# Format all code (packages + Python)
format:
    just format-packages
    just format-python

# Lint and check all code (packages + Python)
check:
    just check-packages
    just check-python

# === QUALITY COMMANDS ===

# Run all quality checks (format, lint, test)
quality: format check test
