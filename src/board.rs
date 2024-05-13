use crate::board_helper::BoardHelper;
use crate::moves::{Move, MoveError};
use crate::piece::{Color, Piece};
use crate::square::Square;

#[derive(Debug)]
pub struct Bitboard {
    mask: u64,
    piece: Piece,
}

#[derive(Debug)]
pub struct Board {
    // Game data
    pub current_turn: Color,

    // White pieces
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_kings: Bitboard,

    // Black pieces
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_kings: Bitboard,

    // Piece move tables (to be optimised)
    pub white_pawn_move_masks: [u64; 64],
    pub black_pawn_move_masks: [u64; 64],
    pub white_pawn_capture_masks: [u64; 64],
    pub black_pawn_capture_masks: [u64; 64],
    pub knight_move_masks: [u64; 64],
    pub bishop_move_masks: [u64; 64],
    pub rook_move_masks: [u64; 64],
    pub queen_move_masks: [u64; 64], // queen_move_masks[i] == rook_move_masks[i] | bishop_move_masks[i]
    pub king_move_masks: [u64; 64],
}

impl Board {
    pub fn new() -> Self {
        // Use rook and bishop masks to generate queen masks
        let bishop_move_masks = BoardHelper::generate_bishop_move_masks();
        let rook_move_masks = BoardHelper::generate_rook_move_masks();

        let mut i = 0;
        let queen_move_masks = bishop_move_masks.map(|bishop_mask| {
            let queen_mask = bishop_mask | rook_move_masks[i];
            i += 1;
            queen_mask
        });

        let board = Board {
            current_turn: Color::White,

            white_pawns: Bitboard {
                mask: 0,
                piece: Piece::Pawn(Color::White),
            },
            white_knights: Bitboard {
                mask: 0,
                piece: Piece::Knight(Color::White),
            },
            white_bishops: Bitboard {
                mask: 0,
                piece: Piece::Bishop(Color::White),
            },
            white_rooks: Bitboard {
                mask: 0,
                piece: Piece::Rook(Color::White),
            },
            white_queens: Bitboard {
                mask: 0,
                piece: Piece::Queen(Color::White),
            },
            white_kings: Bitboard {
                mask: 0,
                piece: Piece::King(Color::White),
            },
            black_pawns: Bitboard {
                mask: 0,
                piece: Piece::Pawn(Color::Black),
            },
            black_knights: Bitboard {
                mask: 0,
                piece: Piece::Knight(Color::Black),
            },
            black_bishops: Bitboard {
                mask: 0,
                piece: Piece::Bishop(Color::Black),
            },
            black_rooks: Bitboard {
                mask: 0,
                piece: Piece::Rook(Color::Black),
            },
            black_queens: Bitboard {
                mask: 0,
                piece: Piece::Queen(Color::Black),
            },
            black_kings: Bitboard {
                mask: 0,
                piece: Piece::King(Color::Black),
            },

            white_pawn_move_masks: BoardHelper::generate_white_pawn_move_masks(),
            black_pawn_move_masks: BoardHelper::generate_black_pawn_move_masks(),
            white_pawn_capture_masks: BoardHelper::generate_white_pawn_capture_masks(),
            black_pawn_capture_masks: BoardHelper::generate_black_pawn_capture_masks(),
            knight_move_masks: BoardHelper::generate_knight_move_masks(),
            bishop_move_masks,
            rook_move_masks,
            queen_move_masks,
            king_move_masks: BoardHelper::generate_king_move_masks(),
        };

        board
    }

    pub fn load_from_fen(&mut self, fen: &str) {
        // Reset bitboards
        self.clear_pieces();

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
                    let piece_type = BoardHelper::char_to_piece(ch).unwrap();
                    let bitboard = self.pieces_mut(piece_type);
                    bitboard.mask |= 1 << current_pos;
                    current_pos += 1;
                }

