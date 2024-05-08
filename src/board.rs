use crate::board_helper::BoardHelper;
use std::ops::RangeInclusive;

const WHITE_PIECE_MASK_INDEXES: RangeInclusive<usize> = 0..=5;
const BLACK_PIECE_MASK_INDEXES: RangeInclusive<usize> = 6..=11;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Debug)]
pub struct Board {
    // Game data
    pub bitboards: Vec<u64>,
    pub is_white_turn: bool,

    // Piece move tables (to be optimised)
    pub white_pawn_move_masks: [u64; 64],
    pub black_pawn_move_masks: [u64; 64],
    pub white_pawn_capture_masks: [u64; 64],
    pub black_pawn_capture_masks: [u64; 64],
    pub knight_masks: [u64; 64],
    pub bishop_masks: [u64; 64],
    pub rook_masks: [u64; 64],
    pub queen_masks: [u64; 64], // queen_masks[i] == rook_masks[i] | bishop_masks[i]
    pub king_masks: [u64; 64],
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    source: Square,
    target: Square,
}

impl Board {
    pub fn new(fen: &str) -> Self {
        // Use rook and bishop masks to generate queen masks
        let bishop_masks = BoardHelper::generate_bishop_masks();
        let rook_masks = BoardHelper::generate_rook_masks();

        let mut i = 0;
        let queen_masks = bishop_masks.map(|bishop_mask| {
            let queen_mask = bishop_mask | rook_masks[i];
            i += 1;
            queen_mask
        });

        let mut board = Board {
            bitboards: vec![
                0, 0, 0, 0, 0, 0, // White pieces
                0, 0, 0, 0, 0, 0, // Black pieces
            ],
            is_white_turn: true,

            white_pawn_move_masks: BoardHelper::generate_white_pawn_masks(),
            black_pawn_move_masks: BoardHelper::generate_black_pawn_masks(),
            white_pawn_capture_masks: BoardHelper::generate_white_pawn_capture_masks(),
            black_pawn_capture_masks: BoardHelper::generate_black_pawn_capture_masks(),
            knight_masks: BoardHelper::generate_knight_masks(),
            bishop_masks,
            rook_masks,
            queen_masks,
            king_masks: [0; 64],
        };
        board.load_from_fen(fen);
        board
    }

    pub fn load_from_fen(&mut self, fen: &str) {
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

    pub fn make_move(&mut self, mv: &Move) {
        let (start_mask_indices, target_mask_indices) = match self.is_white_turn {
            true => (WHITE_PIECE_MASK_INDEXES, BLACK_PIECE_MASK_INDEXES),
            false => (BLACK_PIECE_MASK_INDEXES, WHITE_PIECE_MASK_INDEXES),
        };

        // Start with 1 all the way on the left, then adjust from there
        let start_mask = 1 << 63 >> mv.source as u64;
        let target_mask = 1 << 63 >> mv.target as u64;

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

    pub fn unmake_move(&mut self, mv: &Move) {
        let reverse_move = Move {
            source: mv.target,
            target: mv.source,
        };

        self.is_white_turn = !self.is_white_turn;
        self.make_move(&reverse_move);
        self.is_white_turn = !self.is_white_turn;
    }

    pub fn pieces_mask(&self) -> u64 {
        let mut board_mask: u64 = 0;

        for bitboard in &self.bitboards {
            board_mask |= bitboard;
        }

        board_mask
    }
}

#[derive(Debug, Clone, Copy)]
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
}
