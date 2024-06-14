use crate::board_helper::BoardHelper;
use crate::mask::Mask;
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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Orthogonal,
    Diagonal,
}

impl Direction {
    /// Generate a list of masks for each square representing all of its possible blockers.
    ///
    /// The mask does not include the edges of the board, unless the square itself is on an edge, in which case the edge is included.
    pub fn blockers(&self) -> [Mask; 64] {
        const TOP_EDGE_MASK: u64 =
            0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        const BOTTOM_EDGE_MASK: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
        const LEFT_EDGE_MASK: u64 =
            0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        const RIGHT_EDGE_MASK: u64 =
            0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

        let mut blockers = match self {
            Self::Orthogonal => BoardHelper::generate_rook_move_masks(),
            Self::Diagonal => BoardHelper::generate_bishop_move_masks(),
        };

        for (i, blocker) in blockers.iter_mut().enumerate() {
            let rank = BoardHelper::rank(i);
            let file = BoardHelper::file(i);

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

        blockers
    }

    /// Returns a vector of all possible blocker combinations for each square.
    pub fn blocker_subsets(&self) -> [Vec<Mask>; 64] {
        self.blockers().map(|m| m.submasks())
    }

    /// Returns a singular mask containing all possible squares that a piece can move to from another square.
    ///
    /// **DO NOT** use this function to generate moves at runtime. This function should **ONLY** be used to bootstrap the much faster magic bitboard approach to move gen.
    pub fn moves(&self, square: Square, blockers: Mask) -> Mask {
        //! ---- WIP ----
        let blockerlist = self.blockers();
        let blockers = self.blockers()[square as usize] & blockers;

        let offsets = match self {
            Direction::Orthogonal => ROOK_MOVE_OFFSETS,
            Direction::Diagonal => BISHOP_MOVE_OFFSETS,
        };

        let mut movemask = 0;

        for offset in offsets {
            let mut target = square as i8 + offset;
            let (mut prev_rank, mut prev_file) = (
                BoardHelper::rank(square as usize),
                BoardHelper::file(square as usize),
            );

            while target >= 0 && target < 64 {
                // Prevent wrapping around edges
                if BoardHelper::rank_difference(prev_rank, target as usize) > 1
                    || BoardHelper::file_difference(prev_file, target as usize) > 1
                {
                    break;
                }
                prev_rank = BoardHelper::rank(target as usize);
                prev_file = BoardHelper::file(target as usize);

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

        for (i, blocker) in ortho.blockers().iter().enumerate() {
            println!("blocker {}:", i);
            BoardHelper::print_mask(blocker);
            println!("");
        }
    }

    #[test]
    fn debug_relevant_blockers() {
        for blocker_list in Direction::Orthogonal.blocker_subsets() {
            BoardHelper::print_mask(&blocker_list[0]);
            println!();
        }
    }

    #[test]
    fn debug_move_finding() {
        let blockers =
            Square::E6.mask() | Square::C4.mask() | Square::G4.mask() | Square::A8.mask();
        let rook_moves_a1 = Direction::Orthogonal.moves(Square::E4, blockers);
        BoardHelper::print_mask(&rook_moves_a1);
    }
}
