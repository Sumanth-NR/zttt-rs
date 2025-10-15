use zttt_rs::simulation::{Simulator, SimulationConfig};
use zttt_rs::backend::{Board, Player, GameResult, Engine};
use std::env;
use std::process;

/// A perfect play engine using minimax algorithm with alpha-beta pruning
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

fn print_usage() {
    eprintln!("Usage: perfect-engine [OPTIONS]");
    eprintln!();
    eprintln!("Run TicTacToe simulations using the Perfect Engine (minimax with alpha-beta pruning)");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  -n, --num-games <NUMBER>     Number of games to simulate (default: 1000)");
    eprintln!("  -s, --starting-player <X|O>  Starting player (default: X)");
    eprintln!("  -h, --help                   Print this help message");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("  perfect-engine                          # Run 1000 games with default settings");
    eprintln!("  perfect-engine -n 5000                  # Run 5000 games");
    eprintln!("  perfect-engine -n 100 -s O              # Run 100 games with O starting");
}

fn parse_args() -> Result<(usize, Player), String> {
    let args: Vec<String> = env::args().collect();
    let mut num_games = 1000;
    let mut starting_player = Player::X;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-n" | "--num-games" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for --num-games".to_string());
                }
                num_games = args[i + 1]
                    .parse()
                    .map_err(|_| format!("Invalid number: {}", args[i + 1]))?;
                i += 2;
            }
            "-s" | "--starting-player" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for --starting-player".to_string());
                }
                starting_player = match args[i + 1].to_uppercase().as_str() {
                    "X" => Player::X,
                    "O" => Player::O,
                    _ => return Err(format!("Invalid player: {}. Must be X or O", args[i + 1])),
                };
                i += 2;
            }
            arg => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }

    Ok((num_games, starting_player))
}

fn print_header() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Perfect Engine - TicTacToe Simulator               ║");
    println!("║        Optimal Play using Minimax with Alpha-Beta         ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
}

fn print_settings(num_games: usize, starting_player: Player) {
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Simulation Settings                                        │");
    println!("├────────────────────────────────────────────────────────────┤");
    println!("│ Engine:          Perfect Engine (Minimax + Alpha-Beta)    │");
    println!("│ Number of games: {:<42} │", format!("{}", num_games));
    println!("│ Starting player: {:<42} │", format!("{:?}", starting_player));
    println!("└────────────────────────────────────────────────────────────┘");
    println!();
}

fn print_results(result: &zttt_rs::simulation::SimulationResult) {
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Simulation Results                                         │");
    println!("├────────────────────────────────────────────────────────────┤");
    println!("│ Games completed: {:<42} │", result.games_completed());
    println!("│                                                            │");
    println!("│ Outcomes:                                                  │");
    println!("│   X wins:  {:<10} ({:>6.2}%)                            │", 
        result.x_wins(), result.win_rate(Player::X));
    println!("│   O wins:  {:<10} ({:>6.2}%)                            │", 
        result.o_wins(), result.win_rate(Player::O));
    println!("│   Draws:   {:<10} ({:>6.2}%)                            │", 
        result.draws(), result.draw_rate());
    println!("└────────────────────────────────────────────────────────────┘");
    println!();
}

fn print_performance(result: &zttt_rs::simulation::SimulationResult) {
    let total_ms = result.total_duration().as_millis();
    let avg_ms = result.avg_game_duration().as_micros() as f64 / 1000.0;
    let throughput = result.throughput();

    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Performance Metrics                                        │");
    println!("├────────────────────────────────────────────────────────────┤");
    println!("│ Total duration:  {:<41} │", format!("{} ms", total_ms));
    println!("│ Avg per game:    {:<41} │", format!("{:.2} ms", avg_ms));
    println!("│ Throughput:      {:<41} │", format!("{} games/sec", throughput as u64));
    println!("└────────────────────────────────────────────────────────────┘");
    println!();
}

fn print_note() {
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Note                                                       │");
    println!("├────────────────────────────────────────────────────────────┤");
    println!("│ When both players play optimally using the Perfect        │");
    println!("│ Engine, the result is always a draw. This demonstrates    │");
    println!("│ that TicTacToe is a solved game with perfect play.        │");
    println!("└────────────────────────────────────────────────────────────┘");
}

fn main() {
    let (num_games, starting_player) = match parse_args() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!();
            print_usage();
            process::exit(1);
        }
    };

    print_header();
    print_settings(num_games, starting_player);

    println!("Running simulation...");
    println!();

    let config = SimulationConfig::builder()
        .num_games(num_games)
        .engine(PerfectEngine::new())
        .starting_player(starting_player)
        .build();

    let result = Simulator::new(config).run_sequential();

    print_results(&result);
    print_performance(&result);
    print_note();
}
