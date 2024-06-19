use crate::board_helper::BoardHelper;
use crate::mask::Mask;
use crate::move_gen::MoveGen;
use crate::moves::Move;
use crate::piece::{Color, Piece};
use crate::square::{Rank, Square};

// Starting position
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bitboard {
    mask: Mask,
    piece: Piece,
}

/// Stores all the necessary data to recreate a position on a Board
#[derive(Debug, Clone)]
pub struct BoardState {
    current_turn: Color,

    white_pawns: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_rooks: Bitboard,
    white_queens: Bitboard,
    white_kings: Bitboard,

    black_pawns: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_rooks: Bitboard,
    black_queens: Bitboard,
    black_kings: Bitboard,

    // Used to check if en passant is possible
    prev_move: Option<Move>,
}

impl BoardState {
    pub fn init() -> Self {
        Self {
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
            prev_move: None,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    // Game state
    state: BoardState,

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
            state: BoardState::init(),
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
        self.state.current_turn = match segments.next().expect("FEN string should have 6 segments")
        {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Second segment of FEN string should be either 'w' or 'b'."),
        };
    }

    pub fn swap_current_turn(&mut self) {
        self.state.current_turn = match self.state.current_turn {
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
                Color::White => &mut self.state.white_pawns,
                Color::Black => &mut self.state.black_pawns,
            },
            Piece::Knight(color) => match color {
                Color::White => &mut self.state.white_knights,
                Color::Black => &mut self.state.black_knights,
            },
            Piece::Bishop(color) => match color {
                Color::White => &mut self.state.white_bishops,
                Color::Black => &mut self.state.black_bishops,
            },
            Piece::Rook(color) => match color {
                Color::White => &mut self.state.white_rooks,
                Color::Black => &mut self.state.black_rooks,
            },
            Piece::Queen(color) => match color {
                Color::White => &mut self.state.white_queens,
                Color::Black => &mut self.state.black_queens,
            },
            Piece::King(color) => match color {
                Color::White => &mut self.state.white_kings,
                Color::Black => &mut self.state.black_kings,
            },
        }
    }

