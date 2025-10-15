use zttt_rs::simulation::{Simulator, SimulationConfig};
use zttt_rs::backend::{Board, Player, GameResult, Engine};

/// A perfect play engine using minimax algorithm with alpha-beta pruning
/// This demonstrates that the simulation module works with any Engine implementation
#[derive(Debug, Clone, Copy)]
struct PerfectEngine;

impl PerfectEngine {
    fn new() -> Self {
        PerfectEngine
    }

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
                new_board.make_move(row, col, current_player).unwrap();
                let eval = self.minimax(&new_board, maximizing_player, current_player.opponent(), alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &(row, col) in &board.valid_moves() {
                let mut new_board = board.clone();
                new_board.make_move(row, col, current_player).unwrap();
                let eval = self.minimax(&new_board, maximizing_player, current_player.opponent(), alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
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
            new_board.make_move(row, col, player).unwrap();
            let score = self.minimax(&new_board, player, player.opponent(), i32::MIN, i32::MAX, false);
            
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        Some(best_move)
    }
}

fn main() {
    println!("=== Perfect Engine Simulation (Optimal Play) ===\n");
    
    println!("Running 1,000 games with optimal play...");
    
    let config = SimulationConfig::builder()
        .num_games(1_000)
        .engine(PerfectEngine::new())
        .starting_player(Player::X)
        .build();
    
    let result = Simulator::new(config).run_sequential();
    
    println!("\nResults:");
    println!("Completed: {} games", result.games_completed());
    println!("X wins: {} ({:.2}%)", result.x_wins(), result.win_rate(Player::X));
    println!("O wins: {} ({:.2}%)", result.o_wins(), result.win_rate(Player::O));
    println!("Draws: {} ({:.2}%)", result.draws(), result.draw_rate());
    println!("\nPerformance:");
    println!("Duration: {:?}", result.total_duration());
    println!("Avg per game: {:.2}ms", result.avg_game_duration().as_micros() as f64 / 1000.0);
    println!("Throughput: {} games/sec", result.throughput() as u64);
    
    println!("\nNote: When both players play optimally, the result is always a draw!");
    println!("This demonstrates the simulation module works with any Engine implementation.");
}
