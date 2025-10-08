# Copilot Instructions

## Project Overview
A high-performance TicTacToe library in Rust with minimax AI (~1.6ms per game, ~614 games/second).

## Structure
- `src/lib.rs` - Public API exports
- `src/board.rs` - Board state and game logic
- `src/engine.rs` - AI engine (minimax with alpha-beta pruning)
- `src/player.rs` - Player and Cell types
- `src/game.rs` - GameResult enum

## Key Principles
- **Minimal changes**: Make the smallest possible modifications
- **Performance first**: Avoid allocations in hot paths (minimax, game_result, valid_moves)
- **Type safety**: Leverage Rust's type system
- **No breaking changes**: Maintain backward compatibility

## Before Committing
```bash
cargo fmt          # Format code
cargo clippy       # Check for issues
cargo test         # Verify tests (17 unit + 2 doc tests)
cargo build --release
```

## Common Patterns
- Validate input before state changes
- Check `game_result() != InProgress` before moves
- Clone board for simulation in minimax
- Use `Result<(), &str>` for operations that can fail

## Testing
- Unit tests in same file: `#[cfg(test)] mod tests`
- Test naming: `test_<feature>_<scenario>`
- Include edge cases and error conditions
