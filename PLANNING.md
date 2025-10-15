# zttt-rs Reorganization & Optimization Planning

## Overview

This document outlines the reorganization of zttt-rs to support future enhancements while maintaining the core vision of being the fastest and most optimized TicTacToe simulation backend.

## Motivation

The original structure mixed core game logic with simulation code in examples. To scale the framework and add advanced simulation capabilities, we need:

1. **Clear separation of concerns**: Backend (game logic) vs Simulation (batch processing)
2. **Better modularity**: Each component has a focused responsibility
3. **Room for growth**: New features can be added without cluttering the core
4. **Maintained performance**: Zero overhead from reorganization

## New Architecture

```
zttt-rs/
├── src/
│   ├── lib.rs              # Main entry point
│   ├── backend/            # Core game logic (STABLE)
│   │   ├── mod.rs          # Backend module entry
│   │   ├── board.rs        # Board representation and game logic
│   │   ├── player.rs       # Player and Cell types
│   │   ├── game.rs         # GameResult type
│   │   └── engine.rs       # Engine trait and FastEngine
│   └── simulation/         # Simulation framework (IN PLANNING)
│       ├── mod.rs          # Simulation module with TODO outline
│       └── README.md       # Detailed planning document
├── examples/               # Example usage (UNCHANGED)
└── docs/                   # Documentation
```

## Module Responsibilities

### Backend Module (`src/backend/`)

**Purpose**: Provide core game mechanics with maximum performance

**Responsibilities**:
- Board state representation and manipulation
- Game rule enforcement
- Win/draw detection
- Move validation
- Engine trait definition
- FastEngine implementation

**Stability**: This is the stable core that should rarely change

**Performance Target**: Maintain current performance (~1.8M games/sec with FastEngine)

### Simulation Module (`src/simulation/`)

**Purpose**: High-performance batch simulation framework

**Responsibilities**:
- Sequential simulation runner
- Parallel/multi-threaded simulation
- Configuration management
- Statistics collection and analysis
- Tournament system
- Result streaming
- Memory optimization

**Status**: Currently in planning phase (see [`src/simulation/README.md`](src/simulation/README.md))

**Performance Target**: 
- Sequential: Match backend performance
- Parallel: Near-linear scaling up to 8+ cores

## Module Structure

**Direct Module Access**: All components are accessed through their respective modules:

```rust
// Import from backend module
use zttt_rs::backend::{Board, Player, GameResult, FastEngine, Engine};

let mut board = Board::new();
let engine = FastEngine;
// ... code ...
```

**Internal Structure**: 
- Core game logic: `src/backend/` (board, player, game, engine)
- Simulation framework: `src/simulation/` (planned)
- No root-level re-exports - use modules directly

## Implementation Plan

### Phase 1: Reorganization (COMPLETED)

- [x] Create `src/backend/` directory
- [x] Move core files to backend module
- [x] Create `backend/mod.rs` to expose backend API
- [x] Update internal imports
- [x] Verify all tests pass
- [x] Verify all examples work
- [x] Update documentation

**Status**: ✅ Complete
**Performance Impact**: Zero (no code changes, only file moves)

### Phase 2: Simulation Planning (COMPLETED)

- [x] Create `src/simulation/` directory
- [x] Create `simulation/mod.rs` with comprehensive TODO outline
- [x] Create `simulation/README.md` with detailed design
- [x] Document API design
- [x] Define performance targets
- [x] Outline implementation phases

**Status**: ✅ Complete
**Next**: Review and approve simulation design before implementation

### Phase 3: Simulation Implementation (FUTURE)

Implementation will proceed in phases (see `src/simulation/README.md`):

1. **Phase 1**: Core sequential simulator (1-2 weeks)
2. **Phase 2**: Parallel execution (1 week)
3. **Phase 3**: Statistics system (1 week)
4. **Phase 4**: Advanced features (2-3 weeks)
5. **Phase 5+**: Polish and optimization (2 weeks)