                // Ignore invalid input (at least for now)
                _ => (),
            }
        }

        // Check whose turn it is
        self.current_turn = match segments.next().expect("FEN string should have 6 segments") {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Second section of FEN string should be either 'w' or 'b'."),
        };
    }

    pub fn swap_current_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn pieces_mut<'a>(&'a mut self, piece: Piece) -> &'a mut Bitboard {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => &mut self.white_pawns,
                Color::Black => &mut self.black_pawns,
            },
            Piece::Knight(color) => match color {
                Color::White => &mut self.white_knights,
                Color::Black => &mut self.black_knights,
            },
            Piece::Bishop(color) => match color {
                Color::White => &mut self.white_bishops,
                Color::Black => &mut self.black_bishops,
            },
            Piece::Rook(color) => match color {
                Color::White => &mut self.white_rooks,
                Color::Black => &mut self.black_rooks,
            },
            Piece::Queen(color) => match color {
                Color::White => &mut self.white_queens,
                Color::Black => &mut self.black_queens,
            },
            Piece::King(color) => match color {
                Color::White => &mut self.white_kings,
                Color::Black => &mut self.black_kings,
            },
        }
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), MoveError> {
        // Prevent piece from moving to itself
        if mv.from == mv.to {
            return Err(MoveError);
        }

        // Move piece on its bitboard
        if let Some(from_piece) = self.piece_at_square(mv.from) {
            let from_bitboard = self.pieces_mut(from_piece);
            from_bitboard.mask ^= (1 << mv.from.to_shift()) + (1 << mv.to.to_shift());
        } else {
            return Err(MoveError);
        }

        // Remove piece on target bitboard
        if let Some(to_piece) = self.piece_at_square(mv.to) {
            let to_bitboard = self.pieces_mut(to_piece);
            to_bitboard.mask &= !(1 << mv.to.to_shift());
        }

        self.swap_current_turn();
        Ok(())
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        let reverse_move = Move {
            from: mv.to,
            to: mv.from,
        };

        self.make_move(&reverse_move).unwrap();
        self.swap_current_turn();
    }

    pub fn piece_bitboards<'a>(&'a self) -> [&'a Bitboard; 12] {
        [
            &self.white_pawns,
            &self.white_knights,
            &self.white_bishops,
            &self.white_rooks,
            &self.white_queens,
            &self.white_kings,
            &self.black_pawns,
            &self.black_knights,
            &self.black_bishops,
            &self.black_rooks,
            &self.black_queens,
            &self.black_kings,
        ]
    }

    pub fn piece_at_square(&self, square: Square) -> Option<Piece> {
        let shift = square.to_shift();

        for bitboard in self.piece_bitboards() {
            if bitboard.mask << shift == 1 {
                return Some(bitboard.piece);
            }
        }

        None
    }

    // !====!====! WIP !====!====!
    // pub fn get_pseudolegal_moves(&self) -> Vec<Move> {
    //     let mut moves = vec![];

    //     for bitboard in self.bitboards.iter() {
    //         let mut i = *bitboard;

    //         while i > 0 {
    //             // Check if leftmost bit is 1
    //             let piece_found = i & 1 == 1;

    //             if piece_found {
    //                 let piece = bitboard.
    //             }

    //             // Shift all bits left by one
    //             i <<= 1;
    //         }
    //     }

    //     moves
    // }

    pub fn clear_pieces(&mut self) {
        self.white_pawns.mask = 0;
        self.white_knights.mask = 0;
        self.white_bishops.mask = 0;
        self.white_rooks.mask = 0;
        self.white_queens.mask = 0;
        self.white_kings.mask = 0;
        self.black_pawns.mask = 0;
        self.black_knights.mask = 0;
        self.black_bishops.mask = 0;
        self.black_rooks.mask = 0;
        self.black_queens.mask = 0;
        self.black_kings.mask = 0;
    }

    pub fn pieces_mask(&self) -> u64 {
        self.white_pawns.mask
            | self.white_knights.mask
            | self.white_bishops.mask
            | self.white_rooks.mask
            | self.white_queens.mask
            | self.white_kings.mask
            | self.black_pawns.mask
            | self.black_knights.mask
            | self.black_bishops.mask
            | self.black_rooks.mask
            | self.black_queens.mask
            | self.black_kings.mask
    }
}
