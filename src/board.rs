use crate::board_helper::BoardHelper;
use crate::mask::Mask;
use crate::move_gen::MoveGen;
use crate::moves::{Move, MoveError};
use crate::piece::{Color, Piece};
use crate::square::Square;

// Starting position
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
struct Bitboard {
    mask: Mask,
    piece: Piece,
}

#[derive(Debug)]
pub struct Board {
    // Game data
    current_turn: Color,

    // White pieces
    white_pawns: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_rooks: Bitboard,
    white_queens: Bitboard,
    white_kings: Bitboard,

    // Black pieces
    black_pawns: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_rooks: Bitboard,
    black_queens: Bitboard,
    black_kings: Bitboard,

    // Piece move tables (to be optimised)
    white_pawn_move_masks: [Mask; 64],
    black_pawn_move_masks: [Mask; 64],
    white_pawn_capture_masks: [Mask; 64],
    black_pawn_capture_masks: [Mask; 64],
    knight_move_masks: [Mask; 64],
    king_move_masks: [Mask; 64],

    // Sliding piece magic bitboard helper struct
    sliding_move_generator: MoveGen,
}

#[allow(unused)]
impl Board {
    pub fn new() -> Self {
        // Use rook and bishop masks to generate queen masks
        let bishop_move_masks = BoardHelper::generate_bishop_move_masks();
        let rook_move_masks = BoardHelper::generate_rook_move_masks();

        let mut i = 0;
        let queen_move_masks = bishop_move_masks.clone().map(|bishop_mask| {
            let queen_mask = Mask(bishop_mask.0.clone() | rook_move_masks[i].0.clone());
            i += 1;
            queen_mask
        });

        let board = Board {
            current_turn: Color::White,

            white_pawns: Bitboard {
                mask: Mask(0),
                piece: Piece::Pawn(Color::White),
            },
            white_knights: Bitboard {
                mask: Mask(0),
                piece: Piece::Knight(Color::White),
            },
            white_bishops: Bitboard {
                mask: Mask(0),
                piece: Piece::Bishop(Color::White),
            },
            white_rooks: Bitboard {
                mask: Mask(0),
                piece: Piece::Rook(Color::White),
            },
            white_queens: Bitboard {
                mask: Mask(0),
                piece: Piece::Queen(Color::White),
            },
            white_kings: Bitboard {
                mask: Mask(0),
                piece: Piece::King(Color::White),
            },
            black_pawns: Bitboard {
                mask: Mask(0),
                piece: Piece::Pawn(Color::Black),
            },
            black_knights: Bitboard {
                mask: Mask(0),
                piece: Piece::Knight(Color::Black),
            },
            black_bishops: Bitboard {
                mask: Mask(0),
                piece: Piece::Bishop(Color::Black),
            },
            black_rooks: Bitboard {
                mask: Mask(0),
                piece: Piece::Rook(Color::Black),
            },
            black_queens: Bitboard {
                mask: Mask(0),
                piece: Piece::Queen(Color::Black),
            },
            black_kings: Bitboard {
                mask: Mask(0),
                piece: Piece::King(Color::Black),
            },

            white_pawn_move_masks: BoardHelper::generate_white_pawn_move_masks(),
            black_pawn_move_masks: BoardHelper::generate_black_pawn_move_masks(),
            white_pawn_capture_masks: BoardHelper::generate_white_pawn_capture_masks(),
            black_pawn_capture_masks: BoardHelper::generate_black_pawn_capture_masks(),
            knight_move_masks: BoardHelper::generate_knight_move_masks(),
            king_move_masks: BoardHelper::generate_king_move_masks(),
            sliding_move_generator: MoveGen::init(),
        };

        board
    }

    pub fn load_from_fen(&mut self, fen: &str) {
        // Reset bitboards
        self.clear_pieces();

        // Get segments of FEN string
        let mut segments = fen.split(' ');

        let rows = segments
            .next()
            .expect("FEN string should have 6 segments")
            .split('/')
            .rev();

        for (i, row) in rows.enumerate() {
            let mut chars = row.chars();
            let mut current_pos = 0;

            while current_pos < 8 {
                if let Some(ch) = chars.next() {
                    match ch {
                        // Skip squares
                        '1'..='8' => {
                            let digit = ch.to_digit(9).unwrap();
                            current_pos += digit as usize;
                        }

                        // Add piece
                        'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                            let piece_type = BoardHelper::char_to_piece(ch).unwrap();
                            let bitboard = self.bitboard_mut(piece_type);
                            bitboard.mask.0 |= 1 << (current_pos + i * 8);
                            current_pos += 1;
                        }

                        // Ignore invalid input (for now)
                        _ => (),
                    }
                } else {
                    break;
                }
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

    fn move_masks(&self, piece: Piece) -> Option<Vec<Mask>> {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => Some(Vec::from_iter(self.white_pawn_move_masks)),
                Color::Black => Some(Vec::from_iter(self.black_pawn_move_masks)),
            },
            Piece::Knight(_) => Some(Vec::from_iter(self.knight_move_masks)),
            Piece::King(_) => Some(Vec::from_iter(self.king_move_masks)),
            _ => None,
        }
    }