**Total Timeline**: 7-9 weeks for complete implementation

### Phase 4: Optimization Opportunities (FUTURE)

Once simulation module is implemented, consider:

1. **Backend Optimizations**:
   - SIMD for board operations
   - Bitboard representation
   - Faster win detection
   - Move generation optimization

2. **Simulation Optimizations**:
   - Lock-free data structures
   - Work-stealing improvements
   - NUMA-aware scheduling
   - Result caching for deterministic engines

3. **Advanced Features**:
   - GPU acceleration for massive simulations
   - Distributed simulation across machines
   - Real-time monitoring and visualization
   - Integration with analysis tools

## Performance Guarantees

### Current Performance (Maintained)
- **FastEngine**: ~1.8M games/sec (sequential)
- **Per-game overhead**: ~0.55µs
- **Memory per board**: < 100 bytes

### Target Performance (Future Simulation Module)
- **Sequential**: Match or exceed current examples
- **Parallel (8 cores)**: 12-14M games/sec
- **Statistics overhead**: < 5% for basic, < 10% for detailed
- **Memory**: < 1KB per 1000 games

## Migration Guide

### For Backend Module Users

Access backend components directly through the module:

```rust
// Import from backend module
use zttt_rs::backend::{Board, Player, FastEngine, Engine};
```

### For Future Simulation Users

When the simulation module is implemented:

```rust
// New simulation API (future)
use zttt_rs::simulation::{Simulator, SimulationConfig};
use zttt_rs::{FastEngine, Player};

let config = SimulationConfig::builder()
    .num_games(10_000)
    .engine(FastEngine)
    .starting_player(Player::X)
    .build();

let result = Simulator::new(config).run_sequential();
```

## Design Principles

1. **Backend Stability**: Core game logic is stable and changes infrequently
2. **Simulation Flexibility**: Simulation module can evolve with new features
3. **Zero Overhead**: Unused features have no performance cost
4. **Simple Defaults**: Common cases are trivial, advanced features available when needed
5. **Type Safety**: Leverage Rust's type system to prevent errors
6. **Performance First**: Every change must maintain or improve performance

## Success Criteria

### Reorganization (Current Phase)
- [x] All tests pass
- [x] All examples work
- [x] No performance regression
- [x] Public API unchanged
- [x] Clear module boundaries
- [x] Comprehensive planning documents

### Simulation Implementation (Future)
- [ ] Sequential simulator matches current performance
- [ ] Parallel simulator achieves 7-8x scaling on 8 cores
- [ ] Statistics collection < 5% overhead
- [ ] Clean, intuitive API
- [ ] Comprehensive tests
- [ ] Documentation with examples

## Questions & Decisions

### Resolved
1. ✅ **Module naming**: "backend" chosen for clarity (core game logic)
2. ✅ **Module access**: Direct module imports (no root re-exports)
3. ✅ **Planning approach**: Detailed TODOs in code + comprehensive README

### Open (For Future Phases)
1. Should simulation module support async/await?
2. GPU acceleration worth the complexity?
3. How to handle engine state in multi-threaded context?
4. Built-in visualization vs external tools?

## Resources

- **Simulation Planning**: [`src/simulation/README.md`](src/simulation/README.md)
- **Simulation TODOs**: [`src/simulation/mod.rs`](src/simulation/mod.rs)
- **Architecture**: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- **Performance**: Examples in `examples/` directory

## Next Steps

1. ✅ Complete reorganization
2. ✅ Create comprehensive planning documents
3. 🔄 Review and approve simulation design
4. ⏳ Begin Phase 1 simulation implementation
5. ⏳ Iterative development and benchmarking

---

**Last Updated**: 2025-10-15  
**Status**: Reorganization complete, ready for simulation implementation  
**Performance**: Maintained (no regression from reorganization)
