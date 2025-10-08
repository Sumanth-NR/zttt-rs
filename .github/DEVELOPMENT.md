# Development Guide

This guide provides detailed information for developers working on zttt-rs.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/Sumanth-NR/zttt-rs.git
cd zttt-rs

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example ai_game
```

## Development Tools

### Required Tools
- **Rust 1.90+**: Install via [rustup](https://rustup.rs/)
- **Cargo**: Comes with Rust

### Recommended Tools
- **rust-analyzer**: LSP implementation for Rust
- **clippy**: Linting tool
- **rustfmt**: Code formatter

### Installing Development Tools

```bash
# Install clippy and rustfmt
rustup component add clippy rustfmt

# Install cargo-edit for managing dependencies
cargo install cargo-edit

# Install cargo-watch for auto-rebuilding
cargo install cargo-watch

# Install cargo-tarpaulin for coverage (Linux only)
cargo install cargo-tarpaulin
```

## Project Structure

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

```
zttt-rs/
├── .cargo/            # Cargo configuration
├── .github/           # GitHub-specific files
│   ├── workflows/     # CI/CD workflows
│   ├── ISSUE_TEMPLATE/# Issue templates
│   ├── ARCHITECTURE.md
│   ├── CONTRIBUTING.md
│   └── DEVELOPMENT.md
├── examples/          # Usage examples
├── src/               # Source code
│   ├── lib.rs         # Main library
│   ├── board.rs       # Board logic
│   ├── engine.rs      # AI engines
│   ├── game.rs        # Game types
│   └── player.rs      # Player types
├── Cargo.toml         # Project metadata
├── rustfmt.toml       # Formatting config
└── README.md          # Project overview
```

## Development Workflow

### 1. Creating a Feature

```bash
# Create a new branch
git checkout -b feature/my-feature

# Make changes
# ... edit files ...

# Format code
cargo fmt

# Check code
cargo clippy

# Run tests
cargo test

# Commit changes
git commit -am "Add my feature"
```

### 2. Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run doc tests
cargo test --doc

# Run with coverage (Linux only)
cargo tarpaulin --verbose
```

### 3. Checking Code Quality

```bash
# Format check (CI will fail if not formatted)
cargo fmt --check

# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features

# Build documentation
cargo doc --open

# Check for warnings
cargo build --all-targets --all-features
```

### 4. Running Examples

```bash
# Run AI game example
cargo run --example ai_game

# Run basic game example
cargo run --example basic_game

# Run performance benchmark
cargo run --example benchmark --release

# Run simulation
cargo run --example simulation --release
```

### 5. Performance Testing

```bash
# Build with optimizations
cargo build --release

# Run benchmarks
cargo run --example benchmark --release

# Profile with perf (Linux)
cargo build --release
perf record target/release/examples/benchmark
perf report
```

## Code Style Guide

### Formatting
- Use `cargo fmt` to format all code
- Follow Rust standard naming conventions
- Keep lines under 100 characters (configured in rustfmt.toml)

### Naming Conventions
- Types: `PascalCase` (e.g., `PerfectEngine`)
- Functions: `snake_case` (e.g., `make_move`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_SIZE`)
- Modules: `snake_case` (e.g., `game_result`)

### Documentation
- Document all public items
- Include examples in doc comments
- Use proper markdown formatting
- Reference related items with backticks

Example:
```rust
/// Makes a move on the board at the specified position
///
/// # Arguments
///
/// * `row` - Row index (0-2)
/// * `col` - Column index (0-2)
/// * `player` - The player making the move
///
/// # Returns
///
/// Returns `Ok(())` if the move was valid, or an error message if invalid.
///
/// # Example
///
/// ```
/// use zttt_rs::{Board, Player};
///
/// let mut board = Board::new();
/// board.make_move(0, 0, Player::X).unwrap();
/// ```
pub fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), &str> {
    // Implementation
}
```

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Provide clear error messages
- Document error conditions

### Testing
- Write unit tests for all public APIs
- Include edge cases
- Test error conditions
- Use descriptive test names

## Performance Guidelines

### General Rules
1. Avoid unnecessary allocations
2. Prefer stack over heap when possible
3. Use `Copy` types where appropriate
4. Profile before optimizing

### Benchmarking
- Use release builds for benchmarking
- Run multiple iterations
- Compare against baseline
- Document performance characteristics

## Debugging

### Using rust-analyzer
- Install rust-analyzer in your editor
- Use "Go to Definition" for navigation
- Use "Find All References" to see usage
- Hover for type information

### Print Debugging
```rust
// Use dbg! macro
dbg!(board.game_result());

// Use Display trait
println!("{}", board);

// Use Debug trait
println!("{:?}", board);
```

### Using the Rust Debugger
```bash
# With rust-gdb (Linux/macOS)
rust-gdb target/debug/examples/ai_game

# With rust-lldb (macOS)
rust-lldb target/debug/examples/ai_game
```

## Continuous Integration

Our CI runs on every push and PR:

1. **Tests**: Run on Ubuntu, Windows, macOS with stable and beta Rust
2. **Format**: Checks code formatting
3. **Clippy**: Lints for common mistakes
4. **Docs**: Builds documentation
5. **Examples**: Runs all examples
6. **Coverage**: Generates code coverage report

View CI status in the GitHub Actions tab.

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Commit changes: `git commit -am "Bump version to X.Y.Z"`
4. Tag release: `git tag -a vX.Y.Z -m "Version X.Y.Z"`
5. Push changes: `git push && git push --tags`
6. Create GitHub release from tag
7. Publish to crates.io: `cargo publish`

## Common Tasks

### Adding a New Feature
1. Create feature branch
2. Implement feature
3. Add tests
4. Update documentation
5. Run full test suite
6. Submit PR

### Fixing a Bug
1. Create bug fix branch
2. Add failing test
3. Fix the bug
4. Verify test passes
5. Submit PR

### Updating Dependencies
```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Test after updating
cargo test
```

## Getting Help

- **Issues**: Open an issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check docs.rs/zttt-rs

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Rustfmt](https://github.com/rust-lang/rustfmt)
