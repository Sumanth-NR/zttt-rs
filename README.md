# zttt-rs

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A high-performance Rust backend for TicTacToe games with pluggable engines, inspired by [ZTicTacToe](https://github.com/ZTicTacToe) in Python.

## Features

- **Efficient State Representation**: Compact board representation optimized for performance
- **Complete Game Logic**: Move validation, win detection, and game state management
- **Pluggable Engine Architecture**: Trait-based system for implementing custom move selection strategies
- **Perfect Engine**: Built-in engine using minimax with alpha-beta pruning for optimal play
- **Fast Simulations**: Designed for researchers and users who need high-speed game simulations
- **Well-Tested**: Comprehensive test suite ensuring correctness
- **Easy to Use**: Simple, intuitive API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zttt-rs = "0.1.0"
```

Or use cargo-add:

```bash
cargo add zttt-rs
```

## Usage

### Basic Game Play

```rust
use zttt_rs::{Board, Player, GameResult};

fn main() {
    // Create a new board
    let mut board = Board::new();
    
    // Make some moves
    board.make_move(0, 0, Player::X).unwrap();
    board.make_move(1, 1, Player::O).unwrap();
    board.make_move(0, 1, Player::X).unwrap();
    
    // Display the board
    println!("{}", board);
    // Output:
    // X X .
    // . O .
    // . . .
    
    // Check game status
    match board.game_result() {
        GameResult::Win(player) => println!("{} wins!", player),
        GameResult::Draw => println!("It's a draw!"),
        GameResult::InProgress => println!("Game in progress"),
    }
}
```

### Using the Perfect Engine

```rust
use zttt_rs::{Board, Player, PerfectEngine, Engine};

fn main() {
    let mut board = Board::new();
    board.make_move(0, 0, Player::X).unwrap();
    board.make_move(1, 1, Player::O).unwrap();
    
    // Create a perfect engine
    let engine = PerfectEngine::new();
    
    // Get the best move for Player X
    if let Some((row, col)) = engine.choose_move(&board, Player::X) {
        println!("Best move for X: ({}, {})", row, col);
        board.make_move(row, col, Player::X).unwrap();
    }
}
```

### Engine vs Engine Simulation

```rust
use zttt_rs::{Board, Player, GameResult, PerfectEngine, Engine};

fn simulate_game() -> GameResult {
    let mut board = Board::new();
    let mut current_player = Player::X;
    let engine = PerfectEngine::new();
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        }
    }
    
    board.game_result()
}

fn main() {
    // Run multiple simulations
    let mut wins_x = 0;
    let mut wins_o = 0;
    let mut draws = 0;
    
    for _ in 0..1000 {
        match simulate_game() {
            GameResult::Win(Player::X) => wins_x += 1,
            GameResult::Win(Player::O) => wins_o += 1,
            GameResult::Draw => draws += 1,
            _ => {}
        }
    }
    
    println!("Results after 1000 games:");
    println!("X wins: {}", wins_x);
    println!("O wins: {}", wins_o);
    println!("Draws: {}", draws);
}
```

### Implementing a Custom Engine

You can implement your own engine by implementing the `Engine` trait:

```rust
use zttt_rs::{Board, Player, Engine};

/// A random move engine (for demonstration)
struct RandomEngine;

impl Engine for RandomEngine {
    fn choose_move(&self, board: &Board, _player: Player) -> Option<(usize, usize)> {
        let moves = board.valid_moves();
        if moves.is_empty() {
            None
        } else {
            // In a real implementation, you'd use proper randomization
            Some(moves[0])
        }
    }
}

fn main() {
    let mut board = Board::new();
    let engine = RandomEngine;
    
    if let Some((row, col)) = engine.choose_move(&board, Player::X) {
        board.make_move(row, col, Player::X).unwrap();
    }
}
```

## API Documentation

### Core Types

#### `Player`
Represents a player in the game (X or O).

- `Player::X` - The X player
- `Player::O` - The O player
- `opponent()` - Returns the opponent player

#### `GameResult`
Represents the current state of the game.

- `GameResult::Win(Player)` - A player has won
- `GameResult::Draw` - The game is a draw
- `GameResult::InProgress` - The game is still ongoing

#### `Board`
The main game board structure.

**Methods:**

- `new()` - Creates a new empty board
- `make_move(row, col, player)` - Makes a move at the specified position
- `is_valid_move(row, col)` - Checks if a move is valid
- `valid_moves()` - Returns all valid move positions
- `game_result()` - Returns the current game result
- `choose_move(engine, player)` - Convenience method to get a move from an engine
- `get(row, col)` - Gets the cell at the specified position
- `reset()` - Resets the board to empty state

### Engines

#### `Engine` Trait
Defines the interface for implementing custom move selection strategies.

**Required Methods:**

- `choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)>` - Choose the best move for the given player

#### `PerfectEngine`
A built-in engine that uses minimax with alpha-beta pruning for perfect play.

**Methods:**

- `new()` - Creates a new perfect engine

## Performance

The PerfectEngine uses minimax with alpha-beta pruning for optimal play while maintaining excellent performance:

- **Move calculation**: Typically < 1ms for mid-game positions
- **Full game simulation**: ~1.6ms per game with two perfect engines
- **Throughput**: ~614 games/second
- **Suitable for**: Running thousands of simulations quickly

## Future Plans

- Python bindings via PyO3 for easy Python integration
- Additional engine implementations (Random, Monte Carlo Tree Search, Neural Networks)
- Difficulty levels for engines
- Position analysis and evaluation functions
- Opening book support
- Multi-threaded batch simulations

## Building from Source

```bash
git clone https://github.com/Sumanth-NR/zttt-rs.git
cd zttt-rs
cargo build --release
```

## Running Tests

```bash
cargo test
```

## Benchmarking

```bash
cargo bench
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Inspiration

Inspired by [ZTicTacToe](https://github.com/ZTicTacToe) in Python, this crate aims to provide even faster simulations for researchers and developers working with TicTacToe game analysis.

## Author

Sumanth-NR

## Acknowledgments

- The Rust community for excellent tooling and documentation
- ZTicTacToe project for inspiration