# Architecture Documentation

## Overview

zttt-rs is designed as a high-performance TicTacToe game simulation backend. The architecture prioritizes speed, minimal memory usage, and simplicity.

## Design Goals

1. **Performance**: Achieve >500 games/second throughput
2. **Minimal Memory**: Use stack allocation and arrays instead of heap allocations
3. **Zero Dependencies**: Keep the core library dependency-free
4. **Pluggable Engines**: Allow custom AI implementations via trait
5. **Type Safety**: Leverage Rust's type system to prevent invalid states

## Core Architecture

```
┌─────────────────────────────────────────┐
│           Public API (lib.rs)           │
│  Board, Player, Cell, GameResult,       │
│  Engine, PerfectEngine                  │
└─────────────────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
┌──────────────┐        ┌──────────────┐
│  Game Logic  │        │   Engine     │
│  (board.rs)  │◄───────│ (engine.rs)  │
└──────────────┘        └──────────────┘
        │                       │
        ▼                       ▼
┌──────────────┐        ┌──────────────┐
│    Types     │        │   Minimax    │
│ (player.rs,  │        │  Algorithm   │
│  game.rs)    │        │ + α-β Prune  │
└──────────────┘        └──────────────┘
```

## Module Breakdown

### lib.rs
**Purpose**: Public API surface and re-exports

**Responsibilities**:
- Re-export public types and traits
- Provide module-level documentation
- Contain integration tests

**Key Decisions**:
- Keep the public API minimal and focused
- Tests are co-located for visibility
- Documentation examples are executable (doc tests)

### board.rs
**Purpose**: Game state representation and core game logic

**Key Types**:
- `Board`: Contains `[[Cell; 3]; 3]` array
- Methods for move validation, execution, and game state queries

**Design Decisions**:
- **Array-based storage**: `[[Cell; 3]; 3]` is stack-allocated and cache-friendly
- **Public(crate) cells field**: Allows efficient engine access without getter overhead
- **Immutable queries**: `get()`, `valid_moves()`, `game_result()` don't mutate
- **Mutable operations**: Only `make_move()` and `reset()` mutate state

**Performance Characteristics**:
- `new()`: O(1) - stack allocation
- `make_move()`: O(1) - single array write
- `game_result()`: O(1) - check 8 win conditions
- `valid_moves()`: O(9) - iterate all cells
- Memory: 9 bytes (9 enum variants) + padding

### player.rs
**Purpose**: Player and cell representations

**Key Types**:
- `Player`: X or O (2 bytes with discriminant)
- `Cell`: Empty or Occupied(Player) (2 bytes)

**Design Decisions**:
- Use enums for type safety and pattern matching
- Implement `Copy` for zero-cost passing
- `opponent()` method for common operation
- Display trait for debugging/visualization

### game.rs
**Purpose**: Game result type

**Key Type**:
- `GameResult`: Win(Player), Draw, or InProgress

**Design Decisions**:
- Separate type for clarity (not mixing with Cell/Player)
- Implements `Copy`, `PartialEq` for easy comparison
- Simple enum for pattern matching in game loops

### engine.rs
**Purpose**: AI move selection trait and implementations

**Key Components**:
- `Engine` trait: Interface for move selection
- `PerfectEngine`: Optimal play using minimax + alpha-beta pruning

**Algorithm - Minimax with Alpha-Beta Pruning**:

```
function minimax(board, maximizing_player, current_player, alpha, beta, is_maximizing):
    if game_over:
        return evaluation
    
    if is_maximizing:
        max_eval = -∞
        for each valid move:
            eval = minimax(new_board, ..., alpha, beta, false)
            max_eval = max(max_eval, eval)
            alpha = max(alpha, eval)
            if beta <= alpha:
                break  // Pruning!
        return max_eval
    else:
        min_eval = +∞
        for each valid move:
            eval = minimax(new_board, ..., alpha, beta, true)
            min_eval = min(min_eval, eval)
            beta = min(beta, eval)
            if beta <= alpha:
                break  // Pruning!
        return min_eval
```

**Design Decisions**:
- Trait-based for extensibility
- Perfect engine guarantees optimal play
- Alpha-beta pruning reduces search space by ~50%
- Direct cell access (`board.cells`) for performance
- Board cloning is cheap (72 bytes on stack)

## Performance Analysis

### Critical Paths (Hot Code)

