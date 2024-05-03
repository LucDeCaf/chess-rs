use std::ops::RangeInclusive;

// sourceing position
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const WHITE_PIECE_MASK_INDEXES: RangeInclusive<usize> = 0..=5;
const BLACK_PIECE_MASK_INDEXES: RangeInclusive<usize> = 6..=11;

#[derive(Debug)]
struct Board {
    bitboards: Vec<u64>,
    is_white_turn: bool,
}

struct Move {
    source: u8,
    target: u8,
}

impl Board {
    fn new(fen: &str) -> Self {
        let mut s = Self {
            bitboards: vec![
                0, 0, 0, 0, 0, 0, // White pieces
                0, 0, 0, 0, 0, 0, // Black pieces
            ],
            is_white_turn: true,
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

        // Check whose turn it is
        self.is_white_turn = match segments.next().expect("FEN string should have 6 segments") {
            "w" => true,
            "b" => false,
            _ => panic!("Second section of FEN string should be either 'w' or 'b'."),
        };
    }

    fn make_move(&mut self, mv: &Move) {
        let (start_mask_indices, target_mask_indices) = match self.is_white_turn {
            true => (WHITE_PIECE_MASK_INDEXES, BLACK_PIECE_MASK_INDEXES),
            false => (BLACK_PIECE_MASK_INDEXES, WHITE_PIECE_MASK_INDEXES),
        };

        // Start with 1 all the way on the left, then adjust from there
        let start_mask = 1 << 63 >> mv.source;
        let target_mask = 1 << 63 >> mv.target;

        let mut piece_found = false;
        for i in start_mask_indices {
            if self.bitboards[i] & start_mask == start_mask {
                piece_found = true;

                // If current mask == mask of moved piece, remove the piece from its current
                // square and replace it on the correct one
                self.bitboards[i] &= !start_mask;
                self.bitboards[i] |= target_mask;
            }
        }

        // Capture enemy pieces if there
        for i in target_mask_indices {
            // Remove any enemy pieces found on the target
            self.bitboards[i] &= !target_mask;
        }

        // Swap turns
        self.is_white_turn = !self.is_white_turn;
    }

    fn unmake_move(&mut self, mv: &Move) {
        let reverse_move = Move {
            source: mv.target,
            target: mv.source,
        };

        self.is_white_turn = !self.is_white_turn;
        self.make_move(&reverse_move);
        self.is_white_turn = !self.is_white_turn;
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
        let string_mask = format!("{:064b}", mask);
        let mut lines = [""; 8];

        let mut i = 7;
        while i < 64 {
            lines[(i + 1) / 8 - 1] = &string_mask[i - 7..=i];
            i += 8;
        }

        for line in lines.iter().rev() {
            println!("{}", line);
        }
    }
}

fn main() {
    let mut board = Board::new(START_FEN);

    let e4 = Move {
        source: 12,  // e2
        target: 28, // e4
    };
    let e5 = Move {
        source: 52,  // e7
        target: 36, // e5
    };
    let c5 = Move {
        source: 50,  // c7
        target: 34, // c5
    };

    board.make_move(&e4);
    board.make_move(&c5);
    board.unmake_move(&c5);
    board.make_move(&e5);

    Board::print_mask(board.pieces_mask());
}
