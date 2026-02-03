# Contributing to AllTheSkills

Thank you for your interest in contributing! This document outlines the process for contributing to AllTheSkills.

## Getting Started

### Prerequisites

- Rust 1.70.0 or later
- cargo

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/alltheskills/alltheskills.git
cd alltheskills

# Install dependencies and run tests
cargo test --workspace

# Build all packages
cargo build --workspace
```

## Development Workflow

1. **Fork the repository** on GitHub
2. **Create a branch** for your feature or bugfix
3. **Make changes** following the coding standards
4. **Add tests** for your changes
5. **Run tests** to ensure everything passes
6. **Submit a pull request**

## Coding Standards

### Rust Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Documentation

- Add docstrings to public APIs
- Update README.md for user-facing changes
- Add examples for new features

### Testing

- Add unit tests for new functionality
- Add integration tests for CLI commands
- Ensure all tests pass before submitting

## Adding New Providers

To add support for a new AI skill source:

1. Create a new file in `crates/alltheskills/src/providers/<source>.rs`
2. Implement the `SkillProvider` trait
3. Add the module to `crates/alltheskills/src/providers/mod.rs`
4. Add source detection in `crates/alltheskills/src/providers/detect.rs`
5. Update `README.md` with the new source
6. Add tests for the new provider

## Pull Request Process

1. Ensure all tests pass
2. Update documentation as needed
3. Add a clear description of your changes
4. Link any related issues

## Publishing to crates.io

Only maintainers can publish to crates.io. For release process:

1. Update version in `Cargo.toml`
2. Create a release tag: `git tag v0.x.y`
3. Push tag: `git push origin v0.x.y`
4. GitHub Actions will build and publish to crates.io

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/conduct.html).
