use crate::board_helper::BoardHelper;
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

    pub fn relevant_blockers(&self) -> Vec<Vec<Mask>> {
        self.blockers().into_iter().map(|m| m.submasks()).collect()
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
        for blocker_list in Direction::Orthogonal.relevant_blockers() {
            BoardHelper::print_mask(&blocker_list[0]);
            println!();
        }
    }
}
