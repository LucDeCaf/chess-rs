pub mod mask;
pub mod moves;
pub mod piece;
pub mod square;

use crate::board::mask::Mask;
use crate::board::moves::{Move, MoveError};
use crate::board::piece::{Color, Piece};
use crate::board::square::{Rank, Square};
use crate::move_gen::move_masks::{
    BISHOP_MOVE_MASKS, BLACK_PAWN_CAPTURE_MASKS, BLACK_PAWN_MOVE_MASKS, KING_MOVE_MASKS,
    KNIGHT_MOVE_MASKS, ROOK_MOVE_MASKS, WHITE_PAWN_CAPTURE_MASKS, WHITE_PAWN_MOVE_MASKS,
};
use crate::move_gen::SlidingMoves;

// Starting position
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Testing position
pub const TEST_POSITION_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

#[derive(Debug)]
pub enum FenError {
    BadPosition,
    BadActiveColor,
    BadCastlingRights,
    BadEnPassant,
    BadHalfmoves,
    BadFullmoves,
    MissingSection,
    TooManySections,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleDirection {
    Kingside,
    Queenside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialMove {
    Castle(CastleDirection),
    EnPassant,
    Promotion,
}

/// Stores all the necessary data to recreate a position on a Board
#[derive(Debug, Clone)]
pub struct BoardState {
    active_color: Color,
    masks: [Mask; 12],

    // Historical data
    last_move: Option<Move>, // En passant
    a1_rook_moved: bool,
    h1_rook_moved: bool,
    a8_rook_moved: bool,
    h8_rook_moved: bool,
    halfmoves: u8, // 50 move rule
    fullmoves: u32,
}

#[allow(unused)]
impl BoardState {
    fn new() -> Self {
        Self {
            active_color: Color::White,
            masks: [Mask(0); 12],

            last_move: None,
            a1_rook_moved: false,
            h1_rook_moved: false,
            a8_rook_moved: false,
            h8_rook_moved: false,
            halfmoves: 0,
            fullmoves: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let mut state = Self::new();

        // Get segments of FEN string
        let mut segments = fen.trim().split(' ');

        'pieces: {
            let Some(piece_string) = segments.next() else {
                return Err(FenError::MissingSection);
            };
            let rows = piece_string.split('/').rev();

            for (i, row) in rows.enumerate() {
                let mut chars = row.chars();
                let mut current_pos = 0;

                while current_pos < 8 {
                    if let Some(ch) = chars.next() {
                        match ch {
                            // Skip squares
                            '1'..='8' => {
                                // * Guaranteed to never panic due to above range check
                                let digit = ch.to_digit(9).unwrap();
                                current_pos += digit as usize;
                            }

                            // Add piece
                            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q'
                            | 'k' => {
                                // * Guaranteed to never panic due to above range check
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

        'active_color: {
            let Some(active_color) = segments.next() else {
                return Err(FenError::MissingSection);
            };
            state.active_color = match active_color {
                "w" => Color::White,
                "b" => Color::Black,
                _ => return Err(FenError::BadActiveColor),
            };
        }

        'castling_rights: {
            let Some(castling_rights) = segments.next() else {
                return Err(FenError::MissingSection);
            };

            if castling_rights == "-" {
                break 'castling_rights;
            }

            state.a1_rook_moved = true;
            state.h1_rook_moved = true;
            state.a8_rook_moved = true;
            state.h8_rook_moved = true;

            let mut prev: u8 = 0;

            for ch in castling_rights.chars() {
                match ch {
                'K' => {
                    if prev > 0 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    prev = 1;
                    state.a1_rook_moved = false;
                }
                'Q' => {
                    if prev > 1 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    prev = 2;
                    state.h1_rook_moved = false;
                }
                'k' => {
                    if prev > 2 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    prev = 3;
                    state.a8_rook_moved = false;
                }
                'q' => {
                    if prev > 3 {
                        panic!("Castling rights in FEN string ordered incorrectly");
                    }
                    state.h8_rook_moved = false;
                }
                _ => panic!("Third segment of FEN string should only contain the characters 'K', 'Q', 'k', 'q'"),
            }
            }
        }

        'en_passant: {
            state.last_move = None;

            let Some(next_segment) = segments.next() else {
                return Err(FenError::BadEnPassant);
            };

            if next_segment == "-" {
                break 'en_passant;
            }

            let Some(square) = Square::from_str(next_segment) else {
                return Err(FenError::BadEnPassant);
            };

            let Some(above_rank) = square.rank().plus(1) else {
                return Err(FenError::BadEnPassant);
            };
            let Some(below_rank) = square.rank().minus(1) else {
                return Err(FenError::BadEnPassant);
            };
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
                _ => return Err(FenError::BadEnPassant),
            };

            state.last_move = Some(mv);
        }

        'halfmoves: {
            let Some(halfmoves) = segments.next() else {
                return Err(FenError::MissingSection);
            };
            let Ok(halfmoves) = halfmoves.to_owned().parse::<u8>() else {
                return Err(FenError::BadHalfmoves);
            };

            if halfmoves > 100 {
                return Err(FenError::BadHalfmoves);
            }

            state.halfmoves = halfmoves;
        }

        'fullmoves: {
            let Some(fullmoves) = segments.next() else {
                return Err(FenError::MissingSection);
            };
            let Ok(fullmoves) = fullmoves.to_owned().parse::<u32>() else {
                return Err(FenError::BadFullmoves);
            };

            state.fullmoves = fullmoves;
        }

        if let Some(_extra_section) = segments.next() {
            return Err(FenError::TooManySections);
        }

        Ok(state)
    }

    /// Makes a move on the board, regardless of whether the move is legal or not.
    /// Despite its name, this function does still check if the move is possible to make or not.
    pub fn make_move_unchecked(&self, mv: Move) -> Result<BoardState, MoveError> {
        // Store copy of old board state
        let mut new_state = self.clone();
        let active_color = new_state.active_color;

        let Some(from_piece) = new_state.piece_at_square(mv.from) else {
            return Err(MoveError::MissingPiece);
        };

        let mut special_move = None;

        // Handle source-piece specific actions
        match from_piece {
            Piece::Pawn(_) => {
                // Check for promotion
                if mv.to.rank() == Rank::One || mv.to.rank() == Rank::Eight {
                    special_move = Some(SpecialMove::Promotion);
                }

                // Check for en passant
                if let Some(mask) = new_state.en_passant_mask() {
                    // Check if en passant mask equals move mask
                    if mask == mv.to.mask() {
                        special_move = Some(SpecialMove::EnPassant);
                    }
                }
            }
            Piece::King(_) => {
                // Check for castling by checking if king moved 2 squares horizontally
                if mv.file_diff() == 2 {
                    special_move = {
                        // Check if king moved to the right or not
                        let is_kingside = mv.to.file() > mv.from.file();

                        if is_kingside {
                            Some(SpecialMove::Castle(CastleDirection::Kingside))
                        } else {
                            Some(SpecialMove::Castle(CastleDirection::Queenside))
                        }
                    };
                }
            }
            Piece::Rook(_) => match mv.from {
                Square::A1 => new_state.a1_rook_moved = true,
                Square::A8 => new_state.a8_rook_moved = true,
                Square::H1 => new_state.h1_rook_moved = true,
                Square::H8 => new_state.h8_rook_moved = true,
                _ => (),
            },
            _ => (),
        }

        // Handle target-square specific actions

        // Update board state
        let mut is_capture = false;

        // Handle special moves/captures and normal captures separately
        if let Some(special_move) = special_move {
            match special_move {
                SpecialMove::EnPassant => {
                    is_capture = true;

                    let rank = mv.to.rank();
                    let file = mv.to.file();

                    let offset_rank = match active_color {
                        Color::White => rank.minus(1),
                        Color::Black => rank.plus(1),
                    }
                    .unwrap();

                    // Capture the pawn when en passant is played
                    let enemy_pawns = match active_color {
                        Color::White => &mut new_state.masks[Piece::BLACK_PAWN_INDEX],
                        Color::Black => &mut new_state.masks[Piece::WHITE_PAWN_INDEX],
                    };
                    *enemy_pawns &= !(Square::from_coords(offset_rank, file)).mask();
                }
                SpecialMove::Castle(direction) => {
                    let rook_mask = new_state.mask_mut(Piece::Rook(active_color));
                    let start_square: Square;
                    let end_square: Square;

                    match direction {
                        CastleDirection::Kingside => {
                            start_square = match active_color {
                                Color::White => Square::H1,
                                Color::Black => Square::H8,
                            };
                            end_square = match active_color {
                                Color::White => Square::F1,
                                Color::Black => Square::F8,
                            };
                        }
                        CastleDirection::Queenside => {
                            start_square = match active_color {
                                Color::White => Square::A1,
                                Color::Black => Square::A8,
                            };
                            end_square = match active_color {
                                Color::White => Square::D1,
                                Color::Black => Square::D8,
                            };
                        }
                    }

                    *rook_mask &= !start_square.mask();
                    *rook_mask |= end_square.mask();
                }
                SpecialMove::Promotion => {
                    // Only allow promotion to a queen for now
                    let promoted_piece_mask = new_state.mask_mut(Piece::Queen(active_color));
                    *promoted_piece_mask |= mv.to.mask();

                    let pawn_mask = new_state.mask_mut(from_piece);
                    *pawn_mask &= !mv.from.mask();
                }
            }
        } else if let Some(captured_piece) = new_state.piece_at_square(mv.to) {
            is_capture = true;

            if let Piece::Rook(_) = captured_piece {
                match mv.to {
                    // Castling rights
                    Square::A1 => new_state.a1_rook_moved = true,
                    Square::A8 => new_state.a8_rook_moved = true,
                    Square::H1 => new_state.h1_rook_moved = true,
                    Square::H8 => new_state.h8_rook_moved = true,
                    _ => (),
                }
            }

            // Remove captured piece
            let mask = new_state.mask_mut(captured_piece);
            *mask &= !mv.to.mask();
        }

        // Move piece
        // Movement is handled separately for promoting pawns
        if let Some(special_move) = special_move {
            if special_move != SpecialMove::Promotion {
                let from_mask = new_state.mask_mut(from_piece);
                *from_mask &= !mv.from.mask();
                *from_mask |= mv.to.mask();
            }
        } else {
            let from_mask = new_state.mask_mut(from_piece);
            *from_mask &= !mv.from.mask();
            *from_mask |= mv.to.mask();
        }

        // Update move counts
        if new_state.active_color == Color::Black {
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

        new_state.last_move = Some(mv);
        new_state.swap_active_color();

        Ok(new_state)
    }

    pub fn make_move(
        &self,
        mv: Move,
        sliding_moves: &SlidingMoves,
    ) -> Result<BoardState, MoveError> {
        // Make sure move is in pseudolegal move mask
        let possible_moves = self.get_pseudolegal_move_mask(mv.from, sliding_moves);
        if possible_moves == Mask(0) {
            println!("No legal moves from that square");
            return Err(MoveError::IllegalMove);
        }

        self.make_move_unchecked(mv)
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

    fn piece_at_square(&self, square: Square) -> Option<Piece> {
        for (i, mask) in self.masks.iter().enumerate() {
            if mask.0 & 1 << square as u8 > 0 {
                return Piece::from_mask_index(i);
            }
        }

        None
    }

    fn mask_mut(&mut self, piece: Piece) -> &mut Mask {
        &mut self.masks[piece.to_mask_index()]
    }

    fn mask(&self, piece: Piece) -> Mask {
        self.masks[piece.to_mask_index()]
    }

    fn swap_active_color(&mut self) {
        self.active_color = match self.active_color {
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
        let active_color = self.active_color;
        let Some(last_move) = self.last_move else {
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

        return active_color == Color::White && last_move.to.rank() == Rank::Five
            || active_color == Color::Black && last_move.to.rank() == Rank::Four;
    }

    pub fn en_passant_mask(&self) -> Option<Mask> {
        let active_color = self.active_color;
        let last_move = self.last_move?;

        let to_rank = last_move.to.rank();
        let from_rank = last_move.from.rank();

        if !self.possible_en_passant() {
            return None;
        }

        let capture_rank = match active_color {
            Color::White => to_rank.plus(1)?,
            Color::Black => to_rank.minus(1)?,
        };
        let capture_file = last_move.to.file();

        Some(Square::from_coords(capture_rank, capture_file).mask())
    }

    pub fn is_move_legal(&self, mv: Move, sliding_moves: &SlidingMoves) -> bool {
        // Prevent piece from moving to itself
        if mv.from == mv.to {
            return false;
        }

        // Play move temporarily (also ensures it can be played)
        let Ok(mut potential_state) = self.make_move_unchecked(mv) else {
            return false;
        };

        // Make sure move doesn't leave king in check
        let Some(king_square) = Square::from_mask(self.mask(Piece::King(self.active_color))) else {
            return false;
        };
        if potential_state.attacked_by(king_square, potential_state.active_color, sliding_moves) {
            return false;
        }

        true
    }

    fn move_masks(&self, piece: Piece) -> Vec<Mask> {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => Vec::from_iter(WHITE_PAWN_MOVE_MASKS),
                Color::Black => Vec::from_iter(BLACK_PAWN_MOVE_MASKS),
            },
            Piece::Knight(_) => Vec::from_iter(KNIGHT_MOVE_MASKS),
            Piece::Bishop(_) => Vec::from_iter(BISHOP_MOVE_MASKS),
            Piece::King(_) => Vec::from_iter(KING_MOVE_MASKS),
            Piece::Rook(_) => Vec::from_iter(ROOK_MOVE_MASKS),
            Piece::Queen(_) => Vec::from_iter([ROOK_MOVE_MASKS, BISHOP_MOVE_MASKS].concat()),
        }
    }

    pub fn can_castle(
        &self,
        color: Color,
        direction: CastleDirection,
        sliding_moves: &SlidingMoves,
    ) -> bool {
        const WHITE_BLOCKERS_SHORT: &[Square] = &[Square::F1, Square::G1];
        const WHITE_BLOCKERS_LONG: &[Square] = &[Square::D1, Square::C1, Square::B1];
        const BLACK_BLOCKERS_SHORT: &[Square] = &[Square::F8, Square::G8];
        const BLACK_BLOCKERS_LONG: &[Square] = &[Square::D8, Square::C8, Square::B8];

        let king_square = match color {
            Color::White => Square::E1,
            Color::Black => Square::E8,
        };

        let enemy_color = color.swapped();

        // Check if king in check
        if self.attacked_by(king_square, enemy_color, sliding_moves) {
            return false;
        }

        let relevant_blockers = match color {
            Color::White => match direction {
                CastleDirection::Kingside => WHITE_BLOCKERS_SHORT,
                CastleDirection::Queenside => WHITE_BLOCKERS_LONG,
            },
            Color::Black => match direction {
                CastleDirection::Kingside => BLACK_BLOCKERS_SHORT,
                CastleDirection::Queenside => BLACK_BLOCKERS_LONG,
            },
        };

        // Check for pieces in the way
        for blocker_square in relevant_blockers {
            if let Some(_) = self.piece_at_square(*blocker_square) {
                return false;
            }
        }

        // Check for checks in the way of the king's path
        for blocker_square in &relevant_blockers[..2] {
            if self.attacked_by(*blocker_square, enemy_color, sliding_moves) {
                return false;
            };
        }

        true
    }

    pub fn in_check(&self, color: Color, sliding_moves: &SlidingMoves) -> bool {
        let king_mask = self.mask(Piece::King(color));
        let king_square: Square = king_mask.ones()[0];

        self.attacked_by(king_square, color, sliding_moves)
    }

    pub fn attacked_by(&self, square: Square, color: Color, sliding_moves: &SlidingMoves) -> bool {
        let square_index = square as usize;

        let pawn_mask = self.mask(Piece::Pawn(color));
        let pawn_attacks = match color {
            Color::White => &WHITE_PAWN_CAPTURE_MASKS,
            Color::Black => &BLACK_PAWN_CAPTURE_MASKS,
        };
        if (pawn_attacks[square_index] & pawn_mask).0 > 0 {
            return true;
        }

        let knights = self.mask(Piece::Knight(color));
        if (knights & KNIGHT_MOVE_MASKS[square_index]).0 > 0 {
            return true;
        }

        let king = self.mask(Piece::King(color));
        if (king & KING_MOVE_MASKS[square_index]).0 > 0 {
            return true;
        }

        let rooks_queens = self.mask(Piece::Rook(color)) | self.mask(Piece::Queen(color));
        if (sliding_moves.get_rook_moves(square, self.all_pieces_mask()) & rooks_queens).0 > 0 {
            return true;
        }

        let bishops_queens = self.mask(Piece::Bishop(color)) | self.mask(Piece::Queen(color));
        if (sliding_moves.get_bishop_moves(square, self.all_pieces_mask()) & bishops_queens).0 > 0 {
            return true;
        }

        return false;
    }

    pub fn get_pseudolegal_move_mask(&self, square: Square, sliding_moves: &SlidingMoves) -> Mask {
        let blockers = self.all_pieces_mask();

        let Some(piece) = self.piece_at_square(square) else {
            return Mask(0);
        };
        let color = piece.color();

        // Prevent moving pieces of the wrong colour
        if color != self.active_color {
            return Mask(0);
        }

        let mask = self.mask(piece);

        let mut move_mask: Mask;

        if piece.is_slider() {
            move_mask = Mask(0);

            // Rook moves
            match piece {
                Piece::Rook(_) | Piece::Queen(_) => {
                    move_mask |= sliding_moves.get_rook_moves(square, blockers);
                }
                _ => (),
            }

            // Queen moves
            match piece {
                Piece::Bishop(_) | Piece::Queen(_) => {
                    move_mask |= sliding_moves.get_bishop_moves(square, blockers);
                }
                _ => (),
            }
        } else {
            // Grab move mask for the piece at the current square
            move_mask = self.move_masks(piece)[square.to_shift()];

            // Special moves
            match piece {
                Piece::Pawn(_) => {
                    // Prevent pawns double-hopping over pieces
                    if color == Color::White && square.rank() == Rank::Two
                        || color == Color::Black && square.rank() == Rank::Seven
                    {
                        let target_rank = match color {
                            Color::White => Rank::Three,
                            Color::Black => Rank::Six,
                        };

                        if let Some(_) =
                            self.piece_at_square(Square::from_coords(target_rank, square.file()))
                        {
                            move_mask.0 = 0;
                        }
                    }

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
                Piece::King(_) => {
                    // Kingside castling
                    if self.can_castle(color, CastleDirection::Kingside, sliding_moves) {
                        move_mask |= match color {
                            Color::White => Square::G1,
                            Color::Black => Square::G8,
                        }
                        .mask();
                    }

                    // Queenside castling
                    if self.can_castle(color, CastleDirection::Queenside, sliding_moves) {
                        move_mask |= match color {
                            Color::White => Square::C1,
                            Color::Black => Square::C8,
                        }
                        .mask();
                    }
                }
                _ => (),
            }
        }

        // Filter out moves that capture one's own pieces
        move_mask &= !self.friendly_pieces_mask(color);

        move_mask
    }

    pub fn get_pseudolegal_moves(&self, square: Square, sliding_moves: &SlidingMoves) -> Vec<Move> {
        let move_mask = self.get_pseudolegal_move_mask(square, sliding_moves);
        Move::from_move_mask(square, move_mask)
    }

    pub fn print_debugging_information(&self) {
        let sliding_moves = SlidingMoves::init();

        for i in 0..64 {
            let square = Square::from_u8(i).unwrap();
            println!("Pseudolegal moves from {}: ", square.to_string());
            self.get_pseudolegal_move_mask(square, &sliding_moves)
                .print();
            println!();
        }
    }
}

#[derive(Debug)]
pub struct BoardInitError;

#[derive(Debug)]
pub struct Board {
    // Board state and state history
    states: Vec<BoardState>,

    // Sliding piece magic bitboard helper struct
    sliding_moves: SlidingMoves,
}

impl Board {
    pub fn new(fen: &str) -> Result<Self, FenError> {
        let mut board = Board {
            states: Vec::new(),
            sliding_moves: SlidingMoves::init(),
        };

        board.load_from_fen(fen)?;

        Ok(board)
    }

    pub fn current_position(&self) -> &BoardState {
        self.states.last().unwrap()
    }

    pub fn load_from_fen(&mut self, fen: &str) -> Result<(), FenError> {
        let state = BoardState::from_fen(fen)?;

        self.states.clear();
        self.states.push(state);

        Ok(())
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        let state = self.current_position();

        for i in 0..64 {
            let square = Square::from_usize(i).unwrap();
            let pseudolegal_moves = state.get_pseudolegal_moves(square, &self.sliding_moves);

            for mv in pseudolegal_moves {
                if self.is_move_legal(mv) {
                    legal_moves.push(mv);
                }
            }
        }

        legal_moves
    }

    pub fn make_move_unchecked(&mut self, mv: Move) -> Result<(), MoveError> {
        let old_state = self.current_position();
        let new_state = old_state.make_move_unchecked(mv)?;
        self.states.push(new_state);
        Ok(())
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), MoveError> {
        let old_state = self.current_position();
        let new_state = old_state.make_move(mv, &self.sliding_moves)?;
        self.states.push(new_state);
        Ok(())
    }

    pub fn unmake_move(&mut self) -> Result<(), MoveError> {
        if self.states.len() > 1 {
            self.states.pop();
            Ok(())
        } else {
            Err(MoveError::NoPreviousMoves)
        }
    }

    pub fn is_move_legal(&self, mv: Move) -> bool {
        self.current_position()
            .is_move_legal(mv, &self.sliding_moves)
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn castling() {
        let mut board = Board::new(START_FEN).unwrap();

        // Setup pieces to castle (don't have FEN for this yet)
        let _ = board.make_move_unchecked(Move::from_long_algebraic("d2d4").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("g8f6").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("c1f4").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("e7e6").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("b1c3").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("f8e7").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("d1d2").unwrap());

        // Castle black king kingside and white king queenside
        let _ = board.make_move_unchecked(Move::from_long_algebraic("e8g8").unwrap());
        let _ = board.make_move_unchecked(Move::from_long_algebraic("e1c1").unwrap());

        assert_eq!(
            board.states[0].all_pieces_mask(),
            Mask(18446462598732906495), // Pregenerated mask w/ correct piece layout
        );
    }

    #[test]
    fn en_passant_is_legal() {
        let mut board = Board::new(START_FEN).unwrap();

        board
            .make_move_unchecked(Move {
                from: Square::E2,
                to: Square::E5,
            })
            .unwrap();

        board
            .make_move_unchecked(Move {
                from: Square::D7,
                to: Square::D5,
            })
            .unwrap();

        assert!(board.is_move_legal(Move {
            from: Square::E5,
            to: Square::D6,
        }));
    }

    #[test]
    fn en_passant_mask() {
        let mut board = Board::new(START_FEN).unwrap();

        let _ = board.make_move_unchecked(Move {
            from: Square::E2,
            to: Square::E4,
        });

        assert_eq!(
            board.current_position().en_passant_mask(),
            Some(Square::E3.mask())
        );

        let _ = board.make_move_unchecked(Move {
            from: Square::E7,
            to: Square::E5,
        });

        assert_eq!(
            board.current_position().en_passant_mask(),
            Some(Square::E6.mask())
        );

        let _ = board.make_move_unchecked(Move {
            from: Square::G1,
            to: Square::F3,
        });

        assert_eq!(board.current_position().en_passant_mask(), None);
    }

    #[test]
    fn castling_rights() {
        const TEST_POS_FEN: &str = "rnbqk2r/ppppbppp/4pn2/8/3P1B2/2N5/PPPQPPPP/R3KBNR b KQkq - 3 4";

        let mut board = Board::new(TEST_POS_FEN).unwrap();

        assert!(board.is_move_legal(Move::from_long_algebraic("e8g8").unwrap()));

        board
            .make_move_unchecked(Move::from_long_algebraic("e8g8").unwrap())
            .unwrap();

        assert!(board.is_move_legal(Move::from_long_algebraic("e1c1").unwrap()));
    }

    #[test]
    fn excessive_moves() {
        const TEST_POS_FEN: &str = "rnbqk2r/ppppbppp/4pn2/8/3P1B2/2N5/PPPQPPPP/R3KBNR b KQkq - 3 4";
        let mut board = Board::new(TEST_POS_FEN).unwrap();

        board.current_position().print_debugging_information();
    }
}
