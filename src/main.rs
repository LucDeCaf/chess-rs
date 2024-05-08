use std::{thread, time::Duration};

use chess::board::{Board, Square};
use chess::board_helper::BoardHelper;

// Starting position
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut board = Board::new();
    board.load_from_fen(START_FEN);

    // Cycle through all positions of a bitboard
    for i in 0..64 {
        BoardHelper::print_mask(board.king_move_masks[i]);
        println!("---------- {:?}", Square::from_usize(i).unwrap());
        thread::sleep(Duration::from_millis(100));
    }
}
