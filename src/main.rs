use std::{thread, time::Duration};

use chess::board::Board;
use chess::board_helper::BoardHelper;

// Starting position
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let board = Board::new(START_FEN);

    for i in 0..64 {
        BoardHelper::print_mask(board.bishop_masks[i]);
        println!("---------- {i}");
        thread::sleep(Duration::from_millis(1000));
    }
}
