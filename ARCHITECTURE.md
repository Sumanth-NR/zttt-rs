# Architecture Overview

## Vision

**zttt-rs** is the fastest and most optimized Rust backend for simulating TicTacToe games. The architecture is designed around clear abstractions that prioritize high-speed simulations over complex AI algorithms.

## Core Abstractions

### 1. Game Logic: `Board`

The `Board` provides all game state management and validation procedures:

```rust
pub struct Board {
    cells: [[Cell; 3]; 3],
}

// Clear procedures for game logic
impl Board {
    pub fn new() -> Self                           // Initialize empty board
    pub fn get(&self, row, col) -> Option<Cell>    // Query cell state
    pub fn make_move(&mut self, row, col, player)  // Apply move
    pub fn is_valid_move(&self, row, col) -> bool  // Validate move
    pub fn valid_moves(&self) -> Vec<(usize, usize)> // Get all valid moves
    pub fn game_result(&self) -> GameResult        // Check game status
    pub fn reset(&mut self)                        // Reset to empty
}
```

**Design Principle**: Board is responsible for game rules, not strategy.

### 2. Move Selection: `Engine` Trait

The `Engine` trait provides a clean abstraction for different move selection strategies:

```rust
pub trait Engine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)>;
}
```

This simple interface allows:
- Custom strategies without modifying core game logic
- Easy testing and benchmarking of different approaches
- Generic simulation code that works with any engine

**Design Principle**: Engines implement strategy, not game rules.

### 3. High-Speed Implementation: `FastEngine`

The core library provides `FastEngine` for maximum throughput:

```rust
pub struct FastEngine;

impl Engine for FastEngine {
    fn choose_move(&self, board: &Board, _player: Player) -> Option<(usize, usize)> {
        board.valid_moves().into_iter().next()  // First valid move
    }
}
```

**Performance**: ~0.55µs per game, ~1.8M games/second

**Design Principle**: Prioritize speed over move quality for simulations.

### 4. Benchmark Reference: `PerfectEngine` (Examples Only)

Complex AI algorithms are kept in examples for comparison:

```rust
// In examples/benchmark.rs, simulation.rs, etc.
struct PerfectEngine;

impl Engine for PerfectEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Minimax with alpha-beta pruning
        // ... complex algorithm ...
    }
}
```

**Performance**: ~1.8ms per game, ~554 games/second

**Design Principle**: Keep complex algorithms separate from core library.

## Clear Procedures

### Simulating a Game

```rust
use zttt_rs::{Board, Player, GameResult, FastEngine, Engine};

fn simulate_game() -> GameResult {
    let mut board = Board::new();
    let engine = FastEngine;
    let mut current_player = Player::X;
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        }
    }
    
    board.game_result()
}
```

### Batch Simulations

```rust
// Ultra-fast batch processing
let engine = FastEngine;

for _ in 0..1_000_000 {
    let result = simulate_game();
    // Process result...
}
```

### Custom Engine Implementation

```rust
struct MyEngine {
    // Configuration
}

impl Engine for MyEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Your strategy here
        // Use board procedures:
        // - board.valid_moves()
        // - board.is_valid_move(row, col)
        // - board.game_result()
        // - board.get(row, col)
    }
}
```

## Separation of Concerns

### Core Library (src/)
- **Purpose**: Maximum speed, minimal overhead
- **Contains**: FastEngine, Board, core types
- **Focus**: High-throughput simulations
- **Dependencies**: Zero runtime dependencies

### Examples (examples/)
- **Purpose**: Demonstrations and benchmarks
- **Contains**: PerfectEngine, comparisons, use cases
- **Focus**: Show different strategies and trade-offs
- **Dependencies**: None (self-contained)

## Performance Hierarchy

```
FastEngine (Core)
  ↓
  Optimized for: Maximum throughput
  Speed: ~0.55µs/game (~1.8M games/sec)
  Use case: Large-scale simulations
  
PerfectEngine (Examples)
  ↓
  Optimized for: Optimal play
  Speed: ~1.8ms/game (~554 games/sec)
  Use case: Strategy comparison, benchmarking
```

## Design Benefits

1. **Clear Abstractions**: Board handles game logic, Engine handles strategy
2. **Easy Extension**: Implement Engine trait for custom strategies
3. **Performance**: FastEngine keeps simulations blazing fast
4. **Flexibility**: Use different engines for different scenarios
5. **Maintainability**: Separation keeps codebase simple and focused

## Examples

See the `examples/` directory for:
- `basic_game.rs` - Manual game demonstration
- `fast_simulation.rs` - High-speed batch simulations
- `custom_engine.rs` - Custom engine implementation
- `simulation.rs` - PerfectEngine batch simulations
- `benchmark.rs` - Performance comparisons
- `ai_game.rs` - AI vs AI gameplay

## Future Extensions

The architecture supports easy addition of:
- New engine strategies (implement Engine trait)
- Performance optimizations (Board internals)
- Analysis tools (consume GameResult)
- Custom game rules (extend Board)

All while maintaining the core focus: **high-speed game simulations**.
