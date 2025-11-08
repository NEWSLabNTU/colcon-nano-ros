build:
    python -m build

install:
    pip install -e ".[test]"

test:
    pytest test/

format:
    ruff format colcon_cargo_ros2/ test/

lint:
    ruff check colcon_cargo_ros2/ test/