    fn bitboard_mut<'a>(&'a mut self, piece: Piece) -> &'a mut Bitboard {
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

    fn bitboard<'a>(&'a self, piece: Piece) -> &'a Bitboard {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => &self.white_pawns,
                Color::Black => &self.black_pawns,
            },
            Piece::Knight(color) => match color {
                Color::White => &self.white_knights,
                Color::Black => &self.black_knights,
            },
            Piece::Bishop(color) => match color {
                Color::White => &self.white_bishops,
                Color::Black => &self.black_bishops,
            },
            Piece::Rook(color) => match color {
                Color::White => &self.white_rooks,
                Color::Black => &self.black_rooks,
            },
            Piece::Queen(color) => match color {
                Color::White => &self.white_queens,
                Color::Black => &self.black_queens,
            },
            Piece::King(color) => match color {
                Color::White => &self.white_kings,
                Color::Black => &self.black_kings,
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
            let from_bitboard = self.bitboard_mut(from_piece);
            from_bitboard.mask.0 ^= (1 << mv.from as usize) + (1 << mv.to as usize);
        } else {
            return Err(MoveError);
        }

        // Remove piece on target bitboard
        if let Some(to_piece) = self.piece_at_square(mv.to) {
            let to_bitboard = self.bitboard_mut(to_piece);
            to_bitboard.mask.0 &= !(1 << mv.to as usize);
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

    fn bitboards<'a>(&'a self) -> [&'a Bitboard; 12] {
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

    fn white_pieces<'a>(&'a self) -> [&'a Bitboard; 6] {
        [
            &self.white_pawns,
            &self.white_knights,
            &self.white_bishops,
            &self.white_rooks,
            &self.white_queens,
            &self.white_kings,
        ]
    }

    fn black_pieces<'a>(&'a self) -> [&'a Bitboard; 6] {
        [
            &self.black_pawns,
            &self.black_knights,
            &self.black_bishops,
            &self.black_rooks,
            &self.black_queens,
            &self.black_kings,
        ]
    }

    fn friendly_pieces<'a>(&'a self, color: &Color) -> [&'a Bitboard; 6] {
        match color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        }
    }

    fn enemy_pieces<'a>(&'a self, color: &Color) -> [&'a Bitboard; 6] {
        match color {
            Color::White => self.black_pieces(),
            Color::Black => self.white_pieces(),
        }
    }

    pub fn piece_at_square(&self, square: Square) -> Option<Piece> {
        let shift = square as usize;

        for bitboard in self.bitboards() {
            if bitboard.mask.0 & 1 << shift > 0 {
                return Some(bitboard.piece);
            }
            // if bitboard.mask.0 << shift == 1 {
            //     return Some(bitboard.piece);
            // }
        }

        None
    }

    pub fn get_pseudolegal_moves(&self, square: Square) -> Option<(Vec<Move>, Piece)> {
        let mut moves = Vec::new();
        let blockers = self.all_pieces_mask();

        let piece = self.piece_at_square(square)?;
        let color = piece.color();

        // Prevent moving pieces of the wrong colour
        if color != self.current_turn {
            return None;
        }

        let bitboard = self.bitboard(piece);

        let mut move_mask: Mask;

        if bitboard.piece.is_slider() {
            move_mask = Mask(0);

            // Orthogonal sliding moves
            match bitboard.piece {
                Piece::Rook(_) | Piece::Queen(_) => {
                    move_mask |= self.sliding_move_generator.get_rook_moves(square, blockers);
                }
                _ => (),
            }

            // Diagonal sliding moves
            match bitboard.piece {
                Piece::Bishop(_) | Piece::Queen(_) => {
                    move_mask |= self
                        .sliding_move_generator
                        .get_bishop_moves(square, blockers);
                }
                _ => (),
            }

            moves.append(&mut Move::from_move_mask(square, move_mask));
        } else {
            // Grab move mask for the piece at the current square
            move_mask = self.move_masks(piece)?[square.to_shift()];

            // Handle potential pawn captures
            if let Piece::Pawn(_) = piece {
                match color {
                    Color::White => {
                        move_mask |= (self.white_pawn_capture_masks[square.to_shift()] & blockers)
                    }
                    Color::Black => {
                        move_mask |= (self.black_pawn_capture_masks[square.to_shift()] & blockers)
                    }
                }
            }
        }

        // Filter out moves that capture one's own pieces
        move_mask &= !self.friendly_pieces_mask(color);

        Some((moves, piece))
    }

    fn clear_pieces(&mut self) {
        self.white_pawns.mask.0 = 0;
        self.white_knights.mask.0 = 0;
        self.white_bishops.mask.0 = 0;
        self.white_rooks.mask.0 = 0;
        self.white_queens.mask.0 = 0;
        self.white_kings.mask.0 = 0;
        self.black_pawns.mask.0 = 0;
        self.black_knights.mask.0 = 0;
        self.black_bishops.mask.0 = 0;
        self.black_rooks.mask.0 = 0;
        self.black_queens.mask.0 = 0;
        self.black_kings.mask.0 = 0;
    }

    fn all_pieces_mask(&self) -> Mask {
        self.white_pieces_mask() | self.black_pieces_mask()
    }

    fn black_pieces_mask(&self) -> Mask {
        self.black_pawns.mask
            | self.black_knights.mask
            | self.black_bishops.mask
            | self.black_rooks.mask
            | self.black_queens.mask
            | self.black_kings.mask
    }

    fn white_pieces_mask(&self) -> Mask {
        self.white_pawns.mask
            | self.white_knights.mask
            | self.white_bishops.mask
            | self.white_rooks.mask
            | self.white_queens.mask
            | self.white_kings.mask
    }

    fn friendly_pieces_mask(&self, color: Color) -> Mask {
        match color {
            Color::White => self.white_pieces_mask(),
            Color::Black => self.black_pieces_mask(),
        }
    }

    fn enemy_pieces_mask(&self, color: Color) -> Mask {
        match color {
            Color::White => self.black_pieces_mask(),
            Color::Black => self.white_pieces_mask(),
        }
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test() {
        let mut board = Board::new();
        board.load_from_fen(START_FEN);

        board
            .make_move(&Move {
                from: Square::G1,
                to: Square::F3,
            })
            .unwrap();

        board.all_pieces_mask().print();
    }
}
