use crate::mask::Mask;
use crate::move_gen::{
    masks::{
        BLACK_PAWN_CAPTURE_MASKS, BLACK_PAWN_MOVE_MASKS, KING_MOVE_MASKS, KNIGHT_MOVE_MASKS,
        WHITE_PAWN_CAPTURE_MASKS, WHITE_PAWN_MOVE_MASKS,
    },
    move_gen::MoveGen,
};
use crate::moves::{Move, MoveError};
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
struct BoardState {
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

    // Special moves
    prev_move: Option<Move>,                   // En passant
    can_castle_short: bool,                    // Castling kingside
    can_castle_long: bool,                     // Castling queenside
    moves_since_last_capture_or_pawn_move: u8, // 50 move rule
}

#[allow(unused)]
impl BoardState {
    fn new() -> Self {
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
            can_castle_short: true,
            can_castle_long: true,
            moves_since_last_capture_or_pawn_move: 0,
        }
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

    fn piece_at_square(&self, square: Square) -> Option<Piece> {
        for bitboard in self.bitboards() {
            if bitboard.mask.0 & 1 << square as u8 > 0 {
                return Some(bitboard.piece);
            }
        }

        None
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

    fn swap_current_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
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

    pub fn possible_en_passant(&self) -> bool {
        let current_turn = self.current_turn;
        let Some(last_move) = self.prev_move else {
            return false;
        };

        let Some(piece) = self.piece_at_square(last_move.to) else {
            return false;
        };

        match piece {
            Piece::Pawn(_) => (),
            _ => return false,
        };

        let rank_diff = last_move.rank_diff();
        let file_diff = last_move.file_diff();
        if rank_diff != 2 || file_diff != 0 {
            return false;
        }

        return current_turn == Color::White && last_move.to.rank() == Rank::Five
            || current_turn == Color::Black && last_move.to.rank() == Rank::Four;
    }

    pub fn en_passant_mask(&self) -> Option<Mask> {
        let current_turn = self.current_turn;
        let last_move = self.prev_move?;

        let to_rank = last_move.to.rank();
        let from_rank = last_move.from.rank();

        if !self.possible_en_passant() {
            return None;
        }

        let capture_rank = match current_turn {
            Color::White => to_rank.plus(1)?,
            Color::Black => to_rank.minus(1)?,
        };
        let capture_file = last_move.to.file();

        Some(Square::from_coords(capture_rank, capture_file).mask())
    }

    pub fn is_move_legal(&self, mv: Move, sliding_move_generator: &MoveGen) -> bool {
        // Prevent piece from moving to itself
        if mv.from == mv.to {
            return false;
        }

        let Some((legal_moves, _)) =
            self.get_pseudolegal_move_mask(mv.from, sliding_move_generator)
        else {
            return false;
        };

        (legal_moves.0 & 1 << mv.to as usize) > 0
    }

    fn move_masks(&self, piece: Piece) -> Option<Vec<Mask>> {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => Some(Vec::from_iter(WHITE_PAWN_MOVE_MASKS)),
                Color::Black => Some(Vec::from_iter(BLACK_PAWN_MOVE_MASKS)),
            },
            Piece::Knight(_) => Some(Vec::from_iter(KNIGHT_MOVE_MASKS)),
            Piece::King(_) => Some(Vec::from_iter(KING_MOVE_MASKS)),
            _ => None,
        }
    }

    pub fn get_pseudolegal_move_mask(
        &self,
        square: Square,
        sliding_move_generator: &MoveGen,
    ) -> Option<(Mask, Piece)> {
        let blockers = self.all_pieces_mask();
        let tile_mask = square.mask();

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

            match bitboard.piece {
                Piece::Rook(_) | Piece::Queen(_) => {
                    move_mask |= sliding_move_generator.get_rook_moves(square, blockers);
                }
                Piece::Bishop(_) | Piece::Queen(_) => {
                    move_mask |= sliding_move_generator.get_bishop_moves(square, blockers);
                }
                _ => (),
            }
        } else {
            // Grab move mask for the piece at the current square
            move_mask = self.move_masks(piece)?[square.to_shift()];

            if let Piece::Pawn(_) = piece {
                // Handle pawn captures and en passant
                let capture_mask = match color {
                    Color::White => WHITE_PAWN_CAPTURE_MASKS[square.to_shift()],
                    Color::Black => BLACK_PAWN_CAPTURE_MASKS[square.to_shift()],
                };

                move_mask |= capture_mask & blockers;

                if let Some(en_passant_mask) = self.en_passant_mask() {
                    // If en passant mask can be found in capture mask
                    if capture_mask & en_passant_mask != Mask(0) {
                        move_mask |= en_passant_mask;
                    }
                }
            }
        }

        // Filter out moves that capture one's own pieces
        move_mask &= !self.friendly_pieces_mask(color);

        Some((move_mask, piece))
    }

    pub fn get_pseudolegal_moves(
        &self,
        square: Square,
        sliding_move_generator: &MoveGen,
    ) -> Option<(Vec<Move>, Piece)> {
        let (move_mask, piece) = self.get_pseudolegal_move_mask(square, sliding_move_generator)?;
        Some((Move::from_move_mask(square, move_mask), piece))
    }
}

#[derive(Debug)]
pub struct Board {
    // Board state and state history
    states: Vec<BoardState>,

    // Sliding piece magic bitboard helper struct
    sliding_move_generator: MoveGen,
}

#[allow(unused)]
impl Board {
    pub fn new() -> Self {
        // Use rook and bishop masks to generate queen masks
        let board = Board {
            states: Vec::new(),
            sliding_move_generator: MoveGen::init(),
        };

        board
    }

