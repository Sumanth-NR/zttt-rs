//! # zttt-rs
//!
//! A high-performance Rust backend for TicTacToe games with pluggable engines.
//!
//! This crate provides:
//! - Efficient game state representation
//! - Move validation and game logic
//! - Pluggable engine trait for custom move selection logic
//! - Built-in perfect engine using minimax with alpha-beta pruning
//! - Fast simulations for research and analysis
//! - `ValidatedBoard` for performance-critical operations without repeated bounds checking
//!
//! ## Board vs ValidatedBoard
//!
//! The crate provides two board types:
//!
//! - **`Board`**: The standard board with full validation on all operations. Use this for
//!   normal game play where safety is paramount.
//! - **`ValidatedBoard`**: A performance-optimized board that provides unchecked operations
//!   for use in hot paths like minimax search. Use this when you can guarantee that all
//!   coordinates are valid (0..3).
//!
//! ## Example
//!
//! ```
//! use zttt_rs::{Board, Player, GameResult, PerfectEngine, Engine};
//!
//! let mut board = Board::new();
//! board.make_move(0, 0, Player::X).unwrap();
//! board.make_move(1, 1, Player::O).unwrap();
//! 
//! let engine = PerfectEngine::new();
//! let best_move = engine.choose_move(&board, Player::X);
//! println!("Best move: {:?}", best_move);
//! ```
//!
//! ## Performance Example with ValidatedBoard
//!
//! ```
//! use zttt_rs::{ValidatedBoard, Player, Cell};
//!
//! let mut board = ValidatedBoard::default();
//! // When you know coordinates are valid (0..3), use unchecked operations
//! unsafe {
//!     board.make_move_unchecked(0, 0, Player::X);
//!     board.make_move_unchecked(1, 1, Player::O);
//! }
//! println!("Board: {}", board);
//! ```

mod player;
mod game;
mod board;
mod engine;

// Re-export public API
pub use player::{Player, Cell};
pub use game::GameResult;
pub use board::{Board, ValidatedBoard};
pub use engine::{Engine, PerfectEngine};

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
    fn test_engine_blocks_win() {
        let mut board = Board::new();
        // O has two in a row, X should block
        board.make_move(0, 0, Player::O).unwrap();
        board.make_move(1, 1, Player::X).unwrap();
        board.make_move(0, 1, Player::O).unwrap();
        
        let engine = PerfectEngine::new();
        let best = engine.choose_move(&board, Player::X);
        assert_eq!(best, Some((0, 2))); // Block the win
    }

    #[test]
    fn test_engine_takes_win() {
        let mut board = Board::new();
        // X has two in a row, should take the win
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 0, Player::O).unwrap();
        board.make_move(0, 1, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();
        
        let engine = PerfectEngine::new();
        let best = engine.choose_move(&board, Player::X);
        assert_eq!(best, Some((0, 2))); // Take the win
    }

    #[test]
    fn test_engine_center() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        
        let engine = PerfectEngine::new();
        let best = engine.choose_move(&board, Player::O);
        // Center is typically the best response
        assert_eq!(best, Some((1, 1)));
    }

    #[test]
    fn test_board_choose_move_convenience() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        
        let engine = PerfectEngine::new();
        let best = board.choose_move(&engine, Player::O);
        // Center is typically the best response
        assert_eq!(best, Some((1, 1)));
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

    #[test]
    fn test_validated_board_from_board() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        board.make_move(1, 1, Player::O).unwrap();

        let validated = ValidatedBoard::from_board(board.clone());
        assert_eq!(validated.get(0, 0), Some(Cell::Occupied(Player::X)));
        assert_eq!(validated.get(1, 1), Some(Cell::Occupied(Player::O)));
        assert_eq!(validated.get(2, 2), Some(Cell::Empty));
    }

    #[test]
    fn test_validated_board_to_board() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();

        let validated = ValidatedBoard::from_board(board.clone());
        let back_to_board = validated.to_board();
        assert_eq!(back_to_board, board);
    }

    #[test]
    fn test_validated_board_set() {
        let mut validated = ValidatedBoard::default();
        assert!(validated.set(0, 0, Cell::Occupied(Player::X)).is_ok());
        assert_eq!(validated.get(0, 0), Some(Cell::Occupied(Player::X)));
    }

    #[test]
    fn test_validated_board_set_out_of_bounds() {
        let mut validated = ValidatedBoard::default();
        assert!(validated.set(3, 3, Cell::Occupied(Player::X)).is_err());
    }

    #[test]
    fn test_validated_board_get_out_of_bounds() {
        let validated = ValidatedBoard::default();
        assert_eq!(validated.get(3, 3), None);
    }

    #[test]
    fn test_validated_board_unchecked() {
        let mut validated = ValidatedBoard::default();
        unsafe {
            validated.make_move_unchecked(0, 0, Player::X);
            assert_eq!(validated.get_unchecked(0, 0), Cell::Occupied(Player::X));
        }
    }

    #[test]
    fn test_validated_board_game_result() {
        let mut validated = ValidatedBoard::default();
        unsafe {
            validated.make_move_unchecked(0, 0, Player::X);
            validated.make_move_unchecked(0, 1, Player::X);
            validated.make_move_unchecked(0, 2, Player::X);
        }
        assert_eq!(validated.game_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn test_validated_board_valid_moves() {
        let mut validated = ValidatedBoard::default();
        assert_eq!(validated.valid_moves().len(), 9);
        
        unsafe {
            validated.make_move_unchecked(0, 0, Player::X);
        }
        assert_eq!(validated.valid_moves().len(), 8);
    }

    #[test]
    fn test_validated_board_display() {
        let mut validated = ValidatedBoard::default();
        unsafe {
            validated.make_move_unchecked(0, 0, Player::X);
            validated.make_move_unchecked(1, 1, Player::O);
        }
        let display = format!("{}", validated);
        assert!(display.contains("X"));
        assert!(display.contains("O"));
        assert!(display.contains("."));
    }

    #[test]
    fn test_validated_board_as_board() {
        let mut board = Board::new();
        board.make_move(0, 0, Player::X).unwrap();
        
        let validated = ValidatedBoard::from_board(board.clone());
        let board_ref = validated.as_board();
        assert_eq!(board_ref.get(0, 0), Some(Cell::Occupied(Player::X)));
    }
}
