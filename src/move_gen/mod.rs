pub mod magics;
pub mod move_masks;

pub mod direction {
    use crate::{
        board::{
            mask::Mask,
            piece::{BISHOP_MOVE_OFFSETS, ROOK_MOVE_OFFSETS},
            square::Square,
        },
        move_gen::{
            file, file_difference, generate_bishop_move_masks, generate_rook_move_masks, rank,
            rank_difference,
        },
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

use direction::Direction;

use crate::board::{
    mask::Mask,
    piece::{BISHOP_MOVE_OFFSETS, KNIGHT_MOVE_OFFSETS, ROOK_MOVE_OFFSETS},
    square::Square,
};

use magics::{BISHOP_MAGICS, ROOK_MAGICS};

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

pub fn generate_white_pawn_move_masks() -> [Mask; 64] {
    let mut masks = [0; 64];

    for i in 8..56 {
        masks[i] |= 1 << (i + 8);

        if i < 16 {
            masks[i] |= 1 << (i + 16);
        }
    }

    masks.map(|val| Mask(val))
}

pub fn generate_black_pawn_move_masks() -> [Mask; 64] {
    let mut masks = [0; 64];

    for i in 8..56 {
        masks[i] |= 1 << (i - 8);

        if i >= 48 {
            masks[i] |= 1 << (i - 16);
        }
    }

    masks.map(|val| Mask(val))
}

pub fn generate_white_pawn_capture_masks() -> [Mask; 64] {
    let mut masks = [0; 64];
    let mut mask: usize;

    for i in 8..56 {
        mask = 0;

        let rank = rank(i);
        let file = file(i);

        if rank_difference(rank, i + 9) == 1 && file_difference(file, i + 9) == 1 {
            mask |= 1 << (i + 9);
        }

        if rank_difference(rank, i + 7) == 1 && file_difference(file, i + 7) == 1 {
            mask |= 1 << (i + 7);
        }

        masks[i] = mask as u64;
    }

    masks.map(|val| Mask(val))
}

pub fn generate_black_pawn_capture_masks() -> [Mask; 64] {
    let mut masks = [0; 64];
    let mut mask: usize;

    for i in 8..56 {
        mask = 0;

        let rank = rank(i);
        let file = file(i);

        if i >= 9 && rank_difference(rank, i - 9) == 1 && file_difference(file, i - 9) == 1 {
            mask |= (1 << i) >> 9;
        }

        if i >= 7 && rank_difference(rank, i - 7) == 1 && file_difference(file, i - 7) == 1 {
            mask |= (1 << i) >> 7;
        }

        masks[i] = mask as u64;
    }

    masks.map(|val| Mask(val))
}

pub fn generate_rook_move_masks() -> [Mask; 64] {
    let mut masks = [0; 64];

    for start in 0..64 {
        for offset in ROOK_MOVE_OFFSETS {
            let mut target = start as i8 + offset;
            let mut prev_rank = rank(start);
            let mut prev_file = file(start);

            while target >= 0 && target < 64 {
                // If moving by offset wraps you around the board then stop
                if rank_difference(prev_rank, target as usize) > 1
                    || file_difference(prev_file, target as usize) > 1
                {
                    break;
                }

                prev_rank = rank(target as usize);
                prev_file = file(target as usize);

                masks[start] |= 1 << target;
                target += offset;
            }
        }
    }

    masks.map(|val| Mask(val))
}

pub fn generate_bishop_move_masks() -> [Mask; 64] {
    let mut masks = [0; 64];

    for start in 0..64 {
        for offset in BISHOP_MOVE_OFFSETS {
            let mut target = start as i8 + offset;
            let mut prev_rank = rank(start as usize);
            let mut prev_file = file(start as usize);

            while target >= 0 && target < 64 {
                if rank_difference(prev_rank, target as usize) > 1
                    || file_difference(prev_file, target as usize) > 1
                {
                    break;
                }

                prev_rank = rank(target as usize);
                prev_file = file(target as usize);

                masks[start] |= 1 << target;
                target += offset;
            }
        }
    }

    masks.map(|val| Mask(val))
}

pub fn generate_knight_move_masks() -> [Mask; 64] {
    let mut masks = [0; 64];

    for start in 0..64 {
        let rank = rank(start);
        let file = file(start);

        for offset in KNIGHT_MOVE_OFFSETS {
            let target = start as i8 + offset;

            if target < 0 || target > 63 {
                continue;
            }

            let target = target as usize;

            let rank_diff = rank_difference(rank, target);
            let file_diff = file_difference(file, target);

            if !(rank_diff == 1 && file_diff == 2 || rank_diff == 2 && file_diff == 1) {
                continue;
            }

            masks[start] |= 1 << target;
        }
    }

    masks.map(|val| Mask(val))
}

pub fn generate_king_move_masks() -> [Mask; 64] {
    let mut masks = [0; 64];

    for start in 0..64 {
        let rank = rank(start);
        let file = file(start);

        for offset_set in [ROOK_MOVE_OFFSETS, BISHOP_MOVE_OFFSETS] {
            for offset in offset_set {
                let target = start as i8 + offset;

                if target < 0 || target > 63 {
                    continue;
                }

                let target = target as usize;

                if rank_difference(rank, target) > 1 || file_difference(file, target) > 1 {
                    continue;
                }

                masks[start] |= 1 << target;
            }
        }
    }

    masks.map(|val| Mask(val))
}

pub fn create_move_list(direction: Direction, magics: &[MagicEntry; 64]) -> Vec<Vec<Mask>> {
    let mut moves = Vec::with_capacity(64);

    for (i, magic) in magics.into_iter().enumerate() {
        let move_table =
            try_fill_magic_table(direction, magic, Square::from_usize(i).unwrap()).unwrap();
        moves.push(move_table);
    }

    if direction == Direction::Diagonal {
        magics[0].mask.print();
    }

    moves
}

#[derive(Debug)]
pub struct TableFillError;

pub fn try_fill_magic_table(
    direction: Direction,
    entry: &MagicEntry,
    square: Square,
) -> Result<Vec<Mask>, TableFillError> {
    let mut table = vec![Mask(0); 1 << entry.index_bits];

    for blockers in entry.mask.subsets() {
        let moves = direction.moves_for(square, blockers);
        let new_entry = &mut table[entry.index(blockers)];

        if new_entry.0 == 0 {
            *new_entry = moves;
        } else if *new_entry != moves {
            return Err(TableFillError);
        }
    }

    Ok(table)
}

#[derive(Debug)]
pub struct SlidingMoves {
    rook_magic_table: Vec<Vec<Mask>>,
    bishop_magic_table: Vec<Vec<Mask>>,
}

impl SlidingMoves {
    pub fn init() -> Self {
        Self {
            rook_magic_table: create_move_list(Direction::Orthogonal, &ROOK_MAGICS),
            bishop_magic_table: create_move_list(Direction::Diagonal, &BISHOP_MAGICS),
        }
    }

    pub fn get_rook_moves(&self, square: Square, blockers: Mask) -> Mask {
        let magic = &ROOK_MAGICS[square as usize];
        let moves = &self.rook_magic_table[square as usize];

        moves[magic.index(blockers)]
    }

    pub fn get_bishop_moves(&self, square: Square, blockers: Mask) -> Mask {
        let magic = &BISHOP_MAGICS[square as usize];
        let moves = &self.bishop_magic_table[square as usize];

        moves[magic.index(blockers)]
    }
}

#[derive(Debug, Clone)]
pub struct MagicEntry {
    pub mask: Mask,
    pub magic: u64,
    pub index_bits: u8,
}

impl MagicEntry {
    pub fn index(&self, blockers: Mask) -> usize {
        let blockers = blockers & self.mask;
        let mul = blockers.0.wrapping_mul(self.magic);

        (mul >> (64 - self.index_bits)) as usize
    }
}

#[cfg(test)]
mod move_gen_tests {
    use super::*;

    #[test]
    fn bishop_jumping() {
        let movegen = SlidingMoves::init();

        BISHOP_MAGICS[Square::C1 as usize].mask.print();

        movegen.get_bishop_moves(Square::C1, Mask(u64::MAX)).print();
    }
}
