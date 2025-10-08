# Architecture Overview

This document provides an overview of the zttt-rs architecture and design decisions.

## Design Philosophy

zttt-rs is designed with the following principles:

1. **Performance First**: Optimized for speed with minimal allocations
2. **Simple API**: Easy to use and understand
3. **Extensibility**: Plugin architecture via traits
4. **Type Safety**: Leverage Rust's type system for correctness

## Core Components

### Module Structure

```
src/
├── lib.rs       - Public API exports and top-level documentation
├── player.rs    - Player and Cell type definitions
├── game.rs      - GameResult type
├── board.rs     - Board state and game logic
└── engine.rs    - Move selection engines (AI)
```

### Component Details

#### `player.rs` - Core Types

**Player Enum**
- Represents the two players (X and O)
- Implements `opponent()` method for turn switching
- Copy type for efficiency

**Cell Enum**
- Represents board cell states (Empty or Occupied)
- Used internally by Board

#### `game.rs` - Game State

**GameResult Enum**
- `Win(Player)` - A player has won
- `Draw` - Game ended in a draw
- `InProgress` - Game is still ongoing

#### `board.rs` - Game Logic

**Board Struct**
- 3x3 array of cells
- Handles move validation
- Checks win/draw conditions
- Provides convenience methods

**Key Methods**:
- `new()` - Create empty board
- `make_move(row, col, player)` - Place a mark
- `game_result()` - Check game status
- `valid_moves()` - Get available moves
- `reset()` - Clear the board

**Internal Logic**:
- Validates moves (bounds, occupied cells)
- Checks rows, columns, diagonals for wins
- Detects draws (board full with no winner)

#### `engine.rs` - AI Implementation

**Engine Trait**
- Defines interface for move selection
- `choose_move(&board, player) -> Option<(usize, usize)>`
- Allows custom implementations

**PerfectEngine**
- Implements optimal play using minimax algorithm
- Alpha-beta pruning for efficiency
- Guarantees best possible moves
- Never loses when playing first

**Algorithm Details**:
- Minimax explores game tree recursively
- Alpha-beta pruning reduces search space
- Scoring: +10 for wins, -10 for losses, 0 for draws
- Depth-based adjustment for faster wins

## Data Flow

### Making a Move

```
User/Engine
    ↓
Board::make_move(row, col, player)
    ↓
Validate move (bounds, not occupied, game not over)
    ↓
Update cells[row][col]
    ↓
Return Result<(), &str>
```

### Game Result Check

```
Board::game_result()
    ↓
Check rows for win → Win(player)
    ↓
Check columns for win → Win(player)
    ↓
Check diagonals for win → Win(player)
    ↓
Check if board full → Draw
    ↓
Otherwise → InProgress
```

### AI Move Selection

```
Engine::choose_move(&board, player)
    ↓
Get valid_moves()
    ↓
For each move:
    ├─ Simulate move on cloned board
    ├─ Run minimax recursively
    └─ Track best score
    ↓
Return move with highest score
```

## Performance Characteristics

### Time Complexity
- `make_move()`: O(1)
- `game_result()`: O(1) - checks 8 lines
- `valid_moves()`: O(9) - checks all cells
- `minimax()`: O(9!) worst case, much better with pruning

### Space Complexity
- Board: 72 bytes (3x3 cells)
- Minimax recursion: O(depth) stack space
- No heap allocations during gameplay

## Design Decisions

### Why Array Instead of Vec?
- Fixed 3x3 size known at compile time
- Stack allocation (faster)
- No heap allocations or bounds checks at runtime
- Copy trait for free

### Why Minimax with Alpha-Beta Pruning?
- Guarantees optimal play
- Alpha-beta pruning reduces complexity
- TicTacToe small enough for full search
- No heuristics needed

### Why Trait-Based Engines?
- Extensibility: users can implement custom strategies
- Testing: easy to mock or create test engines
- Flexibility: different difficulty levels possible

### Why Separate Module Files?
- Clear separation of concerns
- Easy to navigate and understand
- Testability: each module can be tested independently
- Maintainability: changes localized to relevant modules

## Testing Strategy

- **Unit tests**: In each module for internal functions
- **Integration tests**: In lib.rs for public API
- **Doc tests**: Examples in documentation
- **Property tests**: Could add quickcheck/proptest

## Future Considerations

### Potential Enhancements
- Configurable board sizes (4x4, 5x5)
- Different rule variants
- More engine types (random, difficulty levels)
- Serialization support
- WASM compilation for web use

### Performance Optimizations
- Bitboard representation for faster operations
- Transposition tables for minimax
- Iterative deepening
- Opening book

## Contributing

When adding features, maintain these principles:
- Keep modules focused and cohesive
- Document public APIs thoroughly
- Add tests for new functionality
- Consider performance implications
- Maintain backward compatibility where possible