1. **`Engine::choose_move()`**
   - Called once per move
   - Triggers full minimax search tree
   - Optimization: Alpha-beta pruning, move ordering

2. **`minimax()` recursion**
   - Called thousands of times per move selection
   - Optimization: Early termination, minimal allocations, direct cell access

3. **`Board::game_result()`**
   - Called at each minimax node
   - Optimization: Fast win-condition checks, early returns

4. **`Board::valid_moves()`**
   - Called for move generation
   - Optimization: Simple iteration, Vec allocation only when needed

### Memory Layout

```
Board: 9 bytes (9 × Cell)
Cell:  2 bytes (enum discriminant + Player)
Player: 1 byte (enum discriminant)

Stack frame for game: ~100 bytes
Minimax recursion: ~1KB peak (depth ≤ 9)
```

## Data Flow

### Single Move Selection

```
User Request
    │
    ▼
Engine::choose_move(board, player)
    │
    ├─► Board::valid_moves()
    │   └─► Returns Vec<(usize, usize)>
    │
    ├─► For each move:
    │   ├─► Clone board
    │   ├─► Apply move
    │   └─► minimax(board, ...)
    │       ├─► Board::game_result()
    │       └─► Recursive search...
    │
    └─► Return best move
```

### Complete Game Simulation

```
Initialize Board
    │
    ▼
While game in progress:
    │
    ├─► Engine::choose_move()
    │   └─► (minimax search)
    │
    ├─► Board::make_move()
    │
    ├─► Switch player
    │
    └─► Check Board::game_result()
```

## Extensibility Points

### Custom Engines

Implement the `Engine` trait:

```rust
pub struct MyEngine {
    // Your state/config
}

impl Engine for MyEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Your strategy:
        // - Random selection
        // - Heuristic-based
        // - Limited-depth minimax
        // - Neural network
        // - etc.
    }
}
```

### Custom Board Operations

The board can be extended with helper methods:

```rust
impl Board {
    pub fn custom_analysis(&self) -> AnalysisResult {
        // Use self.cells or public methods
    }
}
```

## Testing Strategy

### Test Pyramid

```
           ┌─────────────┐
           │   Examples  │ ← Integration/Simulation tests
           │ (4 examples)│
           └─────────────┘
          ┌───────────────┐
          │  Unit Tests   │ ← Core logic tests
          │  (17 tests)   │
          └───────────────┘
         ┌─────────────────┐
         │   Doc Tests     │ ← API usage verification
         │   (2 tests)     │
         └─────────────────┘
```

### Test Coverage

- **Win conditions**: All 8 ways to win (3 rows, 3 cols, 2 diagonals)
- **Draw conditions**: Full board without winner
- **Invalid moves**: Out of bounds, occupied cells, game-over state
- **Engine behavior**: Optimal moves, blocking, taking wins
- **Edge cases**: Empty board, first move, last move

## Benchmarking

### Benchmarking Approach

1. **Micro-benchmarks**: Individual operations (move generation, result checking)
2. **Game benchmarks**: Complete game simulations
3. **Batch benchmarks**: High-throughput scenarios (1k, 10k, 100k games)

### Current Performance (Baseline)

- Single game: ~1.6ms
- Throughput: ~614 games/second
- Move calculation (mid-game): ~150µs
- Move calculation (opening): ~1.5ms

## Future Optimization Opportunities

1. **Move Ordering**: Try center/corners first in minimax
2. **Transposition Table**: Cache evaluated positions
3. **Bitboard Representation**: Use u16 instead of array (requires major refactor)
4. **Iterative Deepening**: For time-limited scenarios
5. **SIMD**: Vectorize win-condition checks (likely overkill)

## Design Trade-offs

| Decision | Pro | Con |
|----------|-----|-----|
| Array-based board | Fast, cache-friendly | Limited to 3×3 |
| Public(crate) cells | Engine performance | Less encapsulation |
| Clone boards in minimax | Simple, safe | Some memory churn |
| Trait-based engines | Extensible | Virtual call overhead |
| Zero dependencies | Fast compile, portable | Implement everything |

## Conclusion

The architecture achieves its goals through:
- Simple, focused design
- Performance-conscious data structures
- Careful algorithm implementation
- Extensibility via traits
- Comprehensive testing

The result is a fast, reliable TicTacToe simulation backend suitable for research, benchmarking, and integration into larger systems.
