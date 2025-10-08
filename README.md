# zttt-rs

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A high-performance TicTacToe engine in Rust with perfect play using minimax with alpha-beta pruning.

## Features

- **Perfect Play Engine**: Minimax algorithm with alpha-beta pruning for optimal moves
- **Fast Performance**: ~1.6ms per game, ~614 games/second
- **Simple API**: Easy-to-use board and engine interface
- **Pluggable Architecture**: Implement custom engines via the `Engine` trait

## Installation

```bash
cargo add zttt-rs
```

## Quick Start

```rust
use zttt_rs::{Board, Player, PerfectEngine, Engine};

fn main() {
    let mut board = Board::new();
    let engine = PerfectEngine::new();
    
    // Make a move
    board.make_move(0, 0, Player::X).unwrap();
    
    // Get the best move for O
    if let Some((row, col)) = engine.choose_move(&board, Player::O) {
        board.make_move(row, col, Player::O).unwrap();
    }
    
    println!("{}", board);
}
```

See the [examples](examples/) directory for more usage patterns.

## Documentation

- **API Documentation**: [docs.rs/zttt-rs](https://docs.rs/zttt-rs)
- **Architecture**: [.github/ARCHITECTURE.md](.github/ARCHITECTURE.md)
- **Contributing**: [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md)
- **Development Guide**: [.github/DEVELOPMENT.md](.github/DEVELOPMENT.md)
- **AI Conventions**: [.github/AI_CONVENTIONS.md](.github/AI_CONVENTIONS.md)

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy

# Build release
cargo build --release

# Run examples
cargo run --example ai_game
```

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for detailed development guidelines.

## Contributing

Contributions are welcome! Please read our [Contributing Guide](.github/CONTRIBUTING.md) and [Code of Conduct](.github/CODE_OF_CONDUCT.md).

## License

MIT License - see [LICENSE](LICENSE) for details

Inspired by [ZTicTacToe](https://github.com/ZTicTacToe)