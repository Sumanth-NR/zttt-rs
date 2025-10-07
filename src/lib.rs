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

use std::fmt;

/// Represents a player in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// Returns the opponent of this player
    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

/// Represents a cell on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

/// Represents the result of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win(Player),
    Draw,
    InProgress,
}

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
                new_board.cells[row][col] = Cell::Occupied(current_player);
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
                new_board.cells[row][col] = Cell::Occupied(current_player);
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
            new_board.cells[row][col] = Cell::Occupied(player);
            let score = self.minimax(&new_board, player, player.opponent(), i32::MIN, i32::MAX, false);
            
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        Some(best_move)
    }
}

/// The TicTacToe board
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    /// Gets the cell at the given position
    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row < 3 && col < 3 {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    /// Makes a move on the board
    pub fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), &'static str> {
        if row >= 3 || col >= 3 {
            return Err("Position out of bounds");
        }

        if self.cells[row][col] != Cell::Empty {
            return Err("Cell already occupied");
        }

        if self.game_result() != GameResult::InProgress {
            return Err("Game is already over");
        }

        self.cells[row][col] = Cell::Occupied(player);
        Ok(())
    }

    /// Checks if a move is valid
    pub fn is_valid_move(&self, row: usize, col: usize) -> bool {
        row < 3 && col < 3 && self.cells[row][col] == Cell::Empty && self.game_result() == GameResult::InProgress
    }

    /// Gets all valid moves
    pub fn valid_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        if self.game_result() != GameResult::InProgress {
            return moves;
        }
        
        for row in 0..3 {
            for col in 0..3 {
                if self.cells[row][col] == Cell::Empty {
                    moves.push((row, col));
                }
            }
        }
        moves
    }

    /// Checks the current game result
    pub fn game_result(&self) -> GameResult {
        // Check rows
        for row in 0..3 {
            if let Cell::Occupied(player) = self.cells[row][0] {
                if self.cells[row][1] == Cell::Occupied(player) 
                    && self.cells[row][2] == Cell::Occupied(player) {
                    return GameResult::Win(player);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if let Cell::Occupied(player) = self.cells[0][col] {
                if self.cells[1][col] == Cell::Occupied(player) 
                    && self.cells[2][col] == Cell::Occupied(player) {
                    return GameResult::Win(player);
                }
            }
        }

        // Check diagonals
        if let Cell::Occupied(player) = self.cells[0][0] {
            if self.cells[1][1] == Cell::Occupied(player) 
                && self.cells[2][2] == Cell::Occupied(player) {
                return GameResult::Win(player);
            }
        }

        if let Cell::Occupied(player) = self.cells[0][2] {
            if self.cells[1][1] == Cell::Occupied(player) 
                && self.cells[2][0] == Cell::Occupied(player) {
                return GameResult::Win(player);
            }
        }

        // Check for draw
        let has_empty = self.cells.iter()
            .flat_map(|row| row.iter())
            .any(|&cell| cell == Cell::Empty);

        if has_empty {
            GameResult::InProgress
        } else {
            GameResult::Draw
        }
    }

    /// Convenience method to find the best move using an engine
    ///
    /// This is a helper method that accepts any engine implementing the `Engine` trait.
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::{Board, Player, PerfectEngine};
    ///
    /// let board = Board::new();
    /// let engine = PerfectEngine::new();
    /// let best_move = board.choose_move(&engine, Player::X);
    /// ```
    pub fn choose_move(&self, engine: &impl Engine, player: Player) -> Option<(usize, usize)> {
        engine.choose_move(self, player)
    }

    /// Resets the board to empty state
    pub fn reset(&mut self) {
        self.cells = [[Cell::Empty; 3]; 3];
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Occupied(player) => write!(f, "{}", player)?,
                }
                if j < 2 {
                    write!(f, " ")?;
                }
            }
            if i < 2 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

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
}
