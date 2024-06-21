use crate::mask::Mask;
use crate::move_gen::move_gen::{
    file, file_difference, generate_bishop_move_masks, generate_rook_move_masks, rank,
    rank_difference,
};
use crate::square::Square;

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
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Orthogonal,
    Diagonal,
}

impl Direction {
    /// Generate a list of masks for each square representing all of its possible blockers.
    ///
    /// The mask does not include the edges of the board, unless the square itself is on an edge, in which case the edge is included.
    pub fn all_blockers(&self) -> Vec<Mask> {
        const TOP_EDGE_MASK: u64 =
            0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        const BOTTOM_EDGE_MASK: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
        const LEFT_EDGE_MASK: u64 =
            0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        const RIGHT_EDGE_MASK: u64 =
            0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

        let mut blockers = match self {
            Self::Orthogonal => generate_rook_move_masks(),
            Self::Diagonal => generate_bishop_move_masks(),
        };

        for (i, blocker) in blockers.iter_mut().enumerate() {
            let rank = rank(i);
            let file = file(i);

            let mut exclusion_mask = 0;
            if rank != 0 {
                exclusion_mask |= BOTTOM_EDGE_MASK;
            }
            if rank != 7 {
                exclusion_mask |= TOP_EDGE_MASK;
            }
            if file != 0 {
                exclusion_mask |= LEFT_EDGE_MASK;
            }
            if file != 7 {
                exclusion_mask |= RIGHT_EDGE_MASK;
            }

            blocker.0 &= !exclusion_mask;
        }

        blockers.into_iter().collect()
    }

    /// Returns a vector of all possible blocker combinations for each square.
    pub fn all_blocker_subsets(&self) -> Vec<Vec<Mask>> {
        self.all_blockers()
            .into_iter()
            .map(|m| m.subsets())
            .collect()
    }

    /// Returns a singular mask containing all possible squares that a piece can move to from another square.
    ///
    /// **DO NOT** use this function to generate moves at runtime. This function should **ONLY** be used to bootstrap the much faster magic bitboard approach to move gen.
    pub fn moves_for(&self, square: Square, blockers: Mask) -> Mask {
        let blockers = self.all_blockers()[square as usize] & blockers;

        let offsets = match self {
            Direction::Orthogonal => ROOK_MOVE_OFFSETS,
            Direction::Diagonal => BISHOP_MOVE_OFFSETS,
        };

        let mut movemask = 0;

        for offset in offsets {
            let mut target = square as i8 + offset;
            let (mut prev_rank, mut prev_file) = (rank(square as usize), file(square as usize));

            while target >= 0 && target < 64 {
                // Prevent wrapping around edges
                if rank_difference(prev_rank, target as usize) > 1
                    || file_difference(prev_file, target as usize) > 1
                {
                    break;
                }
                prev_rank = rank(target as usize);
                prev_file = file(target as usize);

                // Add move to mask
                movemask |= 1 << target;

                // Check for piece in the way (should still be included in mask)
                if blockers.0 & (1 << target) != 0 {
                    break;
                }

                target += offset;
            }
        }

        Mask(movemask)
    }
}

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn debug_blockers() {
        let ortho = Direction::Orthogonal;

        for (i, blocker) in ortho.all_blockers().iter().enumerate() {
            println!("blocker {}:", i);
            blocker.print();
            println!("");
        }
    }

    #[test]
    fn debug_relevant_blockers() {
        for blocker_list in Direction::Orthogonal.all_blocker_subsets() {
            blocker_list[0].print();
            println!();
        }
    }

    #[test]
    fn debug_move_finding() {
        let blockers =
            Square::E6.mask() | Square::C4.mask() | Square::G4.mask() | Square::A8.mask();
        let rook_moves_a1 = Direction::Orthogonal.moves_for(Square::E4, blockers);
        rook_moves_a1.print();
    }
}
