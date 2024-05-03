use chess::board_helper::BoardHelper;
use std::ops::RangeInclusive;

// Starting position
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const WHITE_PIECE_MASK_INDEXES: RangeInclusive<usize> = 0..=5;
const BLACK_PIECE_MASK_INDEXES: RangeInclusive<usize> = 6..=11;

#[derive(Debug)]
struct Board {
    // Game data
    bitboards: Vec<u64>,
    is_white_turn: bool,

    // Piece move tables (to be optimised)
    white_pawn_move_masks: [u64; 64],
    black_pawn_move_masks: [u64; 64],
    white_pawn_capture_masks: [u64; 64],
    black_pawn_capture_masks: [u64; 64],
    knight_masks: [u64; 64],
    bishop_masks: [u64; 64],
    rook_masks: [u64; 64],
    queen_masks: [u64; 64], // queen_masks[i] == rook_masks[i] & bishop_masks[i]
    king_masks: [u64; 64],
}

struct Move {
    source: u8,
    target: u8,
}

impl Board {
    fn new(fen: &str) -> Self {
        let mut board = Board {
            bitboards: vec![
                0, 0, 0, 0, 0, 0, // White pieces
                0, 0, 0, 0, 0, 0, // Black pieces
            ],
            is_white_turn: true,
            white_pawn_move_masks: BoardHelper::generate_white_pawn_masks(),
            black_pawn_move_masks: BoardHelper::generate_black_pawn_masks(),
            white_pawn_capture_masks: [0; 64],
            black_pawn_capture_masks: [0; 64],
            knight_masks: [0; 64],
            bishop_masks: [0; 64],
            rook_masks: [0; 64],
            queen_masks: [0; 64],
            king_masks: [0; 64],
        };
        board.load_from_fen(fen);
        board
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
                    let board_index = BoardHelper::piece_to_bitboard_index(ch);
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

        for i in start_mask_indices {
            if self.bitboards[i] & start_mask == start_mask {
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

    fn pieces_mask(&self) -> u64 {
        let mut board_mask: u64 = 0;

        for bitboard in &self.bitboards {
            board_mask |= bitboard;
        }

        board_mask
    }
}

fn main() {
    let mut board = Board::new(START_FEN);

    let e4 = Move {
        source: 12, // e2
        target: 28, // e4
    };
    let e5 = Move {
        source: 52, // e7
        target: 36, // e5
    };
    let c5 = Move {
        source: 50, // c7
        target: 34, // c5
    };

    board.make_move(&e4);
    board.make_move(&c5);
    board.unmake_move(&c5);
    board.make_move(&e5);

    let index = 12;
    BoardHelper::print_mask(
        board.black_pawn_move_masks[index] | board.white_pawn_move_masks[index],
    );
}
