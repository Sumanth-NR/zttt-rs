//! Perfect play engine using minimax with alpha-beta pruning
//!
//! This engine is kept as a benchmark reference for optimal play.
//! It uses the minimax algorithm with alpha-beta pruning to guarantee
//! perfect play but is slower than simple engines.

use zttt_rs::{Board, Player, Engine, GameResult};

/// A perfect play engine using minimax algorithm with alpha-beta pruning
///
/// This engine guarantees optimal play and will never lose when playing first.
/// When both players use this engine, the game always results in a draw.
///
/// **Note**: This engine is provided as a benchmark reference. For high-speed
/// simulations, consider using simpler engines like `FastEngine`.
#[derive(Debug, Clone, Copy)]
pub struct PerfectEngine;

impl PerfectEngine {
    /// Creates a new perfect engine
    pub fn new() -> Self {
        PerfectEngine
    }

    /// Minimax algorithm with alpha-beta pruning
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
                    break; // Beta cutoff
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
                    break; // Alpha cutoff
                }
            }
            min_eval
        }
    }
}

impl Default for PerfectEngine {
    fn default() -> Self {
        Self::new()
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

fn main() {
    println!("PerfectEngine is available as a module for benchmarking.");
    println!("Use this engine to compare performance against simpler engines.");
}
