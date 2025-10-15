//! # Backend Module
//!
//! Core game logic and engine implementations for TicTacToe.
//!
//! This module contains the fundamental building blocks:
//! - **Board**: Game state representation and game logic
//! - **Player & Cell**: Basic game types
//! - **GameResult**: Game outcome representation
//! - **Engine**: Trait for move selection strategies
//! - **FastEngine**: High-speed move selection implementation
//!
//! The backend is optimized for maximum performance and minimal memory overhead,
//! making it ideal for high-throughput game simulations.

pub mod board;
pub mod player;
pub mod game;
pub mod engine;

// Re-export public API for convenience
pub use board::Board;
pub use player::{Player, Cell};
pub use game::GameResult;
pub use engine::{Engine, FastEngine};
