use crate::piece::{Color, Piece};

pub struct BoardHelper;

impl BoardHelper {
    pub fn char_to_piece(ch: char) -> Option<Piece> {
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

    pub fn rank(i: usize) -> usize {
        i / 8
    }

    pub fn file(i: usize) -> usize {
        i % 8
    }

    pub fn rank_difference(rank: usize, tile: usize) -> usize {
        rank.abs_diff(tile / 8)
    }

    pub fn file_difference(file: usize, tile: usize) -> usize {
        file.abs_diff(tile % 8)
    }

    pub fn piece_to_bitboard_index(piece: char) -> usize {
        match piece {
            // White
            'P' => 0,
            'N' => 1,
            'B' => 2,
            'R' => 3,
            'Q' => 4,
            'K' => 5,

            // Black
            'p' => 6,
            'n' => 7,
            'b' => 8,
            'r' => 9,
            'q' => 10,
            'k' => 11,

            _ => panic!("invalid piece char '{piece}'"),
        }
    }
}
