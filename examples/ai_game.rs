use zttt_rs::{Board, Engine, GameResult, PerfectEngine, Player};

fn main() {
    println!("=== Engine vs Engine TicTacToe Game ===\n");

    let mut board = Board::new();
    let mut current_player = Player::X;
    let mut move_count = 0;
    let engine = PerfectEngine::new();

    println!("Starting position:");
    println!("{}\n", board);

    while board.game_result() == GameResult::InProgress {
        move_count += 1;

        // Get the best move for current player
        if let Some((row, col)) = engine.choose_move(&board, current_player) {
            board.make_move(row, col, current_player).unwrap();
            println!("Move {}: {} plays at ({}, {})", move_count, current_player, row, col);
            println!("{}\n", board);

            current_player = current_player.opponent();
        }
    }

    println!("Game over!");
    match board.game_result() {
        GameResult::Win(player) => println!("{} wins!", player),
        GameResult::Draw => println!("It's a draw!"),
        GameResult::InProgress => println!("Game in progress (should not happen)"),
    }
}
