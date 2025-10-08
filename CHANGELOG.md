# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub repository structure with CI/CD
- Comprehensive documentation (CONTRIBUTING.md, ARCHITECTURE.md, DEVELOPMENT.md)
- AI conventions guide for AI agents
- Issue and PR templates
- Code of Conduct
- Security policy
- rustfmt and clippy configurations

## [0.1.0] - 2024

### Added
- Initial release
- Board representation with 3x3 grid
- Player types (X and O)
- Game result tracking (Win, Draw, InProgress)
- Perfect play engine using minimax with alpha-beta pruning
- Engine trait for custom implementations
- Move validation and game logic
- Basic examples (ai_game, basic_game)
- Performance benchmarks (~1.6ms per game)
- Simulation example for testing

### Features
- Fast performance: ~614 games/second
- Zero external dependencies
- Safe Rust (no unsafe code)
- Comprehensive test suite
- Documentation with examples

[Unreleased]: https://github.com/Sumanth-NR/zttt-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Sumanth-NR/zttt-rs/releases/tag/v0.1.0
