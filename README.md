# zttt-rs

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

The fastest and most optimized Rust backend for simulating TicTacToe games.

## Features

- **Blazing Fast Simulations**: ~1.6ms per game, ~614 games/second - optimized for high-throughput game simulations
- **Efficient Game State**: Minimal memory footprint with optimized board representation
- **Perfect Play Engine**: Built-in minimax algorithm with alpha-beta pruning for optimal AI moves
- **Pluggable Architecture**: Implement custom engines via the `Engine` trait for research and experimentation
- **Simple API**: Clean, easy-to-use interface designed for fast integration

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

## Contributing

When contributing to this repository, please:
- Keep the vision in mind: **fastest and most optimized simulation backend**
- Update [.ai-context.md](.ai-context.md) if you make significant changes to structure, API, or performance
- See [.github/copilot-instructions.md](.github/copilot-instructions.md) for detailed guidelines

## License

MIT License - see [LICENSE](LICENSE) for details

Inspired by [ZTicTacToe](https://github.com/ZTicTacToe)