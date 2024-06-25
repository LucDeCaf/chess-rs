#[cfg(test)]
pub mod perft {
    use chess::board::{Board, START_FEN};

    fn perft(board: &mut Board, depth: usize) -> u64 {
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

    fn divide(fen: &str, depth: usize) -> Vec<String> {
        let mut board = Board::new(fen).unwrap();

        let moves = board.legal_moves();

        let mut results = Vec::with_capacity(moves.len());

        for mv in moves {
            board.make_move(mv).unwrap();
            let nodes = perft(&mut board, depth - 1);
            board.unmake_move().unwrap();

            results.push(format!(
                "{}{} {}",
                mv.from.to_string(),
                mv.to.to_string(),
                nodes
            ));
        }

        results
    }

    #[test]
    fn starting_position() {
        let roce = "a4a5 418\nb2b3 440\nb2b4 441\nc2c3 420\nc2c4 441\nd2d3 559\nd2d4 580\ne2e3 619\ne2e4 620\nf2f3 400\nf2f4 421\ng2g3 440\ng2g4 441\nh2h3 400\nh2h4 440\na1a2 420\na1a3 540\nb1c3 460\nb1a3 440\ng1h3 420\ng1f3 460";
        // const EXPECTED: &[u64] = &[1, 20, 400, 8902, 197218, 4865609];

        let depth = 3;

        let mut chress = divide(
            "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR w  - 0 1",
            depth,
        );
        chress.sort();

        let mut roce = roce.lines().map(|s| s.to_string()).collect::<Vec<String>>();
        roce.sort();

        for i in 0..chress.len() {
            let roce_res = &roce[i];
            let chress_res = &chress[i];

            let move_str = roce_res.split(" ").next().unwrap();
            let roce_moves = roce_res.split(" ").last().unwrap();
            let chress_moves = chress_res.split(" ").last().unwrap();

            if roce_res != chress_res {
                println!("{move_str}: {roce_moves} vs. {chress_moves}");
            }
            // println!("{} vs. ", chress[i]);
        }
    }
}
