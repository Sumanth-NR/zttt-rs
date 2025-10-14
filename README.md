# zttt-rs

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

The fastest and most optimized Rust backend for simulating TicTacToe games.

## Features

- **Blazing Fast Simulations**: Optimized for maximum throughput - over 1.8M games/second with FastEngine
- **Efficient Game State**: Minimal memory footprint with optimized board representation
- **High-Speed Engine**: FastEngine for ultra-fast simulations (0.55µs per game)
- **Pluggable Architecture**: Implement custom engines via the `Engine` trait for research and experimentation
- **Simple API**: Clean, easy-to-use interface designed for fast integration
- **Benchmark Reference**: PerfectEngine with minimax/alpha-beta pruning available in examples for comparison

## Installation

```bash
cargo add zttt-rs
```

## Quick Start

```rust
use zttt_rs::{Board, Player, FastEngine, Engine};

fn main() {
    let mut board = Board::new();
    let engine = FastEngine;
    
    // Make a move
    board.make_move(0, 0, Player::X).unwrap();
    
    // Get next move for O
    if let Some((row, col)) = engine.choose_move(&board, Player::O) {
        board.make_move(row, col, Player::O).unwrap();
    }
    
    println!("{}", board);
}
```

### High-Speed Simulation Example

```rust
use zttt_rs::{Board, Player, GameResult, FastEngine, Engine};

// Simulate a complete game in ~0.55µs
let mut board = Board::new();
let mut current_player = Player::X;
let engine = FastEngine;

while board.game_result() == GameResult::InProgress {
    if let Some((row, col)) = engine.choose_move(&board, current_player) {
        board.make_move(row, col, current_player).unwrap();
        current_player = current_player.opponent();
    }
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

# Run fast simulation example
cargo run --example fast_simulation --release

# Run benchmark (includes PerfectEngine for comparison)
cargo run --example benchmark --release
```

## Contributing

When contributing to this repository, please:
- Keep the vision in mind: **fastest and most optimized simulation backend**
- Update [.ai-context.md](.ai-context.md) if you make significant changes to structure, API, or performance
- See [.github/copilot-instructions.md](.github/copilot-instructions.md) for detailed guidelines

## License

MIT License - see [LICENSE](LICENSE) for details

Inspired by [ZTicTacToe](https://github.com/ZTicTacToe)