    fn bitboard<'a>(&'a self, piece: Piece) -> &'a Bitboard {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => &self.state.white_pawns,
                Color::Black => &self.state.black_pawns,
            },
            Piece::Knight(color) => match color {
                Color::White => &self.state.white_knights,
                Color::Black => &self.state.black_knights,
            },
            Piece::Bishop(color) => match color {
                Color::White => &self.state.white_bishops,
                Color::Black => &self.state.black_bishops,
            },
            Piece::Rook(color) => match color {
                Color::White => &self.state.white_rooks,
                Color::Black => &self.state.black_rooks,
            },
            Piece::Queen(color) => match color {
                Color::White => &self.state.white_queens,
                Color::Black => &self.state.black_queens,
            },
            Piece::King(color) => match color {
                Color::White => &self.state.white_kings,
                Color::Black => &self.state.black_kings,
            },
        }
    }

    /// Makes a move on the board, regardless of whether the move is legal or not.
    pub fn make_move(&mut self, mv: Move) {
        // Remove piece on dest bitboard
        if let Some(to_piece) = self.piece_at_square(mv.to) {
            let to_bitboard = self.bitboard_mut(to_piece);
            to_bitboard.mask.0 &= !(1 << mv.to as usize);
        }

        // Move piece on source bitboard
        if let Some(from_piece) = self.piece_at_square(mv.from) {
            let from_bitboard = self.bitboard_mut(from_piece);
            from_bitboard.mask.0 ^= (1 << mv.from as usize) + (1 << mv.to as usize);
        }

        self.state.prev_move = Some(mv);

        self.swap_current_turn();
    }

    pub fn unmake_move(&mut self, mv: Move) {
        let reverse_move = Move {
            from: mv.to,
            to: mv.from,
        };

        self.make_move(reverse_move);

        self.state.prev_move = None;

        self.swap_current_turn();
    }

    pub fn is_move_legal(&self, mv: Move) -> bool {
        // Prevent piece from moving to itself
        if mv.from == mv.to {
            return false;
        }

        let Some((legal_moves, _)) = self.get_pseudolegal_move_mask(mv.from) else {
            return false;
        };

        (legal_moves.0 & 1 << mv.to as usize) > 0
    }

    fn bitboards<'a>(&'a self) -> [&'a Bitboard; 12] {
        [
            &self.state.white_pawns,
            &self.state.white_knights,
            &self.state.white_bishops,
            &self.state.white_rooks,
            &self.state.white_queens,
            &self.state.white_kings,
            &self.state.black_pawns,
            &self.state.black_knights,
            &self.state.black_bishops,
            &self.state.black_rooks,
            &self.state.black_queens,
            &self.state.black_kings,
        ]
    }

    fn white_pieces<'a>(&'a self) -> [&'a Bitboard; 6] {
        [
            &self.state.white_pawns,
            &self.state.white_knights,
            &self.state.white_bishops,
            &self.state.white_rooks,
            &self.state.white_queens,
            &self.state.white_kings,
        ]
    }

    fn black_pieces<'a>(&'a self) -> [&'a Bitboard; 6] {
        [
            &self.state.black_pawns,
            &self.state.black_knights,
            &self.state.black_bishops,
            &self.state.black_rooks,
            &self.state.black_queens,
            &self.state.black_kings,
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
        for bitboard in self.bitboards() {
            if bitboard.mask.0 & 1 << square as u8 > 0 {
                return Some(bitboard.piece);
            }
        }

        None
    }

    pub fn get_pseudolegal_move_mask(&self, square: Square) -> Option<(Mask, Piece)> {
        let blockers = self.all_pieces_mask();
        let tile_mask = square.mask();

        let piece = self.piece_at_square(square)?;
        let color = piece.color();

        // Prevent moving pieces of the wrong colour
        if color != self.state.current_turn {
            return None;
        }

        let bitboard = self.bitboard(piece);

        let mut move_mask: Mask;

        if bitboard.piece.is_slider() {
            move_mask = Mask(0);

            match bitboard.piece {
                Piece::Rook(_) | Piece::Queen(_) => {
                    move_mask |= self.sliding_move_generator.get_rook_moves(square, blockers);
                }
                Piece::Bishop(_) | Piece::Queen(_) => {
                    move_mask |= self
                        .sliding_move_generator
                        .get_bishop_moves(square, blockers);
                }
                _ => (),
            }
        } else {
            // Grab move mask for the piece at the current square
            move_mask = self.move_masks(piece)?[square.to_shift()];

            if let Piece::Pawn(_) = piece {
                // Handle pawn captures
                match color {
                    Color::White => {
                        move_mask |= (self.white_pawn_capture_masks[square.to_shift()] & blockers)
                    }
                    Color::Black => {
                        move_mask |= (self.black_pawn_capture_masks[square.to_shift()] & blockers)
                    }
                }

                // Handle en passant
                todo!();
            }
        }

        // Filter out moves that capture one's own pieces
        move_mask &= !self.friendly_pieces_mask(color);

        Some((move_mask, piece))
    }

    pub fn get_pseudolegal_moves(&self, square: Square) -> Option<(Vec<Move>, Piece)> {
        let (move_mask, piece) = self.get_pseudolegal_move_mask(square)?;
        Some((Move::from_move_mask(square, move_mask), piece))
    }

    fn clear_pieces(&mut self) {
        self.state.white_pawns.mask.0 = 0;
        self.state.white_knights.mask.0 = 0;
        self.state.white_bishops.mask.0 = 0;
        self.state.white_rooks.mask.0 = 0;
        self.state.white_queens.mask.0 = 0;
        self.state.white_kings.mask.0 = 0;
        self.state.black_pawns.mask.0 = 0;
        self.state.black_knights.mask.0 = 0;
        self.state.black_bishops.mask.0 = 0;
        self.state.black_rooks.mask.0 = 0;
        self.state.black_queens.mask.0 = 0;
        self.state.black_kings.mask.0 = 0;
    }

    fn all_pieces_mask(&self) -> Mask {
        self.white_pieces_mask() | self.black_pieces_mask()
    }

    fn black_pieces_mask(&self) -> Mask {
        self.state.black_pawns.mask
            | self.state.black_knights.mask
            | self.state.black_bishops.mask
            | self.state.black_rooks.mask
            | self.state.black_queens.mask
            | self.state.black_kings.mask
    }

    fn white_pieces_mask(&self) -> Mask {
        self.state.white_pawns.mask
            | self.state.white_knights.mask
            | self.state.white_bishops.mask
            | self.state.white_rooks.mask
            | self.state.white_queens.mask
            | self.state.white_kings.mask
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

        let mv = Move {
            from: Square::E2,
            to: Square::E4,
        };

        if board.is_move_legal(mv) {
            board.make_move(mv);
        } else {
            panic!("Move is illegal");
        }

        board.all_pieces_mask().print();

        board.unmake_move(mv);
    }
}
