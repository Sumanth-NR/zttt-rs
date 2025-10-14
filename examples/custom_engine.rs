/// Example demonstrating how to implement custom engines with clear abstractions
///
/// This shows the clean separation between game logic (Board) and move selection (Engine)

use zttt_rs::{Board, Player, GameResult, Engine};

/// A custom engine that picks moves based on priority positions
///
/// This demonstrates the Engine trait abstraction - any strategy can be implemented
/// as long as it provides a `choose_move` implementation.
#[derive(Debug, Clone, Copy)]
struct PriorityEngine {
    /// Priority positions to check first (center, then corners, then edges)
    priorities: [(usize, usize); 9],
}

impl PriorityEngine {
    fn new() -> Self {
        Self {
            // Priority order: center, corners, edges
            priorities: [
                (1, 1), // center
                (0, 0), (0, 2), (2, 0), (2, 2), // corners
                (0, 1), (1, 0), (1, 2), (2, 1), // edges
            ],
        }
    }
}

impl Engine for PriorityEngine {
    fn choose_move(&self, board: &Board, _player: Player) -> Option<(usize, usize)> {
        // Check positions in priority order
        for &pos in &self.priorities {
            if board.is_valid_move(pos.0, pos.1) {
                return Some(pos);
            }
        }
        None
    }
}

/// Simulate a complete game with the given engine
fn simulate_game<E: Engine>(engine: &E) -> GameResult {
    let mut board = Board::new();
    let mut current_player = Player::X;
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        } else {
            break;
        }
    }
    
    board.game_result()
}

fn main() {
    println!("=== Custom Engine Example ===\n");
    println!("This demonstrates the clear abstraction between:");
    println!("  - Game Logic (Board)");
    println!("  - Move Selection (Engine trait)");
    println!("  - Game Simulation (generic over Engine)\n");
    
    // Create custom engine
    let priority_engine = PriorityEngine::new();
    
    println!("Testing PriorityEngine strategy:");
    println!("Priority order: center → corners → edges\n");
    
    // Simulate a few games
    let num_games = 10;
    let mut results = Vec::new();
    
    for _ in 0..num_games {
        results.push(simulate_game(&priority_engine));
    }
    
    // Count results
    let mut wins_x = 0;
    let mut wins_o = 0;
    let mut draws = 0;
    
    for result in &results {
        match result {
            GameResult::Win(Player::X) => wins_x += 1,
            GameResult::Win(Player::O) => wins_o += 1,
            GameResult::Draw => draws += 1,
            _ => {}
        }
    }
    
    println!("Results from {} games:", num_games);
    println!("  X wins: {}", wins_x);
    println!("  O wins: {}", wins_o);
    println!("  Draws:  {}", draws);
    
    println!("\n=== Demonstration Complete ===");
    println!("\nKey Takeaways:");
    println!("1. Clean Engine trait abstraction allows any strategy");
    println!("2. Game simulation is generic over Engine");
    println!("3. Board provides clear game logic procedures");
    println!("4. Easy to implement and test custom strategies");
}
