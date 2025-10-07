use zttt_rs::{Board, Player, GameResult};
use std::time::Instant;

fn simulate_game() -> GameResult {
    let mut board = Board::new();
    let mut current_player = Player::X;
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = board.best_move(current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        }
    }
    
    board.game_result()
}

fn benchmark_simulations(num_games: usize) -> std::time::Duration {
    let start = Instant::now();
    
    for _ in 0..num_games {
        simulate_game();
    }
    
    start.elapsed()
}

fn main() {
    println!("=== zttt-rs Performance Benchmark ===\n");
    
    // Warm up
    println!("Warming up...");
    for _ in 0..100 {
        simulate_game();
    }
    
    // Benchmark different batch sizes
    let test_sizes = vec![100, 1_000, 10_000];
    
    for &size in &test_sizes {
        println!("\nBenchmarking {} games...", size);
        let duration = benchmark_simulations(size);
        let total_ms = duration.as_millis();
        let avg_us = duration.as_micros() as f64 / size as f64;
        let games_per_sec = (size as f64 / duration.as_secs_f64()) as u64;
        
        println!("Total time: {}ms", total_ms);
        println!("Average per game: {:.2}µs", avg_us);
        println!("Throughput: {} games/second", games_per_sec);
    }
    
    println!("\n=== Additional Benchmarks ===\n");
    
    // Benchmark move generation
    println!("Benchmarking move generation...");
    let board = Board::new();
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let _ = board.valid_moves();
    }
    let duration = start.elapsed();
    println!("1M move generations: {}ms", duration.as_millis());
    println!("Average: {:.2}ns per generation", duration.as_nanos() as f64 / 1_000_000.0);
    
    // Benchmark best move calculation at various positions
    println!("\nBenchmarking best move calculation...");
    
    // Empty board (most expensive)
    let board = Board::new();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = board.best_move(Player::X);
    }
    let duration = start.elapsed();
    println!("Empty board (100 iterations): {}ms", duration.as_millis());
    println!("Average: {:.2}ms per calculation", duration.as_millis() as f64 / 100.0);
    
    // Mid-game position
    let mut board = Board::new();
    board.make_move(0, 0, Player::X).unwrap();
    board.make_move(1, 1, Player::O).unwrap();
    board.make_move(0, 1, Player::X).unwrap();
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = board.best_move(Player::O);
    }
    let duration = start.elapsed();
    println!("Mid-game position (1000 iterations): {}ms", duration.as_millis());
    println!("Average: {:.2}µs per calculation", duration.as_micros() as f64 / 1000.0);
    
    println!("\n=== Benchmark Complete ===");
}
