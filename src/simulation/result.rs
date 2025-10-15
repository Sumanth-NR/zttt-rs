//! Simulation results and statistics

use std::time::Duration;
use crate::backend::Player;

/// Results and statistics from a completed simulation
///
/// Contains comprehensive information about the simulation run including
/// game outcomes, performance metrics, and throughput statistics.
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
/// let result = Simulator::new(config).run_sequential();
/// println!("X win rate: {:.2}%", result.win_rate(Player::X));
/// println!("Throughput: {} games/sec", result.throughput() as u64);
/// ```
#[derive(Debug, Clone)]
pub struct SimulationResult {
    games_completed: usize,
    x_wins: usize,
    o_wins: usize,
    draws: usize,
    total_duration: Duration,
}

impl SimulationResult {
    /// Create a new simulation result
    pub(crate) fn new(
        games_completed: usize,
        x_wins: usize,
        o_wins: usize,
        draws: usize,
        total_duration: Duration,
    ) -> Self {
        Self {
            games_completed,
            x_wins,
            o_wins,
            draws,
            total_duration,
        }
    }

    /// Get the total number of completed games
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
    /// let result = Simulator::new(config).run_sequential();
    /// assert_eq!(result.games_completed(), 1000);
    /// ```
    pub fn games_completed(&self) -> usize {
        self.games_completed
    }

    /// Get the number of games won by X
    pub fn x_wins(&self) -> usize {
        self.x_wins
    }

    /// Get the number of games won by O
    pub fn o_wins(&self) -> usize {
        self.o_wins
    }

    /// Get the number of drawn games
    pub fn draws(&self) -> usize {
        self.draws
    }

    /// Get the total duration of the simulation
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    /// Get the average duration per game
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
    /// let result = Simulator::new(config).run_sequential();
    /// println!("Average: {:?}", result.avg_game_duration());
    /// ```
    pub fn avg_game_duration(&self) -> Duration {
        if self.games_completed == 0 {
            Duration::ZERO
        } else {
            self.total_duration / self.games_completed as u32
        }
    }

    /// Get the throughput in games per second
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
    /// println!("Throughput: {} games/sec", result.throughput() as u64);
    /// ```
    pub fn throughput(&self) -> f64 {
        let secs = self.total_duration.as_secs_f64();
        if secs == 0.0 {
            0.0
        } else {
            self.games_completed as f64 / secs
        }
    }

    /// Get the win rate for a specific player as a percentage
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
    /// let result = Simulator::new(config).run_sequential();
    /// println!("X win rate: {:.2}%", result.win_rate(Player::X));
    /// ```
    pub fn win_rate(&self, player: Player) -> f64 {
        if self.games_completed == 0 {
            0.0
        } else {
            let wins = match player {
                Player::X => self.x_wins,
                Player::O => self.o_wins,
            };
            (wins as f64 / self.games_completed as f64) * 100.0
        }
    }

    /// Get the draw rate as a percentage
    pub fn draw_rate(&self) -> f64 {
        if self.games_completed == 0 {
            0.0
        } else {
            (self.draws as f64 / self.games_completed as f64) * 100.0
        }
    }
}
