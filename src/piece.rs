use crate::board_helper::{BoardHelper, BISHOP_MOVE_OFFSETS, ROOK_MOVE_OFFSETS};
use crate::mask::Mask;

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

pub enum Direction {
    Orthogonal,
    Diagonal,
}

// pub fn generate_rook_move_masks() -> [Mask; 64] {
//     let mut masks = [0; 64];

//     for start in 0..64 {
//         for offset in ROOK_MOVE_OFFSETS {
//             let mut target = start as i8 + offset;
//             let mut prev_rank = Self::rank(start as usize);
//             let mut prev_file = Self::file(start as usize);

//             while target >= 0 && target < 64 {
//                 if Self::rank_difference(prev_rank, target as usize) > 1
//                     || Self::file_difference(prev_file, target as usize) > 1
//                 {
//                     break;
//                 }

//                 prev_rank = Self::rank(target as usize);
//                 prev_file = Self::file(target as usize);

//                 masks[start] |= 1 << target;
//                 target += offset;
//             }
//         }
//     }

//     masks.map(|val| Mask(val))
// }

impl Direction {
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
}

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn test_blockers() {
        let ortho = Direction::Orthogonal;

        for (i, blocker) in ortho.blockers().iter().enumerate() {
            println!("blocker {}:", i);
            BoardHelper::print_mask(blocker);
            println!("");
        }
    }
}
