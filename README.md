# Quanta Engine

Quanta Engine is a high-performance trading engine implemented in Rust with Python bindings.

## Features

- Strategy management
- Risk validation
- Order execution
- Python integration

## Installation

### From PyPI

```bash
pip install quanta-engine
```

### From source

```bash
git clone https://github.com/your-username/quanta-engine.git
cd quanta-engine
pip install .
```

## Usage

```python
from quanta_engine import StrategyManager, StrategyStatus

manager = StrategyManager()
manager.add_strategy("strat1", "Moving Average Crossover", "user1")
manager.add_strategy("strat2", "RSI Reversal", "user2")

active_strategies = manager.list_active_strategies()
for strategy in active_strategies:
    print(f"Active strategy: {strategy.name} (User: {strategy.user_id})")
```

## Development

### Setup

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install Python 3.7+
3. Clone the repository
4. Install development dependencies: `pip install -r requirements-dev.txt`

### Running tests

```bash
cargo test
pytest python/tests
```

### Building

```bash
python build.py
```

### Running benchmarks

```bash
cargo bench
```

### Creating a release

1. Update the version in `Cargo.toml`
2. Commit the changes
3. Run:
   ```bash
   cargo release patch # or minor, or major
   ```

## Contributing

# TODO:
Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.