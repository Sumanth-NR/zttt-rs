# Contributing to zttt-rs

Thank you for considering contributing to zttt-rs! This document provides guidelines and information for contributors.

## Development Setup

1. **Install Rust**: Make sure you have Rust 1.90+ installed. Visit [rustup.rs](https://rustup.rs/) for installation instructions.

2. **Clone the repository**:
   ```bash
   git clone https://github.com/Sumanth-NR/zttt-rs.git
   cd zttt-rs
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## Project Structure

```
zttt-rs/
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── board.rs        # Board representation and game logic
│   ├── engine.rs       # AI engine implementations (minimax)
│   ├── game.rs         # Game result types
│   └── player.rs       # Player and Cell types
├── examples/           # Usage examples
│   ├── ai_game.rs      # Engine vs Engine game
│   ├── basic_game.rs   # Basic usage example
│   ├── benchmark.rs    # Performance benchmarks
│   └── simulation.rs   # Multiple game simulations
└── tests/              # Integration tests
```

## Coding Standards

### Code Style
- Follow standard Rust conventions
- Use `cargo fmt` to format code
- Run `cargo clippy` to catch common mistakes
- Write documentation comments for public APIs
- Keep functions small and focused

### Testing
- Write unit tests for new functionality
- Ensure all tests pass before submitting PR
- Aim for high test coverage
- Include edge cases in tests

### Documentation
- Document all public APIs with doc comments
- Include examples in documentation
- Update README.md if adding new features
- Keep documentation up-to-date with code changes

## Making Changes

1. **Create a branch**: Create a new branch for your work
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**: Implement your feature or fix

3. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Run examples** to verify functionality:
   ```bash
   cargo run --example basic_game
   cargo run --example benchmark
   ```

5. **Commit your changes**: Write clear, descriptive commit messages
   ```bash
   git commit -m "Add feature: description of change"
   ```

6. **Push and create PR**: Push your branch and create a pull request

## Pull Request Guidelines

- **Title**: Use a clear, descriptive title
- **Description**: Explain what changes you made and why
- **Tests**: Include tests for new functionality
- **Documentation**: Update docs if needed
- **Small PRs**: Keep PRs focused and reasonably sized
- **One concern per PR**: Don't mix unrelated changes

## Code Review Process

- Maintainers will review your PR
- Address any feedback or requested changes
- Once approved, your PR will be merged

## Performance Considerations

This library focuses on performance:
- Avoid unnecessary allocations
- Use efficient algorithms
- Benchmark performance-critical changes
- Document any performance trade-offs

## Questions?

If you have questions, please:
- Open an issue for discussion
- Check existing issues and PRs
- Review the documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
