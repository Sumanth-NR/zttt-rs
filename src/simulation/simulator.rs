//! Core simulation runner

use std::time::Instant;
use crate::backend::{Board, Engine, GameResult, Player};
use crate::simulation::{SimulationConfig, SimulationResult};

/// High-performance sequential game simulator
///
/// Runs batch simulations of TicTacToe games using a configured engine.
/// Optimized for maximum throughput with minimal overhead.
///
/// # Example
///
/// ```
/// use zttt_rs::simulation::{Simulator, SimulationConfig};
/// use zttt_rs::backend::{FastEngine, Player};
///
/// let config = SimulationConfig::builder()
///     .num_games(10_000)
///     .engine(FastEngine)
///     .starting_player(Player::X)
///     .build();
///
/// let result = Simulator::new(config).run_sequential();
/// println!("Completed {} games", result.games_completed());
/// println!("Throughput: {} games/sec", result.throughput() as u64);
/// ```
pub struct Simulator<E> {
    config: SimulationConfig<E>,
}

impl<E: Engine> Simulator<E> {
    /// Create a new simulator with the given configuration
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::{Simulator, SimulationConfig};
    /// use zttt_rs::backend::{FastEngine, Player};
    ///
    /// let config = SimulationConfig::builder()
    ///     .num_games(1000)
    ///     .engine(FastEngine)
    ///     .starting_player(Player::X)
    ///     .build();
    ///
    /// let simulator = Simulator::new(config);
    /// ```
    pub fn new(config: SimulationConfig<E>) -> Self {
        Self { config }
    }

    /// Run the simulation sequentially with a callback
    ///
    /// The callback is invoked after each game with the game result.
    /// This allows for custom processing, progress tracking, or streaming results.
    ///
    /// # Performance
    ///
    /// Using a callback adds minimal overhead when the callback is simple.
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::{Simulator, SimulationConfig};
    /// use zttt_rs::backend::{FastEngine, Player, GameResult};
    ///
    /// let config = SimulationConfig::builder()
    ///     .num_games(1000)
    ///     .engine(FastEngine)
    ///     .starting_player(Player::X)
    ///     .build();
    ///
    /// let simulator = Simulator::new(config);
    /// let mut game_count = 0;
    /// let result = simulator.run_with_callback(|_result| {
    ///     game_count += 1;
    /// });
    ///
    /// assert_eq!(game_count, 1000);
    /// ```
    pub fn run_with_callback<F>(self, mut callback: F) -> SimulationResult
    where
        F: FnMut(GameResult),
    {
        let start = Instant::now();
        
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut draws = 0;
        
        for _ in 0..self.config.num_games {
            let result = self.simulate_single_game();
            
            // Invoke callback
            callback(result);
            
            // Update statistics
            match result {
                GameResult::Win(Player::X) => x_wins += 1,
                GameResult::Win(Player::O) => o_wins += 1,
                GameResult::Draw => draws += 1,
                GameResult::InProgress => {
                    panic!("Game ended in InProgress state");
                }
            }
        }
        
        let total_duration = start.elapsed();
        
        SimulationResult::new(
            self.config.num_games,
            x_wins,
            o_wins,
            draws,
            total_duration,
        )
    }

    /// Run the simulation sequentially on a single thread
    ///
    /// This method runs all configured games sequentially and collects
    /// comprehensive statistics about the results and performance.
    ///
    /// # Performance
    ///
    /// With FastEngine, this achieves approximately 1.8M games/second,
    /// matching the performance of raw simulation loops.
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::{Simulator, SimulationConfig};
    /// use zttt_rs::backend::{FastEngine, Player};
    ///
    /// let config = SimulationConfig::builder()
    ///     .num_games(10_000)
    ///     .engine(FastEngine)
    ///     .starting_player(Player::X)
    ///     .build();
    ///
    /// let simulator = Simulator::new(config);
    /// let result = simulator.run_sequential();
    ///
    /// println!("X wins: {}", result.x_wins());
    /// println!("O wins: {}", result.o_wins());
    /// println!("Draws: {}", result.draws());
    /// println!("Win rate: {:.2}%", result.win_rate(Player::X));
    /// ```
    pub fn run_sequential(self) -> SimulationResult {
        let start = Instant::now();
        
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut draws = 0;
        
        for _ in 0..self.config.num_games {
            let result = self.simulate_single_game();
            match result {
                GameResult::Win(Player::X) => x_wins += 1,
                GameResult::Win(Player::O) => o_wins += 1,
                GameResult::Draw => draws += 1,
                GameResult::InProgress => {
                    // This shouldn't happen, but handle gracefully
                    panic!("Game ended in InProgress state");
                }
            }
        }
        
        let total_duration = start.elapsed();
        
        SimulationResult::new(
            self.config.num_games,
            x_wins,
            o_wins,
            draws,
            total_duration,
        )
    }

