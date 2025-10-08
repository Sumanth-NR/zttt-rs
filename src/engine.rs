//! Engine trait and implementations for move selection

use crate::board::Board;
use crate::game::GameResult;
use crate::player::{Cell, Player};

/// Trait for implementing custom game engines
///
/// This trait allows you to implement different strategies for selecting moves.
/// You can create engines with different algorithms, difficulty levels, or
/// heuristics to suit your needs.
pub trait Engine {
    /// Choose the best move for the given player on the given board
    ///
    /// Returns `None` if no valid moves are available or the game is over.
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)>;
}

/// A perfect play engine using minimax algorithm with alpha-beta pruning
///
/// This engine guarantees optimal play and will never lose when playing first.
/// When both players use this engine, the game always results in a draw.
#[derive(Debug, Clone, Copy)]
pub struct PerfectEngine;

impl PerfectEngine {
    /// Creates a new perfect engine
    pub fn new() -> Self {
        PerfectEngine
    }

    /// Minimax algorithm with alpha-beta pruning
    fn minimax(
        &self,
        board: &Board,
        maximizing_player: Player,
        current_player: Player,
        mut alpha: i32,
        mut beta: i32,
        is_maximizing: bool,
    ) -> i32 {
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
                new_board.cells[row][col] = Cell::Occupied(current_player);
                let eval = self.minimax(
                    &new_board,
                    maximizing_player,
                    current_player.opponent(),
                    alpha,
                    beta,
                    false,
                );
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
                new_board.cells[row][col] = Cell::Occupied(current_player);
                let eval = self.minimax(
                    &new_board,
                    maximizing_player,
                    current_player.opponent(),
                    alpha,
                    beta,
                    true,
                );
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
            new_board.cells[row][col] = Cell::Occupied(player);
            let score =
                self.minimax(&new_board, player, player.opponent(), i32::MIN, i32::MAX, false);

            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        Some(best_move)
    }
}
