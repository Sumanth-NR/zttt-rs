# Migration Guide: Alpha-Beta Pruning to High-Speed Architecture

## Overview

This document describes the restructuring of zttt-rs to focus on high-speed simulations. The alpha-beta pruning engine has been moved from the core library to examples, and a new FastEngine has been introduced for maximum throughput.

## What Changed

### Before (v0.x)

**Core Library Exports**:
```rust
pub use engine::{Engine, PerfectEngine};
```

**Typical Usage**:
```rust
use zttt_rs::{Board, Player, PerfectEngine, Engine};

let engine = PerfectEngine::new();
let next_move = engine.choose_move(&board, player);

// Performance: ~1.8ms per game, ~554 games/second
```

**Focus**: Optimal play with minimax algorithm

### After (Current)

**Core Library Exports**:
```rust
pub use engine::{Engine, FastEngine};
```

**Typical Usage**:
```rust
use zttt_rs::{Board, Player, FastEngine, Engine};

let engine = FastEngine;
let next_move = engine.choose_move(&board, player);

// Performance: ~0.55µs per game, ~1.8M games/second
```

**Focus**: Maximum simulation throughput

## Migration Steps

### For Users Who Need Speed (Recommended)

**Old Code**:
```rust
use zttt_rs::{Board, Player, PerfectEngine, Engine};

let engine = PerfectEngine::new();
```

**New Code**:
```rust
use zttt_rs::{Board, Player, FastEngine, Engine};

let engine = FastEngine;  // 3,200x faster!
```

**Benefit**: ~3,200x performance improvement for simulations

### For Users Who Need Perfect Play

If you need optimal play (minimax with alpha-beta pruning), include PerfectEngine directly in your code:

```rust
use zttt_rs::{Board, Player, GameResult, Engine};

// Include PerfectEngine implementation
struct PerfectEngine;

impl PerfectEngine {
    fn new() -> Self { PerfectEngine }
    
    fn minimax(&self, board: &Board, ...) -> i32 {
        // Copy from examples/benchmark.rs or examples/simulation.rs
        // See these files for complete implementation
    }
}

impl Engine for PerfectEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Copy from examples/benchmark.rs or examples/simulation.rs
    }
}
```

**Alternative**: Copy the implementation from any of these example files:
- `examples/benchmark.rs`
- `examples/simulation.rs`
- `examples/ai_game.rs`

## Performance Comparison

| Engine | Location | Speed | Throughput | Use Case |
|--------|----------|-------|------------|----------|
| FastEngine | Core library | ~0.55µs/game | ~1.8M games/sec | High-speed simulations |
| PerfectEngine | Examples only | ~1.8ms/game | ~554 games/sec | Optimal play reference |

## Why This Change?

### Vision Alignment

The repository's core vision is to be **"the fastest and most optimized Rust backend for simulating TicTacToe games."**

The previous architecture included complex AI in the core library, which:
- Slowed down simulations significantly (~3,200x)
- Added complexity to the core API
- Mixed game logic with AI strategy

### New Architecture Benefits

1. **Performance**: 3,200x faster simulations for research and testing
2. **Simplicity**: Core library focuses on game logic and speed
3. **Flexibility**: PerfectEngine still available in examples for comparison
4. **Clarity**: Clear separation between game procedures and AI strategy

## Example Code

### High-Speed Batch Simulations

```rust
use zttt_rs::{Board, Player, GameResult, FastEngine, Engine};

// Simulate 1 million games
let engine = FastEngine;
let mut results = Vec::new();

for _ in 0..1_000_000 {
    let mut board = Board::new();
    let mut current_player = Player::X;
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        }
    }
    
    results.push(board.game_result());
}

// Completes in ~0.55 seconds (vs ~30 minutes with PerfectEngine!)
```

### Custom Strategy Development

```rust
use zttt_rs::{Board, Player, Engine};

struct MyCustomEngine {
    // Your configuration
}

impl Engine for MyCustomEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Implement your strategy using board procedures:
        let valid_moves = board.valid_moves();
        // ... your logic ...
    }
}
```

## Running Examples

```bash
# High-speed simulation (100k games)
cargo run --example fast_simulation --release

# Perfect play simulation (10k games) for comparison
cargo run --example simulation --release

# Custom engine demonstration
cargo run --example custom_engine

# Performance benchmarks
cargo run --example benchmark --release
```

## Documentation Updates

- `README.md` - Updated with new API and performance metrics
- `ARCHITECTURE.md` - New file documenting design principles
- `.ai-context.md` - Updated with current architecture
- `.github/copilot-instructions.md` - Updated guidelines

## Questions?

See the following resources:
- `ARCHITECTURE.md` - Design principles and abstractions
- `examples/custom_engine.rs` - How to implement custom strategies
- `examples/fast_simulation.rs` - High-speed simulation example
- `examples/benchmark.rs` - Performance comparisons

## Breaking Changes Summary

1. `PerfectEngine` no longer exported from core library
2. New `FastEngine` is the recommended default
3. PerfectEngine code available in examples for copying if needed

## Upgrade Checklist

- [ ] Replace `PerfectEngine` with `FastEngine` in imports
- [ ] Update engine instantiation (`FastEngine` instead of `PerfectEngine::new()`)
- [ ] Run benchmarks to verify performance improvement
- [ ] Copy PerfectEngine implementation if optimal play is needed
- [ ] Update any documentation referencing PerfectEngine

---

**Result**: Your simulations will be ~3,200x faster while maintaining the same clean API!
