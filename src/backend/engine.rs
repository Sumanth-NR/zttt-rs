//! Engine trait and implementations for move selection
//!
//! This module provides abstractions for move selection strategies optimized
//! for high-speed game simulations.

use crate::backend::board::Board;
use crate::backend::player::Player;

/// Trait for implementing custom game engines
///
/// This trait allows you to implement different strategies for selecting moves.
/// The focus is on speed and efficiency for high-throughput simulations.
///
/// # Example
///
/// ```
/// use zttt_rs::{Board, Player, Engine, FastEngine};
///
/// let board = Board::new();
/// let engine = FastEngine;
/// let next_move = engine.choose_move(&board, Player::X);
/// ```
pub trait Engine {
    /// Choose a move for the given player on the given board
    ///
    /// Returns `None` if no valid moves are available or the game is over.
    fn choose_move(&self, board: &Board, player: Player) -> Option<(usize, usize)>;
}

/// A fast engine optimized for high-speed simulations
///
/// This engine simply selects the first available valid move, making it
/// extremely fast for batch simulations where move quality is less important
/// than throughput.
///
/// # Performance
///
/// This engine adds minimal overhead (~nanoseconds) to move selection,
/// making it ideal for large-scale game simulations and benchmarking.
///
/// # Example
///
/// ```
/// use zttt_rs::{Board, Player, Engine, FastEngine};
///
/// let board = Board::new();
/// let engine = FastEngine;
/// if let Some((row, col)) = engine.choose_move(&board, Player::X) {
///     println!("Selected move: ({}, {})", row, col);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FastEngine;

impl Engine for FastEngine {
    fn choose_move(&self, board: &Board, _player: Player) -> Option<(usize, usize)> {
        board.valid_moves().into_iter().next()
    }
}
