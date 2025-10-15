use zttt_rs::simulation::{Simulator, SimulationConfig};
use zttt_rs::backend::{FastEngine, Player, GameResult};

fn main() {
    println!("=== Simulation with Callback Example ===\n");
    
    let num_games = 50_000;
    let progress_interval = 10_000;
    
    println!("Running {} games with progress tracking...\n", num_games);
    
    let config = SimulationConfig::builder()
        .num_games(num_games)
        .engine(FastEngine)
        .starting_player(Player::X)
        .build();
    
    let mut game_count = 0;
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut draws = 0;
    
    let result = Simulator::new(config).run_with_callback(|game_result| {
        game_count += 1;
        
        match game_result {
            GameResult::Win(Player::X) => x_wins += 1,
            GameResult::Win(Player::O) => o_wins += 1,
            GameResult::Draw => draws += 1,
            GameResult::InProgress => {}
        }
        
        // Print progress at intervals
        if game_count % progress_interval == 0 {
            println!("Progress: {}/{} games", game_count, num_games);
            println!("  Current stats - X: {}, O: {}, Draws: {}", x_wins, o_wins, draws);
        }
    });
    
    println!("\n=== Final Results ===");
    println!("Completed: {} games", result.games_completed());
    println!("X wins: {} ({:.2}%)", result.x_wins(), result.win_rate(Player::X));
    println!("O wins: {} ({:.2}%)", result.o_wins(), result.win_rate(Player::O));
    println!("Draws: {} ({:.2}%)", result.draws(), result.draw_rate());
    println!("Throughput: {} games/sec", result.throughput() as u64);
}
