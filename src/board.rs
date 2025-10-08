//! Board representation and game logic

use std::fmt;
use crate::player::{Player, Cell};
use crate::game::GameResult;
use crate::engine::Engine;

/// The TicTacToe board
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub(crate) cells: [[Cell; 3]; 3],
}

/// A validated board that provides unchecked access for performance-critical operations
///
/// This wrapper ensures that all positions have been validated before use,
/// allowing for faster operations in hot paths like minimax algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBoard {
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

impl ValidatedBoard {
    /// Creates a new validated board from a regular board
    ///
    /// This conversion validates that the board is in a consistent state.
    pub fn from_board(board: Board) -> Self {
        ValidatedBoard {
            cells: board.cells,
        }
    }

    /// Converts back to a regular Board
    pub fn to_board(self) -> Board {
        Board {
            cells: self.cells,
        }
    }

    /// Gets a reference to the underlying board for read-only operations
    pub fn as_board(&self) -> &Board {
        // SAFETY: ValidatedBoard has the same layout as Board
        unsafe { &*(self as *const ValidatedBoard as *const Board) }
    }

    /// Gets the cell at the given position without bounds checking
    ///
    /// # Safety
    /// The caller must ensure that row < 3 and col < 3.
    /// This method is marked unsafe to indicate that it bypasses bounds checking.
    #[inline]
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> Cell {
        *self.cells.get_unchecked(row).get_unchecked(col)
    }

    /// Gets the cell at the given position with bounds checking
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row < 3 && col < 3 {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    /// Sets the cell at the given position without bounds or validity checking
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - row < 3 and col < 3
    /// - This move maintains a valid game state
    #[inline]
    pub unsafe fn set_unchecked(&mut self, row: usize, col: usize, cell: Cell) {
        *self.cells.get_unchecked_mut(row).get_unchecked_mut(col) = cell;
    }

    /// Sets the cell at the given position with bounds checking
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, cell: Cell) -> Result<(), &'static str> {
        if row >= 3 || col >= 3 {
            return Err("Position out of bounds");
        }
        self.cells[row][col] = cell;
        Ok(())
    }

    /// Makes a move on the board without validation
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - row < 3 and col < 3
    /// - The cell is empty
    /// - The game is in progress
    #[inline]
    pub unsafe fn make_move_unchecked(&mut self, row: usize, col: usize, player: Player) {
        self.set_unchecked(row, col, Cell::Occupied(player));
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
}

impl Default for ValidatedBoard {
    fn default() -> Self {
        ValidatedBoard {
            cells: [[Cell::Empty; 3]; 3],
        }
    }
}

impl fmt::Display for ValidatedBoard {
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
