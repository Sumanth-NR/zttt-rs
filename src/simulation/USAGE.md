# Simulation Module - Usage Guide

This guide shows how to use the simulation module for high-performance batch game simulations.

## Basic Usage

The simplest way to run a simulation:

```rust
use zttt_rs::simulation::{Simulator, SimulationConfig};
use zttt_rs::backend::{FastEngine, Player};

let config = SimulationConfig::builder()
    .num_games(10_000)
    .engine(FastEngine)
    .starting_player(Player::X)
    .build();

let result = Simulator::new(config).run_sequential();

println!("X win rate: {:.2}%", result.win_rate(Player::X));
println!("Throughput: {} games/sec", result.throughput() as u64);
```

## Configuration Options

The simulation module uses a builder pattern for configuration:

```rust
SimulationConfig::builder()
    .num_games(100_000)           // Required: number of games to simulate
    .engine(FastEngine)            // Required: engine implementing Engine trait
    .starting_player(Player::X)    // Required: which player starts
    .seed(42)                      // Optional: for future reproducibility
    .build()
```

## Working with Results

The `SimulationResult` struct provides comprehensive statistics:

```rust
let result = Simulator::new(config).run_sequential();

// Game outcomes
let x_wins = result.x_wins();
let o_wins = result.o_wins();
let draws = result.draws();
let total = result.games_completed();

// Win rates (as percentages)
let x_rate = result.win_rate(Player::X);
let o_rate = result.win_rate(Player::O);
let draw_rate = result.draw_rate();

// Performance metrics
let total_time = result.total_duration();
let avg_time = result.avg_game_duration();
let throughput = result.throughput(); // games per second
```

## Progress Tracking with Callbacks

For long-running simulations, you can track progress with callbacks:

```rust
let config = SimulationConfig::builder()
    .num_games(100_000)
    .engine(FastEngine)
    .starting_player(Player::X)
    .build();

let mut game_count = 0;
let result = Simulator::new(config).run_with_callback(|game_result| {
    game_count += 1;
    
    if game_count % 10_000 == 0 {
        println!("Completed {} games", game_count);
    }
});
```

Callbacks receive the `GameResult` for each game, allowing you to:
- Track progress
- Stream results to files or databases
- Implement custom statistics collection
- React to specific game outcomes

## Using Custom Engines

The simulation module works with any engine implementing the `Engine` trait:

```rust
use zttt_rs::backend::{Board, Player, Engine, GameResult};

#[derive(Clone, Copy)]
struct MyEngine;

impl Engine for MyEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        // Your move selection logic here
        board.valid_moves().into_iter().next()
    }
}

// Use it in simulations
let config = SimulationConfig::builder()
    .num_games(1_000)
    .engine(MyEngine)
    .starting_player(Player::X)
    .build();

let result = Simulator::new(config).run_sequential();
```

## Performance Tips

1. **Use FastEngine for maximum throughput**: ~1.6M+ games/sec
2. **Callback overhead is minimal**: Simple callbacks add < 1% overhead
3. **Choose starting player strategically**: May affect statistics depending on engine
4. **Compile with --release**: Debug builds are significantly slower

## Examples

See the `examples/` directory for complete examples:

- `simulation_module.rs` - Basic usage and performance testing
- `simulation_with_callback.rs` - Progress tracking with callbacks
- `perfect_simulation.rs` - Using a custom engine (minimax with alpha-beta pruning)

## What's Next?

Phase 1 (Core Sequential Simulator) is complete. Future phases will add:

- **Phase 2**: Parallel execution with multi-threading
- **Phase 3**: Advanced statistics collection
- **Phase 4**: Tournament system, result streaming, and more

See `src/simulation/README.md` for the complete roadmap.
