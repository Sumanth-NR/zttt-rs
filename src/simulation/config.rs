//! Configuration for simulation runs

use crate::backend::Player;

/// Configuration for running game simulations
///
/// Use the builder pattern to construct a configuration:
///
/// ```
/// use zttt_rs::simulation::SimulationConfig;
/// use zttt_rs::backend::{FastEngine, Player};
///
/// let config = SimulationConfig::builder()
///     .num_games(10_000)
///     .engine(FastEngine)
///     .starting_player(Player::X)
///     .build();
/// ```
pub struct SimulationConfig<E> {
    pub(crate) num_games: usize,
    pub(crate) engine: E,
    pub(crate) starting_player: Player,
    pub(crate) seed: Option<u64>,
}

impl SimulationConfig<()> {
    /// Create a new configuration builder
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::SimulationConfig;
    /// use zttt_rs::backend::{FastEngine, Player};
    ///
    /// let config = SimulationConfig::builder()
    ///     .num_games(1000)
    ///     .engine(FastEngine)
    ///     .starting_player(Player::X)
    ///     .build();
    /// ```
    pub fn builder() -> SimulationConfigBuilder {
        SimulationConfigBuilder::default()
    }
}

/// Builder for SimulationConfig
///
/// Provides a fluent interface for constructing simulation configurations.
#[derive(Default)]
pub struct SimulationConfigBuilder {
    num_games: Option<usize>,
    starting_player: Option<Player>,
    seed: Option<u64>,
}

impl SimulationConfigBuilder {
    /// Set the number of games to simulate
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::SimulationConfig;
    ///
    /// let builder = SimulationConfig::builder()
    ///     .num_games(10_000);
    /// ```
    pub fn num_games(mut self, num_games: usize) -> Self {
        self.num_games = Some(num_games);
        self
    }

    /// Set the engine to use for move selection
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::SimulationConfig;
    /// use zttt_rs::backend::FastEngine;
    ///
    /// let builder = SimulationConfig::builder()
    ///     .engine(FastEngine);
    /// ```
    pub fn engine<E>(self, engine: E) -> SimulationConfigBuilderWithEngine<E> {
        SimulationConfigBuilderWithEngine {
            num_games: self.num_games,
            engine,
            starting_player: self.starting_player,
            seed: self.seed,
        }
    }

    /// Set the starting player
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::SimulationConfig;
    /// use zttt_rs::backend::Player;
    ///
    /// let builder = SimulationConfig::builder()
    ///     .starting_player(Player::X);
    /// ```
    pub fn starting_player(mut self, player: Player) -> Self {
        self.starting_player = Some(player);
        self
    }

    /// Set an optional seed for reproducibility
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::SimulationConfig;
    ///
    /// let builder = SimulationConfig::builder()
    ///     .seed(42);
    /// ```
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

/// Builder for SimulationConfig with an engine type
pub struct SimulationConfigBuilderWithEngine<E> {
    num_games: Option<usize>,
    engine: E,
    starting_player: Option<Player>,
    seed: Option<u64>,
}

impl<E> SimulationConfigBuilderWithEngine<E> {
    /// Set the number of games to simulate
    pub fn num_games(mut self, num_games: usize) -> Self {
        self.num_games = Some(num_games);
        self
    }

    /// Set the starting player
    pub fn starting_player(mut self, player: Player) -> Self {
        self.starting_player = Some(player);
        self
    }

    /// Set an optional seed for reproducibility
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Build the configuration
    ///
    /// # Panics
    ///
    /// Panics if required fields (num_games, starting_player) are not set.
    ///
    /// # Example
    ///
    /// ```
    /// use zttt_rs::simulation::SimulationConfig;
    /// use zttt_rs::backend::{FastEngine, Player};
    ///
    /// let config = SimulationConfig::builder()
    ///     .num_games(10_000)
    ///     .engine(FastEngine)
    ///     .starting_player(Player::X)
    ///     .build();
    /// ```
    pub fn build(self) -> SimulationConfig<E> {
        SimulationConfig {
            num_games: self.num_games.expect("num_games must be set"),
            engine: self.engine,
            starting_player: self.starting_player.expect("starting_player must be set"),
            seed: self.seed,
        }
    }
}
