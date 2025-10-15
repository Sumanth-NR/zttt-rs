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

fn main() {
    println!("=== Running Multiple TicTacToe Simulations ===\n");
    
    let num_games = 10_000;
    
    let start = Instant::now();
    
    let mut wins_x = 0;
    let mut wins_o = 0;
    let mut draws = 0;
    
    for _ in 0..num_games {
        match simulate_game() {
            GameResult::Win(Player::X) => wins_x += 1,
            GameResult::Win(Player::O) => wins_o += 1,
            GameResult::Draw => draws += 1,
            _ => {}
        }
    }
    
    let duration = start.elapsed();
    
    println!("Completed {} games in {:?}", num_games, duration);
    println!("Average time per game: {:.2}µs", duration.as_micros() as f64 / num_games as f64);
    println!("\nResults:");
    println!("X wins: {} ({:.2}%)", wins_x, (wins_x as f64 / num_games as f64) * 100.0);
    println!("O wins: {} ({:.2}%)", wins_o, (wins_o as f64 / num_games as f64) * 100.0);
    println!("Draws:  {} ({:.2}%)", draws, (draws as f64 / num_games as f64) * 100.0);
    
    println!("\nNote: When both players play optimally, the result is always a draw!");
}
