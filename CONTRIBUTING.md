# Contributing to zttt-rs

Thank you for your interest in contributing to zttt-rs! This document provides guidelines and information to help you contribute effectively.

## Project Vision

**zttt-rs** aims to be the fastest and most optimized Rust backend for simulating TicTacToe games. Our focus is on:

- **Performance**: High-throughput game simulations (~614 games/second)
- **Efficiency**: Minimal memory footprint
- **Simplicity**: Clean, intuitive API
- **Reliability**: Thoroughly tested game logic

## Getting Started

### Prerequisites

- Rust 1.90 or higher
- Familiarity with Rust basics
- Understanding of TicTacToe game rules

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/Sumanth-NR/zttt-rs.git
cd zttt-rs

# Run tests
cargo test

# Run examples
cargo run --example basic_game
cargo run --example simulation --release
cargo run --example benchmark --release

# Build documentation
cargo doc --open
```

## Development Workflow

### Before Making Changes

1. Check existing issues or create a new one to discuss your idea
2. Fork the repository and create a feature branch
3. Ensure all tests pass: `cargo test`
4. Run benchmarks to establish baseline: `cargo run --example benchmark --release`

### Making Changes

1. Write your code following our style guidelines
2. Add tests for new functionality
3. Update documentation (doc comments and README if needed)
4. Run tests: `cargo test`
5. Check formatting: `cargo fmt -- --check`
6. Run linter: `cargo clippy -- -D warnings`

### Performance Considerations

Since this is a performance-focused library:

- **Benchmark your changes**: Run `cargo run --example benchmark --release` before and after
- **Profile if needed**: Use `cargo flamegraph` for detailed profiling
- **Avoid allocations**: Especially in hot paths like `minimax()` and `game_result()`
- **Prefer arrays over Vec**: For fixed-size collections
- **Document performance characteristics**: Note if your change affects performance

### Code Style

- Follow standard Rust formatting (use `cargo fmt`)
- Write clear, descriptive variable and function names
- Add doc comments for all public APIs with examples
- Keep functions focused and reasonably sized
- Use `Result<T, &'static str>` for error handling

### Testing Requirements

All contributions must include appropriate tests:

```rust
#[test]
fn test_your_feature() {
    // Setup
    let board = Board::new();
    
    // Action
    let result = board.some_operation();
    
    // Assert
    assert_eq!(result, expected_value);
}
```

Types of tests to consider:
- **Unit tests**: Test individual functions
- **Integration tests**: Test complete workflows
- **Doc tests**: Ensure examples in documentation work
- **Edge cases**: Boundary conditions, invalid inputs, game-over states

### Documentation

- All public APIs need doc comments
- Include usage examples in doc comments
- Update README.md if adding major features
- Add examples in `examples/` directory for complex features

Example documentation:
```rust
/// Creates a new empty TicTacToe board.
///
/// # Example
///
/// ```
/// use zttt_rs::Board;
///
/// let board = Board::new();
/// assert_eq!(board.valid_moves().len(), 9);
/// ```
pub fn new() -> Self {
    // implementation
}
```

## Contribution Guidelines

### What We're Looking For

✅ **Welcome Contributions**:
- Performance optimizations (with benchmarks)
- Bug fixes (with tests)
- Additional engines implementing the `Engine` trait
- Documentation improvements
- More examples
- Better test coverage

⚠️ **Discuss First**:
- API changes
- New public types or traits
- Adding dependencies
- Changing core algorithms

❌ **Generally Not Accepted**:
- Adding runtime dependencies without strong justification
- Features that compromise performance
- UI or visualization code (belongs in separate crates)
- Breaking changes without migration path
- Code that doesn't meet quality standards

### Pull Request Process

1. **Create a feature branch**: `git checkout -b feature/your-feature-name`
2. **Make your changes** following guidelines above
3. **Write/update tests** ensuring they pass
4. **Update documentation** as needed
5. **Run quality checks**:
   ```bash
   cargo test
   cargo fmt -- --check
   cargo clippy -- -D warnings
   cargo doc --no-deps
   ```
6. **Commit with clear messages**:
   ```
   Add feature: brief description
   
   Longer explanation of what changed and why.
   Include performance impact if relevant.
   ```
7. **Push and create PR** with description of changes
8. **Address review feedback** promptly

### Pull Request Description Template

```markdown
## Description
Brief summary of changes

## Motivation
Why is this change needed?

## Changes Made
- List of specific changes
- Include any API changes
- Note any breaking changes

## Testing
- What tests were added/modified?
- How was this tested?

## Performance Impact
- Benchmarks before/after (if applicable)
- Any performance implications?

## Checklist
- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Examples added/updated (if needed)
```

## Code Review

All contributions go through code review. Reviewers will check:

- Code quality and style
- Test coverage
- Documentation completeness
- Performance implications
- API design consistency

Be patient and responsive to feedback. Code review is a learning opportunity for everyone.

## Performance Benchmarking

When making performance-related changes:

```bash
# Run benchmarks
cargo run --example benchmark --release

# For detailed profiling
cargo install flamegraph
cargo flamegraph --example simulation
```

Document results in your PR:
```
Before: ~1.6ms per game, ~614 games/second
After:  ~1.4ms per game, ~710 games/second
Improvement: 15% faster
```

## Common Issues and Solutions

### "Tests failing"
- Ensure you're on latest `main` branch
- Run `cargo clean` and rebuild
- Check if you've added all necessary test cases

### "Clippy warnings"
- Run `cargo clippy --fix` for automatic fixes
- Address any remaining warnings manually
- Some warnings may require code restructuring

### "Performance regression"
- Profile with `flamegraph` to identify bottlenecks
- Check for unnecessary allocations
- Consider algorithm complexity changes

## Questions?

- Open an issue for questions about contributing
- Check existing issues and PRs for similar work
- Review [GitHub Copilot instructions](.github/copilot-instructions.md) for detailed context

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to zttt-rs! 🎮
