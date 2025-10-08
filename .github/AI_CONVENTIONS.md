# AI Conventions and Guidelines

This document provides conventions and guidelines specifically for AI agents working with this repository.

## Project Overview

**zttt-rs** is a high-performance TicTacToe library in Rust featuring:
- Minimax algorithm with alpha-beta pruning for optimal play
- Simple, ergonomic API
- Fast performance (~1.6ms per game)
- Extensible engine system via traits

## Quick Context

### Key Files
- `src/lib.rs` - Main entry point, exports public API
- `src/board.rs` - Board state and game logic (move validation, win detection)
- `src/engine.rs` - AI engines (minimax implementation)
- `src/player.rs` - Player and Cell types
- `src/game.rs` - GameResult enum
- `examples/*.rs` - Usage examples

### Core Types
- `Board` - 3x3 game board with move validation
- `Player` - X or O
- `Cell` - Empty or Occupied(Player)
- `GameResult` - Win(Player), Draw, or InProgress
- `Engine` trait - Interface for move selection
- `PerfectEngine` - Minimax implementation

## Code Modification Guidelines

### General Rules
1. **Minimal Changes**: Make the smallest possible changes to achieve the goal
2. **No Breaking Changes**: Maintain backward compatibility unless explicitly required
3. **Type Safety**: Leverage Rust's type system for correctness
4. **Performance**: Avoid unnecessary allocations or complexity
5. **Documentation**: Document public APIs with examples

### When Adding Features
```rust
// DO: Add new methods with clear documentation
impl Board {
    /// Returns the cell at the specified position
    ///
    /// # Example
    /// ```
    /// let board = Board::new();
    /// assert_eq!(board.get(0, 0), Cell::Empty);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }
}

// DON'T: Break existing APIs
// Don't change method signatures without explicit requirement
```

### When Fixing Bugs
1. First understand the expected behavior
2. Add a failing test that demonstrates the bug
3. Fix the bug with minimal changes
4. Verify the test passes
5. Check that existing tests still pass

### Code Style
- Follow existing patterns in the codebase
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Keep functions focused and short
- Prefer composition over complex logic

## Testing Conventions

### Test Organization
- Unit tests go in the same file as the code: `#[cfg(test)] mod tests`
- Integration tests go in `tests/` directory (currently in lib.rs)
- Doc tests go in doc comments

### Test Naming
```rust
#[test]
fn test_<feature>_<scenario>() {
    // Clear, descriptive test names
}

// Examples:
test_make_move_valid()
test_make_move_out_of_bounds()
test_game_result_win_row()
```

### Test Structure
```rust
#[test]
fn test_feature() {
    // Arrange - set up test data
    let mut board = Board::new();
    
    // Act - perform the action
    let result = board.make_move(0, 0, Player::X);
    
    // Assert - verify the result
    assert!(result.is_ok());
}
```

## Common Patterns

### Error Handling
```rust
// Use Result for operations that can fail
pub fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), &str> {
    if row >= 3 || col >= 3 {
        return Err("Move out of bounds");
    }
    // ...
    Ok(())
}
```

### Using the Engine Trait
```rust
// Implement the Engine trait for custom engines
impl Engine for MyEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Custom logic here
    }
}
```

### Board Manipulation
```rust
// Always validate before modifying
if board.game_result() != GameResult::InProgress {
    return None;
}

// Clone for simulation (minimax)
let mut new_board = board.clone();
new_board.make_move(row, col, player).unwrap();
```

## Build and Test Commands

### Essential Commands
```bash
# Build the project
cargo build

# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-targets --all-features

# Build documentation
cargo doc --no-deps

# Run specific example
cargo run --example ai_game
```

### Before Committing
Always run:
1. `cargo fmt` - Format code
2. `cargo clippy` - Check for issues
3. `cargo test` - Verify tests pass
4. `cargo build --release` - Ensure release builds

## Performance Considerations

### Hot Paths
- `minimax()` - Called repeatedly during move selection
- `game_result()` - Called frequently to check game state
- `valid_moves()` - Called for every minimax node

### Optimization Guidelines
1. Avoid allocations in hot paths
2. Use `Copy` types where possible (Player, Cell)
3. Prefer array indexing over iteration
4. Don't add complex logic without benchmarking

