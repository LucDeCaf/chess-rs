const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
struct Board {
    bitboards: Vec<u64>,
}

impl Board {
    fn new(fen: &str) -> Self {
        let mut s = Self {
            bitboards: vec![
                0, 0, 0, 0, 0, 0, // White pieces
                0, 0, 0, 0, 0, 0, // Black pieces
            ],
        };
        s.load_from_fen(fen);
        s
    }

    fn load_from_fen(&mut self, fen: &str) {
        // Reset bitboards
        for board in self.bitboards.iter_mut() {
            *board = 0;
        }

        // Get segments
        let mut segments = fen.split(' ');

        // Load position
        let mut current_pos: usize = 0;
        for ch in segments
            .next()
            .expect("FEN string should have 6 segments")
            .chars()
        {
            match ch {
                // Skip squares
                '1'..='8' => {
                    let digit = ch
                        .to_digit(9)
                        .expect("invalid number of empty squares for FEN string");

                    current_pos += digit as usize;
                }

                // Add piece
                'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                    let board_index = Board::piece_to_bitboard_index(ch);
                    self.bitboards[board_index] |= 1 << current_pos;
                    current_pos += 1;
                }

                // Ignore invalid input (at least for now)
                _ => (),
            }
        }
    }

    fn piece_to_bitboard_index(piece: char) -> usize {
        match piece {
            // White
            'P' => 0,
            'N' => 1,
            'B' => 2,
            'R' => 3,
            'Q' => 4,
            'K' => 5,

            // Black
            'p' => 6,
            'n' => 7,
            'b' => 8,
            'r' => 9,
            'q' => 10,
            'k' => 11,

            _ => panic!("Invalid piece char '{piece}'"),
        }
    }

    fn pieces_mask(&self) -> u64 {
        let mut board_mask: u64 = 0;

        for bitboard in &self.bitboards {
            board_mask |= bitboard;
        }

        board_mask
    }

    fn print_mask(mask: u64) {
        let string_mask = format!("{:064b}", mask.reverse_bits());

        let mut i = 7;
        while i < 64 {
            println!("{}", &string_mask[i-7..=i]);
            i += 8;
        }
    }
}

fn main() {
    let mut board = Board::new(START_FEN);

    Board::print_mask(board.pieces_mask());
}
