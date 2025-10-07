//! Game result type

use crate::player::Player;

/// Represents the result of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win(Player),
    Draw,
    InProgress,
}