### Benchmarking
```bash
# Run performance benchmark
cargo run --example benchmark --release

# Should maintain ~1.6ms per game, ~614 games/sec
```

## Documentation Guidelines

### Public API Documentation
```rust
/// Brief one-line description
///
/// Longer description with more details.
///
/// # Arguments
///
/// * `arg` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this fails (if Result)
///
/// # Example
///
/// ```
/// use zttt_rs::{Board, Player};
/// let mut board = Board::new();
/// board.make_move(0, 0, Player::X).unwrap();
/// ```
pub fn method(&self) -> Type { }
```

### Module Documentation
```rust
//! Module description
//!
//! Detailed explanation of what this module provides.
```

## Common Pitfalls to Avoid

### 1. Array Bounds
```rust
// WRONG: No bounds checking
board.cells[row][col] = value;

// RIGHT: Check bounds first
if row >= 3 || col >= 3 {
    return Err("Out of bounds");
}
board.cells[row][col] = value;
```

### 2. Game State Validation
```rust
// WRONG: Allow moves after game over
pub fn make_move(&mut self, ...) {
    self.cells[row][col] = Cell::Occupied(player);
}

// RIGHT: Check game state first
pub fn make_move(&mut self, ...) {
    if self.game_result() != GameResult::InProgress {
        return Err("Game is over");
    }
    // ...
}
```

### 3. Unnecessary Allocations
```rust
// WRONG: Allocate vector unnecessarily
let moves: Vec<_> = (0..3).flat_map(|r| (0..3).map(move |c| (r, c))).collect();

// RIGHT: Use array or iterate directly
for row in 0..3 {
    for col in 0..3 {
        // ...
    }
}
```

## Architecture Understanding

### Data Flow
```
User Input → Board::make_move() → Validate → Update State
                                              ↓
                                   Board::game_result()
                                              ↓
                                   Return GameResult

AI Request → Engine::choose_move() → Get valid moves
                                              ↓
                                   For each move: minimax()
                                              ↓
                                   Return best move
```

### Module Dependencies
```
lib.rs
├── player (no dependencies)
├── game (depends on: player)
├── board (depends on: player, game)
└── engine (depends on: board, player, game)
```

## AI-Friendly Features

This repository includes:
- ✅ Clear module structure
- ✅ Comprehensive documentation
- ✅ Type safety via Rust
- ✅ Examples for common use cases
- ✅ Unit and integration tests
- ✅ Performance benchmarks
- ✅ CI/CD pipeline

## Questions to Ask Before Changing

1. Does this change maintain backward compatibility?
2. Are there existing tests that cover this area?
3. Should this be a new function or modify an existing one?
4. Does this affect performance-critical paths?
5. Is this change minimal and focused?
6. Does this require documentation updates?

## Integration Points

### For Adding New Features
- Add to appropriate module (`board.rs`, `engine.rs`, etc.)
- Export from `lib.rs` if public
- Add tests in the same file
- Document with examples
- Update README.md if user-facing

### For Fixing Bugs
- Locate the bug in the appropriate module
- Add a failing test first
- Make minimal fix
- Verify existing tests still pass
- Update documentation if behavior changed

## Useful Patterns

### Pattern: Validating Input
```rust
// Consistent validation pattern
if condition_invalid {
    return Err("Clear error message");
}
// Continue with valid input
```

### Pattern: Checking Game State
```rust
// Always check if game is in progress
if self.game_result() != GameResult::InProgress {
    return None; // or appropriate error
}
```

### Pattern: Engine Implementation
```rust
impl Engine for CustomEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // 1. Check if game is over
        if board.game_result() != GameResult::InProgress {
            return None;
        }
        
        // 2. Get valid moves
        let moves = board.valid_moves();
        
        // 3. Evaluate and select move
        // ...
        
        // 4. Return best move
        Some((row, col))
    }
}
```

## Summary

When working on this repository:
1. **Understand** the existing structure first
2. **Follow** established patterns and conventions
3. **Test** thoroughly before committing
4. **Document** public APIs with examples
5. **Optimize** only when necessary
6. **Ask** if unsure about approach

The goal is to maintain a clean, performant, well-documented codebase that's easy to understand and extend.
