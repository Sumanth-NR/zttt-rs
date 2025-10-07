use zttt_rs::{Board, Player, GameResult};
use std::time::Instant;

fn simulate_game() -> GameResult {
    let mut board = Board::new();
    let mut current_player = Player::X;
    
    while board.game_result() == GameResult::InProgress {
        if let Some((row, col)) = board.best_move(current_player) {
            board.make_move(row, col, current_player).unwrap();
            current_player = current_player.opponent();
        }
    }
    
    board.game_result()
}

fn main() {
    println!("=== Running Multiple TicTacToe Simulations ===\n");
    
    let num_games = 10_000;
    
    let start = Instant::now();
    
    let mut wins_x = 0;
    let mut wins_o = 0;
    let mut draws = 0;
    
    for _ in 0..num_games {
        match simulate_game() {
            GameResult::Win(Player::X) => wins_x += 1,
            GameResult::Win(Player::O) => wins_o += 1,
            GameResult::Draw => draws += 1,
            _ => {}
        }
    }
    
    let duration = start.elapsed();
    
    println!("Completed {} games in {:?}", num_games, duration);
    println!("Average time per game: {:.2}µs", duration.as_micros() as f64 / num_games as f64);
    println!("\nResults:");
    println!("X wins: {} ({:.2}%)", wins_x, (wins_x as f64 / num_games as f64) * 100.0);
    println!("O wins: {} ({:.2}%)", wins_o, (wins_o as f64 / num_games as f64) * 100.0);
    println!("Draws:  {} ({:.2}%)", draws, (draws as f64 / num_games as f64) * 100.0);
    
    println!("\nNote: When both players play optimally, the result is always a draw!");
}
