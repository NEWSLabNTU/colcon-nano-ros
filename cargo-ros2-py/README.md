# cargo-ros2-py

Python bindings for [cargo-ros2](../cargo-ros2), providing direct access to ROS 2 Rust build tools from Python.

## Features

- **Direct API calls**: No subprocess overhead
- **Error propagation**: Rust errors become Python exceptions
- **Type safety**: Python classes mirror Rust configuration structs
- **Fast**: Compiled Rust code with minimal Python overhead

## Installation

```bash
pip install cargo-ros2-py
```

Or for development:

```bash
pip install maturin
cd cargo-ros2-py
maturin develop
```

## Usage

### Generate Bindings

```python
import cargo_ros2_py

config = cargo_ros2_py.BindgenConfig(
    package_name="std_msgs",
    output_dir="target/ros2_bindings",
    package_path=None,  # Use ament index
    verbose=True
)

cargo_ros2_py.generate_bindings(config)
```

### Install to Ament Layout

```python
import cargo_ros2_py

config = cargo_ros2_py.InstallConfig(
    project_root="/path/to/project",
    install_base="install/my_package",
    profile="release",
    verbose=True
)

cargo_ros2_py.install_to_ament(config)
```

### Clean Bindings

```python
import cargo_ros2_py

cargo_ros2_py.clean_bindings(
    project_root="/path/to/project",
    verbose=True
)
```

## API Reference

### `BindgenConfig`

Configuration for binding generation.

**Attributes:**
- `package_name` (str): ROS package name (e.g., "std_msgs")
- `output_dir` (str): Output directory for generated bindings
- `package_path` (Optional[str]): Direct path to package share directory (bypasses ament index)
- `verbose` (bool): Enable verbose output

### `InstallConfig`

Configuration for ament installation.

**Attributes:**
- `project_root` (str): Project root directory (where Cargo.toml is)
- `install_base` (str): Install base directory (install/<package>/)
- `profile` (str): Build profile ("debug" or "release")
- `verbose` (bool): Enable verbose output

### Functions

#### `generate_bindings(config: BindgenConfig) -> None`

Generate Rust bindings for a ROS 2 interface package.

**Raises:** `RuntimeError` if generation fails

#### `install_to_ament(config: InstallConfig) -> None`

Install package binaries and libraries to ament layout.

**Raises:** `RuntimeError` if installation fails

#### `clean_bindings(project_root: str, verbose: bool) -> None`

Clean generated bindings and cache.

**Raises:** `RuntimeError` if cleanup fails

## Building Wheels

```bash
cd cargo-ros2-py
maturin build --release --strip
```

Multi-platform builds via GitHub Actions are configured in the repository.

## License

MIT OR Apache-2.0
