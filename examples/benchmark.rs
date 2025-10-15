use zttt_rs::backend::{Board, Player, GameResult, Engine};
use std::time::Instant;

/// A perfect play engine using minimax algorithm with alpha-beta pruning
/// (Included here for benchmarking purposes)
#[derive(Debug, Clone, Copy)]
struct PerfectEngine;

impl PerfectEngine {
    fn new() -> Self {
        PerfectEngine
    }

    fn minimax(&self, board: &Board, maximizing_player: Player, current_player: Player, mut alpha: i32, mut beta: i32, is_maximizing: bool) -> i32 {
        match board.game_result() {
            GameResult::Win(player) => {
                if player == maximizing_player {
                    return 10;
                } else {
                    return -10;
                }
            }
            GameResult::Draw => return 0,
            GameResult::InProgress => {}
        }

        if is_maximizing {
            let mut max_eval = i32::MIN;
            for &(row, col) in &board.valid_moves() {
                let mut new_board = board.clone();
                new_board.make_move(row, col, current_player).unwrap();
                let eval = self.minimax(&new_board, maximizing_player, current_player.opponent(), alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &(row, col) in &board.valid_moves() {
                let mut new_board = board.clone();
                new_board.make_move(row, col, current_player).unwrap();
                let eval = self.minimax(&new_board, maximizing_player, current_player.opponent(), alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }
}

impl Engine for PerfectEngine {
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)> {
        if board.game_result() != GameResult::InProgress {
            return None;
        }

        let moves = board.valid_moves();
        if moves.is_empty() {
            return None;
        }

        let mut best_score = i32::MIN;
        let mut best_move = moves[0];

        for &(row, col) in &moves {
            let mut new_board = board.clone();
            new_board.make_move(row, col, player).unwrap();
            let score = self.minimax(&new_board, player, player.opponent(), i32::MIN, i32::MAX, false);
            
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        Some(best_move)
    }
}

fn simulate_game() -> GameResult {
    let mut board = Board::new();
    let mut current_player = Player::X;
    let engine = PerfectEngine::new();
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
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
    
    // Benchmark engine move calculation at various positions
    println!("\nBenchmarking engine move calculation...");
    
    let engine = PerfectEngine::new();
    
    // Empty board (most expensive)
    let board = Board::new();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = engine.choose_move(&board, Player::X);
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
        let _ = engine.choose_move(&board, Player::O);
    }
    let duration = start.elapsed();
    println!("Mid-game position (1000 iterations): {}ms", duration.as_millis());
    println!("Average: {:.2}µs per calculation", duration.as_micros() as f64 / 1000.0);
    
    println!("\n=== Benchmark Complete ===");
}
