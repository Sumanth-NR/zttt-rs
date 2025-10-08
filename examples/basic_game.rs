use zttt_rs::{Board, GameResult, Player};

fn main() {
    println!("=== Basic TicTacToe Game Example ===\n");

    // Create a new board
    let mut board = Board::new();

    // Make some moves
    println!("Making moves...");
    board.make_move(0, 0, Player::X).unwrap();
    println!("X plays at (0, 0)");

    board.make_move(1, 1, Player::O).unwrap();
    println!("O plays at (1, 1)");

    board.make_move(0, 1, Player::X).unwrap();
    println!("X plays at (0, 1)");

    board.make_move(2, 2, Player::O).unwrap();
    println!("O plays at (2, 2)");

    board.make_move(0, 2, Player::X).unwrap();
    println!("X plays at (0, 2)");

    // Display the board
    println!("\nFinal board:");
    println!("{}", board);

    // Check game status
    println!("\nGame result:");
    match board.game_result() {
        GameResult::Win(player) => println!("{} wins!", player),
        GameResult::Draw => println!("It's a draw!"),
        GameResult::InProgress => println!("Game in progress"),
    }
}