    /// Simulate a single game
    ///
    /// This is an internal helper method that runs one complete game
    /// using the configured engine and returns the result.
    fn simulate_single_game(&self) -> GameResult {
        let mut board = Board::new();
        let mut current_player = self.config.starting_player;
        
        while board.game_result() == GameResult::InProgress {
            if let Some((row, col)) = self.config.engine.choose_move(&board, current_player) {
                // We can unwrap here because choose_move should only return valid moves
                board.make_move(row, col, current_player).unwrap();
                current_player = current_player.opponent();
            } else {
                // No valid moves available but game is still in progress
                // This shouldn't happen with a correct engine implementation
                break;
            }
        }
        
        board.game_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::FastEngine;

    #[test]
    fn test_simulator_basic() {
        let config = SimulationConfig::builder()
            .num_games(100)
            .engine(FastEngine)
            .starting_player(Player::X)
            .build();
        
        let simulator = Simulator::new(config);
        let result = simulator.run_sequential();
        
        assert_eq!(result.games_completed(), 100);
        assert_eq!(result.x_wins() + result.o_wins() + result.draws(), 100);
    }

    #[test]
    fn test_simulator_statistics() {
        let config = SimulationConfig::builder()
            .num_games(1000)
            .engine(FastEngine)
            .starting_player(Player::X)
            .build();
        
        let simulator = Simulator::new(config);
        let result = simulator.run_sequential();
        
        assert!(result.throughput() > 0.0);
        assert!(result.total_duration().as_nanos() > 0);
        assert!(result.avg_game_duration().as_nanos() > 0);
    }

    #[test]
    fn test_win_rate_calculation() {
        let config = SimulationConfig::builder()
            .num_games(100)
            .engine(FastEngine)
            .starting_player(Player::X)
            .build();
        
        let simulator = Simulator::new(config);
        let result = simulator.run_sequential();
        
        let x_rate = result.win_rate(Player::X);
        let o_rate = result.win_rate(Player::O);
        let draw_rate = result.draw_rate();
        
        // Rates should sum to approximately 100%
        let total = x_rate + o_rate + draw_rate;
        assert!((total - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_different_starting_players() {
        let config_x = SimulationConfig::builder()
            .num_games(100)
            .engine(FastEngine)
            .starting_player(Player::X)
            .build();
        
        let config_o = SimulationConfig::builder()
            .num_games(100)
            .engine(FastEngine)
            .starting_player(Player::O)
            .build();
        
        let result_x = Simulator::new(config_x).run_sequential();
        let result_o = Simulator::new(config_o).run_sequential();
        
        assert_eq!(result_x.games_completed(), 100);
        assert_eq!(result_o.games_completed(), 100);
    }

    #[test]
    fn test_callback_invocation() {
        let config = SimulationConfig::builder()
            .num_games(50)
            .engine(FastEngine)
            .starting_player(Player::X)
            .build();
        
        let mut callback_count = 0;
        let simulator = Simulator::new(config);
        
        let result = simulator.run_with_callback(|_result| {
            callback_count += 1;
        });
        
        assert_eq!(callback_count, 50);
        assert_eq!(result.games_completed(), 50);
    }

    #[test]
    fn test_callback_receives_results() {
        let config = SimulationConfig::builder()
            .num_games(100)
            .engine(FastEngine)
            .starting_player(Player::X)
            .build();
        
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut draws = 0;
        
        let simulator = Simulator::new(config);
        let result = simulator.run_with_callback(|game_result| {
            match game_result {
                GameResult::Win(Player::X) => x_wins += 1,
                GameResult::Win(Player::O) => o_wins += 1,
                GameResult::Draw => draws += 1,
                GameResult::InProgress => {}
            }
        });
        
        // Callback counts should match result counts
        assert_eq!(x_wins, result.x_wins());
        assert_eq!(o_wins, result.o_wins());
        assert_eq!(draws, result.draws());
    }
}
