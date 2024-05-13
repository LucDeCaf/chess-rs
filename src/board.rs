use crate::board_helper::BoardHelper;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Self::Pawn(color)
            | Self::Knight(color)
            | Self::Bishop(color)
            | Self::Rook(color)
            | Self::Queen(color)
            | Self::King(color) => *color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveError;

#[derive(Debug)]
pub struct Bitboard {
    mask: u64,
    piece: Piece,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub to: Square,
    pub from: Square,
}

impl Move {
    pub fn from_long_algebraic(input: &str) -> Option<Self> {
        Some(Self {
            from: Square::from_str(&input[..2])?,
            to: Square::from_str(&input[2..])?,
        })
    }
}

#[derive(Debug)]
pub struct Board {
    // Game data
    pub is_white_turn: bool,

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
            is_white_turn: true,

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
        self.is_white_turn = match segments.next().expect("FEN string should have 6 segments") {
            "w" => true,
            "b" => false,
            _ => panic!("Second section of FEN string should be either 'w' or 'b'."),
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

        self.is_white_turn = !self.is_white_turn;
        Ok(())
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        let reverse_move = Move {
            from: mv.to,
            to: mv.from,
        };

        self.is_white_turn = !self.is_white_turn;
        self.make_move(&reverse_move).unwrap();
        self.is_white_turn = !self.is_white_turn;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::A1),
            1 => Some(Self::B1),
            2 => Some(Self::C1),
            3 => Some(Self::D1),
            4 => Some(Self::E1),
            5 => Some(Self::F1),
            6 => Some(Self::G1),
            7 => Some(Self::H1),
            8 => Some(Self::A2),
            9 => Some(Self::B2),
            10 => Some(Self::C2),
            11 => Some(Self::D2),
            12 => Some(Self::E2),
            13 => Some(Self::F2),
            14 => Some(Self::G2),
            15 => Some(Self::H2),
            16 => Some(Self::A3),
            17 => Some(Self::B3),
            18 => Some(Self::C3),
            19 => Some(Self::D3),
            20 => Some(Self::E3),
            21 => Some(Self::F3),
            22 => Some(Self::G3),
            23 => Some(Self::H3),
            24 => Some(Self::A4),
            25 => Some(Self::B4),
            26 => Some(Self::C4),
            27 => Some(Self::D4),
            28 => Some(Self::E4),
            29 => Some(Self::F4),
            30 => Some(Self::G4),
            31 => Some(Self::H4),
            32 => Some(Self::A5),
            33 => Some(Self::B5),
            34 => Some(Self::C5),
            35 => Some(Self::D5),
            36 => Some(Self::E5),
            37 => Some(Self::F5),
            38 => Some(Self::G5),
            39 => Some(Self::H5),
            40 => Some(Self::A6),
            41 => Some(Self::B6),
            42 => Some(Self::C6),
            43 => Some(Self::D6),
            44 => Some(Self::E6),
            45 => Some(Self::F6),
            46 => Some(Self::G6),
            47 => Some(Self::H6),
            48 => Some(Self::A7),
            49 => Some(Self::B7),
            50 => Some(Self::C7),
            51 => Some(Self::D7),
            52 => Some(Self::E7),
            53 => Some(Self::F7),
            54 => Some(Self::G7),
            55 => Some(Self::H7),
            56 => Some(Self::A8),
            57 => Some(Self::B8),
            58 => Some(Self::C8),
            59 => Some(Self::D8),
            60 => Some(Self::E8),
            61 => Some(Self::F8),
            62 => Some(Self::G8),
            63 => Some(Self::H8),
            _ => None,
        }
    }

    pub fn from_u16(val: u16) -> Option<Self> {
        if val > u8::MAX as u16 {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        if val > u8::MAX as u32 {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_u64(val: u64) -> Option<Self> {
        if val > u8::MAX as u64 {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_usize(val: usize) -> Option<Self> {
        if val > u8::MAX as usize {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "a1" => Some(Square::A1),
            "b1" => Some(Square::B1),
            "c1" => Some(Square::C1),
            "d1" => Some(Square::D1),
            "e1" => Some(Square::E1),
            "f1" => Some(Square::F1),
            "g1" => Some(Square::G1),
            "h1" => Some(Square::H1),
            "a2" => Some(Square::A2),
            "b2" => Some(Square::B2),
            "c2" => Some(Square::C2),
            "d2" => Some(Square::D2),
            "e2" => Some(Square::E2),
            "f2" => Some(Square::F2),
            "g2" => Some(Square::G2),
            "h2" => Some(Square::H2),
            "a3" => Some(Square::A3),
            "b3" => Some(Square::B3),
            "c3" => Some(Square::C3),
            "d3" => Some(Square::D3),
            "e3" => Some(Square::E3),
            "f3" => Some(Square::F3),
            "g3" => Some(Square::G3),
            "h3" => Some(Square::H3),
            "a4" => Some(Square::A4),
            "b4" => Some(Square::B4),
            "c4" => Some(Square::C4),
            "d4" => Some(Square::D4),
            "e4" => Some(Square::E4),
            "f4" => Some(Square::F4),
            "g4" => Some(Square::G4),
            "h4" => Some(Square::H4),
            "a5" => Some(Square::A5),
            "b5" => Some(Square::B5),
            "c5" => Some(Square::C5),
            "d5" => Some(Square::D5),
            "e5" => Some(Square::E5),
            "f5" => Some(Square::F5),
            "g5" => Some(Square::G5),
            "h5" => Some(Square::H5),
            "a6" => Some(Square::A6),
            "b6" => Some(Square::B6),
            "c6" => Some(Square::C6),
            "d6" => Some(Square::D6),
            "e6" => Some(Square::E6),
            "f6" => Some(Square::F6),
            "g6" => Some(Square::G6),
            "h6" => Some(Square::H6),
            "a7" => Some(Square::A7),
            "b7" => Some(Square::B7),
            "c7" => Some(Square::C7),
            "d7" => Some(Square::D7),
            "e7" => Some(Square::E7),
            "f7" => Some(Square::F7),
            "g7" => Some(Square::G7),
            "h7" => Some(Square::H7),
            "a8" => Some(Square::A8),
            "b8" => Some(Square::B8),
            "c8" => Some(Square::C8),
            "d8" => Some(Square::D8),
            "e8" => Some(Square::E8),
            "f8" => Some(Square::F8),
            "g8" => Some(Square::G8),
            "h8" => Some(Square::H8),
            _ => None,
        }
    }

    pub fn to_shift(&self) -> u8 {
        match self {
            Self::A1 => 0,
            Self::B1 => 1,
            Self::C1 => 2,
            Self::D1 => 3,
            Self::E1 => 4,
            Self::F1 => 5,
            Self::G1 => 6,
            Self::H1 => 7,
            Self::A2 => 8,
            Self::B2 => 9,
            Self::C2 => 10,
            Self::D2 => 11,
            Self::E2 => 12,
            Self::F2 => 13,
            Self::G2 => 14,
            Self::H2 => 15,
            Self::A3 => 16,
            Self::B3 => 17,
            Self::C3 => 18,
            Self::D3 => 19,
            Self::E3 => 20,
            Self::F3 => 21,
            Self::G3 => 22,
            Self::H3 => 23,
            Self::A4 => 24,
            Self::B4 => 25,
            Self::C4 => 26,
            Self::D4 => 27,
            Self::E4 => 28,
            Self::F4 => 29,
            Self::G4 => 30,
            Self::H4 => 31,
            Self::A5 => 32,
            Self::B5 => 33,
            Self::C5 => 34,
            Self::D5 => 35,
            Self::E5 => 36,
            Self::F5 => 37,
            Self::G5 => 38,
            Self::H5 => 39,
            Self::A6 => 40,
            Self::B6 => 41,
            Self::C6 => 42,
            Self::D6 => 43,
            Self::E6 => 44,
            Self::F6 => 45,
            Self::G6 => 46,
            Self::H6 => 47,
            Self::A7 => 48,
            Self::B7 => 49,
            Self::C7 => 50,
            Self::D7 => 51,
            Self::E7 => 52,
            Self::F7 => 53,
            Self::G7 => 54,
            Self::H7 => 55,
            Self::A8 => 56,
            Self::B8 => 57,
            Self::C8 => 58,
            Self::D8 => 59,
            Self::E8 => 60,
            Self::F8 => 61,
            Self::G8 => 62,
            Self::H8 => 63,
        }
    }
}
