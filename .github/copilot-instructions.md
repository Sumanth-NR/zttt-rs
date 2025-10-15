# GitHub Copilot Instructions for zttt-rs

## Project Overview

**zttt-rs** is the fastest and most optimized Rust backend for simulating TicTacToe games. The primary goal is to provide high-throughput game simulation capabilities with minimal memory footprint and maximum performance.

## Core Principles

1. **Performance First**: Every change should consider performance impact. The target is ~1.8M games/second with ~0.55µs per game (FastEngine).
2. **Minimal Memory**: Keep the memory footprint small for large-scale simulations.
3. **Clean API**: Maintain a simple, intuitive interface for users.
4. **Zero Dependencies**: Keep the core library dependency-free for maximum portability and minimal overhead.
5. **Speed Over Complexity**: Prioritize simulation throughput; complex AI algorithms belong in examples.

## Project Structure

```
src/
├── lib.rs       - Public API and re-exports
├── board.rs     - Board representation and game logic
├── player.rs    - Player and Cell types
├── game.rs      - GameResult type
└── engine.rs    - Engine trait and FastEngine implementation

examples/
├── basic_game.rs       - Simple game example
├── fast_simulation.rs  - High-speed simulation example (FastEngine)
├── ai_game.rs          - AI vs AI example (PerfectEngine included)
├── simulation.rs       - Batch simulation example (PerfectEngine included)
└── benchmark.rs        - Performance benchmarks (both engines)
```

## Architecture

### Core Types

- **`Board`**: 3x3 grid using `[[Cell; 3]; 3]` for optimal performance
- **`Player`**: Enum with X and O variants
- **`Cell`**: Enum representing Empty or Occupied(Player)
- **`GameResult`**: Enum representing Win(Player), Draw, or InProgress

### Engine System

The `Engine` trait allows pluggable move selection strategies:
```rust
pub trait Engine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)>;
}
```

The `FastEngine` implements a high-speed strategy (first valid move) for maximum throughput.

The `PerfectEngine` (available in examples/) implements minimax with alpha-beta pruning as a benchmark reference.

## Coding Guidelines

### Performance Considerations

1. **Use arrays over vectors** for fixed-size data structures
2. **Clone boards efficiently** - the board is designed to be cheaply cloneable
3. **Avoid allocations in hot paths** - especially in core simulation loops
4. **Use early returns** in game result checking
5. **Keep engines simple** - FastEngine should have near-zero overhead
6. **Complex algorithms in examples** - Keep minimax and other AI algorithms in examples, not core library

### Code Style

1. **Documentation**: All public APIs must have doc comments with examples
2. **Error Handling**: Use `Result<T, &'static str>` for operations that can fail
3. **Testing**: Write tests for all game logic, especially edge cases
4. **Examples**: Keep examples simple and focused on specific use cases

### Naming Conventions

- Use descriptive names: `valid_moves()`, `game_result()`, `make_move()`
- Engine implementations end with "Engine": `PerfectEngine`, `RandomEngine`
- Test functions start with `test_`: `test_win_row()`, `test_draw()`

## Common Tasks

### Adding a New Engine

For high-speed engines (to be included in core library):
1. Implement the `Engine` trait in `src/engine.rs`
2. Keep implementation minimal and fast
3. Add tests in `src/lib.rs`
4. Create an example showing usage
5. Benchmark performance

For complex AI engines (examples only):
1. Create the engine in an example file or as local struct
2. Implement the `Engine` trait
3. Document the algorithm and trade-offs
4. Compare performance to FastEngine

Example (simple engine for core):
```rust
#[derive(Debug, Clone, Copy)]
pub struct RandomEngine;

impl Engine for RandomEngine {
    fn choose_move(&self, board: &Board, _player: Player) -> Option<(usize, usize)> {
        let moves = board.valid_moves();
        moves.into_iter().next()  // Keep it fast!
    }
}
```

### Optimizing Performance

1. Profile with `cargo bench` (run `examples/benchmark.rs`)
2. Check memory usage with simulation examples
3. Use `cargo flamegraph` for detailed profiling
4. Focus on hot paths: `minimax()`, `game_result()`, `valid_moves()`

### Adding Board Features

When adding board functionality:
1. Keep the core representation immutable where possible
2. Provide both mutable (`make_move`) and query methods (`is_valid_move`, `get`)
3. Update `game_result()` if win conditions change
4. Add comprehensive tests

## Testing Strategy

### Test Coverage

- **Unit tests**: Test individual methods and functions
- **Integration tests**: Test full game flows in examples
- **Doc tests**: Ensure documentation examples compile and run
- **Edge cases**: Test boundary conditions, invalid moves, game-over states

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_win_row

# Run doc tests only
cargo test --doc

# Run benchmarks
cargo run --example benchmark --release
```

## Performance Targets

- **Game simulation**: < 2ms per complete game
- **Throughput**: > 500 games/second
- **Move generation**: < 1µs for typical positions
- **Memory**: < 100 bytes per Board instance

## AI Assistant Guidelines

When helping with this codebase:

1. **Prioritize performance**: Suggest optimizations when relevant
2. **Keep it simple**: Don't over-engineer solutions
3. **Maintain zero dependencies**: Avoid suggesting external crates for core functionality
4. **Test thoroughly**: Generate tests for new functionality
5. **Document clearly**: Provide examples in documentation
6. **Consider simulation use cases**: Think about batch processing and high-throughput scenarios

## Common Patterns

### Creating and Playing a Game (High-Speed)

```rust
let mut board = Board::new();
let engine = FastEngine;
let mut current_player = Player::X;

while board.game_result() == GameResult::InProgress {
    if let Some((row, col)) = engine.choose_move(&board, current_player) {
        board.make_move(row, col, current_player)?;
        current_player = current_player.opponent();
    }
}
```

### Batch Simulation (Maximum Speed)

```rust
let engine = FastEngine;
for _ in 0..100_000 {
    let mut board = Board::new();
    let mut current_player = Player::X;
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        }
    }
}
```

## Anti-Patterns to Avoid

1. ❌ Don't add runtime dependencies without strong justification
2. ❌ Don't allocate in hot paths (game simulation loops)
3. ❌ Don't use `String` for error messages (use `&'static str`)
4. ❌ Don't break the public API without good reason
5. ❌ Don't add features that compromise performance for convenience
6. ❌ Don't add complex AI algorithms to the core library (use examples instead)
7. ❌ Don't optimize for single-game UX at the expense of batch simulation throughput

## Questions?

If you're unsure about:
- Performance implications → Run benchmarks
- API design → Check existing patterns in the codebase
- Testing → Look at existing tests in `src/lib.rs`
- Documentation → Follow the style in `src/lib.rs` doc comments
