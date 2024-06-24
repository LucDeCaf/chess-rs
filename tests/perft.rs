#[cfg(test)]
pub mod perft {
    use chess::board::{Board, START_FEN};

    fn perft(board: &mut Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = 0;

        let legal_moves = board.legal_moves();

        for legal_move in legal_moves {
            board.make_move(legal_move).unwrap();
            moves += perft(board, depth - 1);
            board.unmake_move().unwrap();
        }

        moves
    }

    #[test]
    fn starting_position() {
        let mut board = Board::new(START_FEN).unwrap();

        let nodes = perft(&mut board, 1);
        let legal_moves = board.legal_moves();

        dbg!(legal_moves);
        println!("{}", nodes);
    }
}
