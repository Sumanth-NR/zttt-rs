//! # zttt-rs
//!
//! The fastest and most optimized Rust backend for simulating TicTacToe games.
//!
//! ## Architecture
//!
//! This crate is organized into two main modules:
//!
//! ### Backend Module
//! Core game logic and engine implementations optimized for maximum performance:
//! - **Blazing fast game simulations**: Optimized for high-throughput scenarios
//! - **Efficient game state representation**: Minimal memory footprint for large-scale simulations
//! - **Move validation and game logic**: Fast and reliable core game mechanics
//! - **Pluggable engine trait**: Implement custom move selection logic for different use cases
//! - **High-speed engine**: FastEngine for maximum throughput in simulations
//!
//! ### Simulation Module (Planned)
//! High-performance simulation framework for batch processing:
//! - **Sequential and parallel simulation runners**: Scale from single to multi-core
//! - **Configurable simulation scenarios**: Flexible setup for various use cases
//! - **Statistics and analysis**: Comprehensive data collection and insights
//! - **Tournament system**: Engine matchups and comparisons
//! - **Memory optimization**: Efficient handling of millions of games
//!
//! See [`simulation`] module documentation for detailed planning and roadmap.
//!
//! ## Example
//!
//! ```
//! use zttt_rs::{Board, Player, GameResult, FastEngine, Engine};
//!
//! let mut board = Board::new();
//! board.make_move(0, 0, Player::X).unwrap();
//! board.make_move(1, 1, Player::O).unwrap();
//! 
//! let engine = FastEngine;
//! let next_move = engine.choose_move(&board, Player::X);
//! println!("Next move: {:?}", next_move);
//! ```

// Core backend module - game logic and engine implementations
pub mod backend;

// Simulation module - high-performance batch simulation framework
pub mod simulation;

// Re-export public API from backend for convenience
pub use backend::{Board, Player, Cell, GameResult, Engine, FastEngine};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert_eq!(board.game_result(), GameResult::InProgress);
        assert_eq!(board.valid_moves().len(), 9);
    }

    #[test]
    fn test_make_move() {
        let mut board = Board::new();
        assert!(board.make_move(0, 0, Player::X).is_ok());
        assert_eq!(board.get(0, 0), Some(Cell::Occupied(Player::X)));
        assert_eq!(board.valid_moves().len(), 8);
    }

    #[test]
    fn test_invalid_move() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        assert!(board.make_move(0, 0, Player::O).is_err());
    }

    #[test]
    fn test_out_of_bounds() {
        let mut board = Board::new();
        assert!(board.make_move(3, 3, Player::X).is_err());
    }

    #[test]
    fn test_win_row() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 0, Player::O).unwrap();
        board.make_move(0, 1, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        board.make_move(0, 2, Player::X).unwrap();
        assert_eq!(board.game_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn test_win_column() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(0, 1, Player::O).unwrap();
        board.make_move(1, 0, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        board.make_move(2, 0, Player::X).unwrap();
        assert_eq!(board.game_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn test_win_diagonal() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(0, 1, Player::O).unwrap();
        board.make_move(1, 1, Player::X).unwrap();
        board.make_move(0, 2, Player::O).unwrap();
        board.make_move(2, 2, Player::X).unwrap();
        assert_eq!(board.game_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn test_win_anti_diagonal() {
        let mut board = Board::new();
        board.make_move(0, 2, Player::X).unwrap();
        board.make_move(0, 1, Player::O).unwrap();
        board.make_move(1, 1, Player::X).unwrap();
        board.make_move(0, 0, Player::O).unwrap();
        board.make_move(2, 0, Player::X).unwrap();
        assert_eq!(board.game_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn test_draw() {
        let mut board = Board::new();
        // X O X
        // X O O
        // O X X
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(0, 1, Player::O).unwrap();
        board.make_move(0, 2, Player::X).unwrap();
        board.make_move(1, 0, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        board.make_move(1, 2, Player::O).unwrap();
        board.make_move(2, 0, Player::O).unwrap();
        board.make_move(2, 1, Player::X).unwrap();
        board.make_move(2, 2, Player::X).unwrap();
        assert_eq!(board.game_result(), GameResult::Draw);
    }

    #[test]
    fn test_engine_returns_valid_move() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::O).unwrap();
        board.make_move(1, 1, Player::X).unwrap();
        
        let engine = FastEngine;
        let chosen = engine.choose_move(&board, Player::X);
        assert!(chosen.is_some());
        let (row, col) = chosen.unwrap();
        assert!(board.is_valid_move(row, col));
    }

    #[test]
    fn test_engine_returns_none_when_game_over() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 0, Player::O).unwrap();
        board.make_move(0, 1, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        board.make_move(0, 2, Player::X).unwrap();
        
        assert_eq!(board.game_result(), GameResult::Win(Player::X));
        
        let engine = FastEngine;
        let chosen = engine.choose_move(&board, Player::O);
        assert!(chosen.is_none());
    }

    #[test]
    fn test_board_choose_move_convenience() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        
        let engine = FastEngine;
        let chosen = board.choose_move(&engine, Player::O);
        assert!(chosen.is_some());
    }

    #[test]
    fn test_player_opponent() {
        assert_eq!(Player::X.opponent(), Player::O);
        assert_eq!(Player::O.opponent(), Player::X);
    }

    #[test]
    fn test_reset_board() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        board.reset();
        assert_eq!(board.game_result(), GameResult::InProgress);
        assert_eq!(board.valid_moves().len(), 9);
    }

    #[test]
    fn test_display() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        let display = format!("{}", board);
        assert!(display.contains("X"));
        assert!(display.contains("O"));
        assert!(display.contains("."));
    }

    #[test]
    fn test_cannot_move_after_game_over() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 0, Player::O).unwrap();
        board.make_move(0, 1, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        board.make_move(0, 2, Player::X).unwrap();
        
        assert_eq!(board.game_result(), GameResult::Win(Player::X));
        assert!(board.make_move(2, 2, Player::O).is_err());
    }
}
