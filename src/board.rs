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

/// Stores all the necessary data to recreate a position on a Board
#[derive(Debug, Clone)]
struct BoardState {
    current_turn: Color,

    masks: [Mask; 12],

    // Special moves
    prev_move: Option<Move>,      // En passant
    white_can_castle_short: bool, // White castling kingside
    white_can_castle_long: bool,  // White castling queenside
    black_can_castle_short: bool, // Black castling kingside
    black_can_castle_long: bool,  // Black castling queenside
    halfmoves: u8,                // 50 move rule
    fullmoves: u32,               // Keeping track of game length
}

#[allow(unused)]
impl BoardState {
    fn new() -> Self {
        Self {
            current_turn: Color::White,

            masks: [Mask(0); 12],

            prev_move: None,
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
            halfmoves: 0,
            fullmoves: 0,
        }
    }

    fn white_pieces(&self) -> &[Mask] {
        &self.masks[0..6]
    }

    fn black_pieces(&self) -> &[Mask] {
        &self.masks[6..12]
    }

    fn friendly_pieces(&self, color: Color) -> &[Mask] {
        match color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        }
    }

    fn enemy_pieces(&self, color: Color) -> &[Mask] {
        match color {
            Color::White => self.black_pieces(),
            Color::Black => self.white_pieces(),
        }
    }

    fn piece_to_index(piece: Piece) -> usize {
        let mut index = match piece.color() {
            Color::White => 0,
            Color::Black => 6,
        };
        index += match piece {
            Piece::Pawn(_) => 0,
            Piece::Knight(_) => 1,
            Piece::Bishop(_) => 2,
            Piece::Rook(_) => 3,
            Piece::Queen(_) => 4,
            Piece::King(_) => 5,
        };
        index
    }

    fn index_to_piece(i: usize) -> Option<Piece> {
        match i {
            0 => Some(Piece::Pawn(Color::White)),
            1 => Some(Piece::Knight(Color::White)),
            2 => Some(Piece::Bishop(Color::White)),
            3 => Some(Piece::Rook(Color::White)),
            4 => Some(Piece::Queen(Color::White)),
            5 => Some(Piece::King(Color::White)),
            6 => Some(Piece::Pawn(Color::Black)),
            7 => Some(Piece::Knight(Color::Black)),
            8 => Some(Piece::Bishop(Color::Black)),
            9 => Some(Piece::Rook(Color::Black)),
            10 => Some(Piece::Queen(Color::Black)),
            11 => Some(Piece::King(Color::Black)),
            _ => None,
        }
    }

    fn piece_at_square(&self, square: Square) -> Option<Piece> {
        for (i, mask) in self.masks.iter().enumerate() {
            if mask.0 & 1 << square as u8 > 0 {
                return Self::index_to_piece(i);
            }
        }

        None
    }

    fn mask_mut(&mut self, piece: Piece) -> &mut Mask {
        &mut self.masks[Self::piece_to_index(piece)]
    }

    fn mask(&self, piece: Piece) -> &Mask {
        &self.masks[Self::piece_to_index(piece)]
    }

    fn swap_current_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn clear_pieces(&mut self) {
        for mask in self.masks.iter_mut() {
            mask.0 = 0
        }
    }

    fn all_pieces_mask(&self) -> Mask {
        self.white_pieces_mask() | self.black_pieces_mask()
    }

    fn black_pieces_mask(&self) -> Mask {
        let mut mask = Mask(0);
        for m in self.black_pieces() {
            mask |= *m;
        }
        mask
    }

    fn white_pieces_mask(&self) -> Mask {
        let mut mask = Mask(0);
        for m in self.white_pieces() {
            mask |= *m;
        }
        mask
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

        let mask = self.mask(piece);

        let mut move_mask: Mask;

        if piece.is_slider() {
            move_mask = Mask(0);

            match piece {
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

    ///```
    /// const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// ```
    pub fn load_from_fen(&mut self, fen: &str) {
        // Reset board
        self.states.clear();

        let mut state = BoardState::new();

        // Get segments of FEN string
        let mut segments = fen.split(' ');

        'pieces: {
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
                            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q'
                            | 'k' => {
                                let piece_type = Piece::from_char(ch).unwrap();

                                let mask = state.mask_mut(piece_type);
                                mask.0 |= 1 << (current_pos + i * 8);
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
        }

        'current_turn: {
            state.current_turn = match segments.next().expect("FEN string should have 6 segments") {
                "w" => Color::White,
                "b" => Color::Black,
                _ => panic!("Second segment of FEN string should be either 'w' or 'b'."),
            };
        }

        'castling_rights: {
            let castling_rights = segments.next().expect("FEN string should have 6 segments");

            state.white_can_castle_short = false;
            state.white_can_castle_long = false;
            state.black_can_castle_short = false;
            state.black_can_castle_long = false;

            let mut prev: u8 = 0;

            for ch in castling_rights.chars() {
                match ch {
                'K' => {
                    if prev > 0 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    prev = 1;
                    state.white_can_castle_short = true;
                }
                'Q' => {
                    if prev > 1 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    prev = 2;
                    state.white_can_castle_short = true;
                }
                'k' => {
                    if prev > 2 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    prev = 3;
                    state.white_can_castle_short = true;
                }
                'q' => {
                    if prev > 3 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    state.white_can_castle_short = true;
                }
                _ => panic!("Third segment of FEN string should only contain the characters 'K', 'Q', 'k', 'q'"),
            }
            }
        }

        'en_passant: {
            state.prev_move = None;

            let next_segment = segments.next().expect("FEN string should have 6 segments");

            let Some(square) = Square::from_str(next_segment) else {
                break 'en_passant;
            };

            let above_rank = square
                .rank()
                .plus(1)
                .expect("En passant square should be on rank 3 or rank 6");
            let below_rank = square
                .rank()
                .minus(1)
                .expect("En passant square should be on rank 3 or rank 6");
            let file = square.file();

            let mut mv = match above_rank {
                Rank::Four => Move {
                    from: Square::from_coords(below_rank, file),
                    to: Square::from_coords(above_rank, file),
                },
                Rank::Seven => Move {
                    from: Square::from_coords(above_rank, file),
                    to: Square::from_coords(below_rank, file),
                },
                _ => panic!("En passant square should be on rank 3 or rank 6"),
            };

            state.prev_move = Some(mv);
        }

        'halfmoves: {
            let halfmoves = segments.next().expect("FEN string should have 6 segments");
            let halfmoves = halfmoves
                .to_owned()
                .parse::<u8>()
                .expect("Halfmove counter should be a number from 0 to 100");

            if halfmoves > 100 {
                panic!("Halfmove counter should be a number from 0 to 100");
            }

            state.halfmoves = halfmoves;
        }

        'fullmoves: {
            let fullmoves = segments.next().expect("FEN string should have 6 segments");
            let fullmoves = fullmoves
                .to_owned()
                .parse::<u32>()
                .expect("Halfmove counter should be a number from 0 to 100");

            state.fullmoves = fullmoves;
        }

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
        let mut is_capture = false;

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
                Color::White => &mut new_state.masks[Piece::BLACK_PAWN_INDEX],
                Color::Black => &mut new_state.masks[Piece::WHITE_PAWN_INDEX],
            };
            *enemy_pawns &= !(Square::from_coords(offset_rank, file)).mask();
        } else if let Some(to_piece) = new_state.piece_at_square(mv.to) {
            is_capture = true;

            // Remove captured piece
            let mask = new_state.mask_mut(to_piece);
            mask.0 &= !(1 << mv.to as usize);
        }

        let from_mask = new_state.mask_mut(from_piece);
        from_mask.0 ^= (1 << mv.from as usize) + (1 << mv.to as usize);

        // Update move counts
        if new_state.current_turn == Color::Black {
            new_state.fullmoves += 1;
        }
        new_state.halfmoves = match from_piece {
            Piece::Pawn(_) => 0,
            _ => {
                if is_capture {
                    0
                } else {
                    new_state.halfmoves + 1
                }
            }
        };

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
    fn load_from_fen() {
        const TEST_POSITION_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let mut board = Board::new();
        board.load_from_fen(TEST_POSITION_FEN);
        board.current_state().unwrap().all_pieces_mask().print();
    }

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
