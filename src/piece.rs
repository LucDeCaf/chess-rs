pub const KNIGHT_MOVE_OFFSETS: [i8; 8] = [15, 17, 6, 10, -10, -6, -17, -15];
pub const BISHOP_MOVE_OFFSETS: [i8; 4] = [7, 9, -7, -9];
pub const ROOK_MOVE_OFFSETS: [i8; 4] = [8, 1, -8, -1];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub const WHITE_PAWN_INDEX: usize = 0;
    pub const WHITE_KNIGHT_INDEX: usize = 1;
    pub const WHITE_BISHOP_INDEX: usize = 2;
    pub const WHITE_ROOK_INDEX: usize = 3;
    pub const WHITE_QUEEN_INDEX: usize = 4;
    pub const WHITE_KING_INDEX: usize = 5;
    pub const BLACK_PAWN_INDEX: usize = 6;
    pub const BLACK_KNIGHT_INDEX: usize = 7;
    pub const BLACK_BISHOP_INDEX: usize = 8;
    pub const BLACK_ROOK_INDEX: usize = 9;
    pub const BLACK_QUEEN_INDEX: usize = 10;
    pub const BLACK_KING_INDEX: usize = 11;

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

    pub fn is_slider(&self) -> bool {
        match self {
            Piece::Queen(_) | Piece::Rook(_) | Piece::Bishop(_) => true,
            Piece::King(_) | Piece::Knight(_) | Piece::Pawn(_) => false,
        }
    }

    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'P' => Some(Self::Pawn(Color::White)),
            'N' => Some(Self::Knight(Color::White)),
            'B' => Some(Self::Bishop(Color::White)),
            'R' => Some(Self::Rook(Color::White)),
            'Q' => Some(Self::Queen(Color::White)),
            'K' => Some(Self::King(Color::White)),
            'p' => Some(Self::Pawn(Color::Black)),
            'n' => Some(Self::Knight(Color::Black)),
            'b' => Some(Self::Bishop(Color::Black)),
            'r' => Some(Self::Rook(Color::Black)),
            'q' => Some(Self::Queen(Color::Black)),
            'k' => Some(Self::King(Color::Black)),
            _ => None,
        }
    }

    pub fn to_mask_index(&self) -> usize {
        match self {
            Piece::Pawn(Color::White) => Piece::WHITE_PAWN_INDEX,
            Piece::Knight(Color::White) => Piece::WHITE_KNIGHT_INDEX,
            Piece::Bishop(Color::White) => Piece::WHITE_BISHOP_INDEX,
            Piece::Rook(Color::White) => Piece::WHITE_ROOK_INDEX,
            Piece::Queen(Color::White) => Piece::WHITE_QUEEN_INDEX,
            Piece::King(Color::White) => Piece::WHITE_KING_INDEX,
            Piece::Pawn(Color::Black) => Piece::BLACK_PAWN_INDEX,
            Piece::Knight(Color::Black) => Piece::BLACK_KNIGHT_INDEX,
            Piece::Bishop(Color::Black) => Piece::BLACK_BISHOP_INDEX,
            Piece::Rook(Color::Black) => Piece::BLACK_ROOK_INDEX,
            Piece::Queen(Color::Black) => Piece::BLACK_QUEEN_INDEX,
            Piece::King(Color::Black) => Piece::BLACK_KING_INDEX,
        }
    }

    pub fn from_mask_index(i: usize) -> Option<Piece> {
        match i {
            Piece::WHITE_PAWN_INDEX => Some(Piece::Pawn(Color::White)),
            Piece::WHITE_KNIGHT_INDEX => Some(Piece::Knight(Color::White)),
            Piece::WHITE_BISHOP_INDEX => Some(Piece::Bishop(Color::White)),
            Piece::WHITE_ROOK_INDEX => Some(Piece::Rook(Color::White)),
            Piece::WHITE_QUEEN_INDEX => Some(Piece::Queen(Color::White)),
            Piece::WHITE_KING_INDEX => Some(Piece::King(Color::White)),
            Piece::BLACK_PAWN_INDEX => Some(Piece::Pawn(Color::Black)),
            Piece::BLACK_KNIGHT_INDEX => Some(Piece::Knight(Color::Black)),
            Piece::BLACK_BISHOP_INDEX => Some(Piece::Bishop(Color::Black)),
            Piece::BLACK_ROOK_INDEX => Some(Piece::Rook(Color::Black)),
            Piece::BLACK_QUEEN_INDEX => Some(Piece::Queen(Color::Black)),
            Piece::BLACK_KING_INDEX => Some(Piece::King(Color::Black)),
            _ => None,
        }
    }
}
