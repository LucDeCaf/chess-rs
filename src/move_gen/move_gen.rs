use std::array::from_fn;

use crate::{
    mask::Mask,
    piece::{Direction, BISHOP_MOVE_OFFSETS, KNIGHT_MOVE_OFFSETS, ROOK_MOVE_OFFSETS},
    square::Square,
};

use super::magics::{BISHOP_INDEX_BITS, BISHOP_MAGICS, ROOK_INDEX_BITS, ROOK_MAGICS};

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

pub fn create_move_list(direction: Direction, magics: &[MagicEntry]) -> Vec<Vec<Mask>> {
    let mut moves = Vec::with_capacity(64);

    for (i, magic) in magics.into_iter().enumerate() {
        let move_table =
            try_fill_magic_table(direction, magic, Square::from_usize(i).unwrap()).unwrap();
        moves.push(move_table);
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
pub struct MoveGen {
    orthogonal_magics: Vec<MagicEntry>,
    diagonal_magics: Vec<MagicEntry>,
    orthogonal_moves: Vec<Vec<Mask>>,
    diagonal_moves: Vec<Vec<Mask>>,
}

impl MoveGen {
    pub fn init() -> Self {
        let orthogonal_magics = Vec::from_iter::<[MagicEntry; 64]>(from_fn(|i| MagicEntry {
            mask: Square::from_usize(i).unwrap().mask(),
            magic: ROOK_MAGICS[i],
            index_bits: ROOK_INDEX_BITS,
        }));

        let diagonal_magics = Vec::from_iter::<[MagicEntry; 64]>(from_fn(|i| MagicEntry {
            mask: Square::from_usize(i).unwrap().mask(),
            magic: BISHOP_MAGICS[i],
            index_bits: BISHOP_INDEX_BITS,
        }));

        let orthogonal_moves = create_move_list(Direction::Orthogonal, &orthogonal_magics);

        let diagonal_moves = create_move_list(Direction::Diagonal, &diagonal_magics);

        Self {
            orthogonal_magics,
            diagonal_magics,
            orthogonal_moves,
            diagonal_moves,
        }
    }

    pub fn get_rook_moves(&self, square: Square, blockers: Mask) -> Mask {
        let magic = &self.orthogonal_magics[square as usize];
        let moves = &self.orthogonal_moves[square as usize];

        moves[magic.index(blockers)]
    }

    pub fn get_bishop_moves(&self, square: Square, blockers: Mask) -> Mask {
        let magic = &self.diagonal_magics[square as usize];
        let moves = &self.diagonal_moves[square as usize];

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