    fn current_state(&self) -> Result<&BoardState, MoveError> {
        match self.states.last() {
            Some(state) => Ok(state),
            None => Err(MoveError::NoBoardState),
        }
    }

    fn current_state_mut(&mut self) -> Result<&mut BoardState, MoveError> {
        match self.states.last_mut() {
            Some(state) => Ok(state),
            None => Err(MoveError::NoBoardState),
        }
    }

    fn char_to_piece(ch: char) -> Option<Piece> {
        match ch {
            'P' => Some(Piece::Pawn(Color::White)),
            'N' => Some(Piece::Knight(Color::White)),
            'B' => Some(Piece::Bishop(Color::White)),
            'R' => Some(Piece::Rook(Color::White)),
            'Q' => Some(Piece::Queen(Color::White)),
            'K' => Some(Piece::King(Color::White)),
            'p' => Some(Piece::Pawn(Color::Black)),
            'n' => Some(Piece::Knight(Color::Black)),
            'b' => Some(Piece::Bishop(Color::Black)),
            'r' => Some(Piece::Rook(Color::Black)),
            'q' => Some(Piece::Queen(Color::Black)),
            'k' => Some(Piece::King(Color::Black)),
            _ => None,
        }
    }

    pub fn load_from_fen(&mut self, fen: &str) {
        // Reset board
        self.states.clear();

        let mut state = BoardState::new();

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
                            let piece_type = Self::char_to_piece(ch).unwrap();

                            let bitboard = state.bitboard_mut(piece_type);
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
        state.current_turn = match segments.next().expect("FEN string should have 6 segments") {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Second segment of FEN string should be either 'w' or 'b'."),
        };

        self.states.push(state);
    }

    /// Makes a move on the board, regardless of whether the move is legal or not.
    pub fn make_move(&mut self, mv: Move) -> Result<(), MoveError> {
        // Store copy of old board state
        let mut new_state = self.current_state()?.clone();
        let current_turn = new_state.current_turn;

        let Some(from_piece) = new_state.piece_at_square(mv.from) else {
            return Err(MoveError::MissingPiece);
        };

        // Check if is en passant
        let is_en_passant = match from_piece {
            // Check if moved piece is a pawn
            Piece::Pawn(_) => {
                if let Some(mask) = new_state.en_passant_mask() {
                    // Check if en passant mask equals move mask
                    mask == mv.to.mask()
                } else {
                    false
                }
            }
            _ => false,
        };

        // Update board state
        if is_en_passant {
            let rank = mv.to.rank();
            let file = mv.to.file();

            let offset_rank = match current_turn {
                Color::White => rank.minus(1),
                Color::Black => rank.plus(1),
            }
            .unwrap();

            // Capture the pawn when en passant is played
            let enemy_pawns = match current_turn {
                Color::White => &mut new_state.black_pawns,
                Color::Black => &mut new_state.white_pawns,
            };
            enemy_pawns.mask &= !(Square::from_coords(offset_rank, file)).mask();
        } else if let Some(to_piece) = new_state.piece_at_square(mv.to) {
            let to_bitboard = new_state.bitboard_mut(to_piece);
            to_bitboard.mask.0 &= !(1 << mv.to as usize);
        }

        let from_bitboard = new_state.bitboard_mut(from_piece);
        from_bitboard.mask.0 ^= (1 << mv.from as usize) + (1 << mv.to as usize);

        new_state.prev_move = Some(mv);
        new_state.swap_current_turn();

        // Add new state to state history
        self.states.push(new_state);

        Ok(())
    }

    pub fn unmake_move(&mut self) {
        self.states.pop();
    }

    pub fn is_move_legal(&self, mv: Move) -> bool {
        let Ok(state) = self.current_state() else {
            return false;
        };

        state.is_move_legal(mv, &self.sliding_move_generator)
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn en_passant_legal_move() {
        let mut board = Board::new();
        board.load_from_fen(START_FEN);

        let _ = board.make_move(Move {
            from: Square::E2,
            to: Square::E5,
        });

        let _ = board.make_move(Move {
            from: Square::D7,
            to: Square::D5,
        });

        assert!(board.is_move_legal(Move {
            from: Square::E5,
            to: Square::D6,
        }));
    }

    #[test]
    fn en_passant_mask() {
        let mut board = Board::new();
        board.load_from_fen(START_FEN);

        let _ = board.make_move(Move {
            from: Square::E2,
            to: Square::E4,
        });

        assert_eq!(
            board.current_state().unwrap().en_passant_mask(),
            Some(Square::E3.mask())
        );

        let _ = board.make_move(Move {
            from: Square::E7,
            to: Square::E5,
        });

        assert_eq!(
            board.current_state().unwrap().en_passant_mask(),
            Some(Square::E6.mask())
        );

        let _ = board.make_move(Move {
            from: Square::G1,
            to: Square::F3,
        });

        assert_eq!(board.current_state().unwrap().en_passant_mask(), None);
    }

    #[test]
    fn en_passant_captures_pawn() {
        let mut board = Board::new();
        board.load_from_fen(START_FEN);

        let _ = board.make_move(Move {
            from: Square::E2,
            to: Square::E5,
        });

        let _ = board.make_move(Move {
            from: Square::D7,
            to: Square::D5,
        });

        let _ = board.make_move(Move {
            from: Square::E5,
            to: Square::D6,
        });

        board.current_state().unwrap().all_pieces_mask().print();
    }
}
