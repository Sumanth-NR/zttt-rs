use zttt_rs::backend::{Board, Player, GameResult, Engine};

/// A perfect play engine using minimax algorithm with alpha-beta pruning
/// (Included here for demonstration purposes)
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
