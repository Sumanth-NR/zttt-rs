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

Full API documentation is available at [docs.rs/zttt-rs](https://docs.rs/zttt-rs)

## Development

```bash
# Run tests
cargo test

# Build release
cargo build --release

# Run benchmarks
cargo bench
```

## License

MIT License - see [LICENSE](LICENSE) for details

Inspired by [ZTicTacToe](https://github.com/ZTicTacToe)