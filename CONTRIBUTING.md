# Contributing to agentropic-runtime

Thank you for your interest in contributing!

## Development Setup

1. Clone the repository
2. Install Rust: https://rustup.rs/
3. Build the project:
```bash
   cargo build
```

## Running Tests
```bash
cargo test
```

## Running Examples
```bash
cargo run --example basic_runtime
cargo run --example scheduled_agents
cargo run --example isolated_agents
cargo run --example supervised_agents
```

## Running Benchmarks
```bash
cargo bench
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write tests for new features
- Update documentation

## Pull Requests

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and formatting
5. Submit a pull request

## License

By contributing, you agree that your contributions will be licensed under MIT/Apache-2.0.
