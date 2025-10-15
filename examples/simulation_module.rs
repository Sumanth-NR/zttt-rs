use zttt_rs::simulation::{Simulator, SimulationConfig};
use zttt_rs::backend::{FastEngine, Player};

fn main() {
    println!("=== Simulation Module Example ===\n");
    
    // Small simulation
    println!("Running 10,000 games...");
    let config = SimulationConfig::builder()
        .num_games(10_000)
        .engine(FastEngine)
        .starting_player(Player::X)
        .build();
    
    let result = Simulator::new(config).run_sequential();
    
    println!("Completed: {} games", result.games_completed());
    println!("X wins: {} ({:.2}%)", result.x_wins(), result.win_rate(Player::X));
    println!("O wins: {} ({:.2}%)", result.o_wins(), result.win_rate(Player::O));
    println!("Draws: {} ({:.2}%)", result.draws(), result.draw_rate());
    println!("Duration: {:?}", result.total_duration());
    println!("Avg per game: {:?}", result.avg_game_duration());
    println!("Throughput: {} games/sec\n", result.throughput() as u64);
    
    // Large simulation for performance testing
    println!("Running 100,000 games for performance benchmark...");
    let config = SimulationConfig::builder()
        .num_games(100_000)
        .engine(FastEngine)
        .starting_player(Player::X)
        .build();
    
    let result = Simulator::new(config).run_sequential();
    
    println!("Completed: {} games", result.games_completed());
    println!("Duration: {:?}", result.total_duration());
    println!("Avg per game: {:.2}µs", result.avg_game_duration().as_micros() as f64);
    println!("Throughput: {} games/sec", result.throughput() as u64);
    
    // Performance validation
    let throughput = result.throughput();
    if throughput > 1_500_000.0 {
        println!("\n✓ Performance target met! (> 1.5M games/sec)");
    } else {
        println!("\n⚠ Performance below target (< 1.5M games/sec)");
    }
}
