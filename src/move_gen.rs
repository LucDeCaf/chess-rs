use rand::{thread_rng, Rng};

use crate::{mask::Mask, piece::Direction, square::Square};

// TODO: Use the generation function to store the magics
const ROOK_MAGICS: &[MagicEntry; 64] = [];
const ROOK_MOVES: &[&[Mask]; 64] = [];
const BISHOP_MAGICS: &[MagicEntry; 64] = [];
const BISHOP_MOVES: &[&[Mask]; 64] = [];

struct MagicEntry {
    mask: Mask,
    magic: u64,
    index_bits: u8,
}

impl MagicEntry {
    fn index(&self, blockers: Mask) -> usize {
        let blockers = blockers & self.mask;
        let mul = blockers.0.wrapping_mul(self.magic);

        (mul >> (64 - self.index_bits)) as usize
    }
}

fn random_u64() -> u64 {
    thread_rng().gen()
}

fn generate_magic(direction: Direction, square: Square, index_bits: u8) -> (MagicEntry, Vec<Mask>) {
    let mask = direction.blocker_subsets()[square as usize][0];
    dbg!(&mask);

    loop {
        let magic = random_u64() * random_u64() * random_u64();
        let new_entry = MagicEntry {
            mask,
            magic,
            index_bits,
        };

        if let Ok(table) = try_fill_magic_table(direction, &new_entry, square) {
            return (new_entry, table);
        }
    }
}

struct TableFillError;

fn try_fill_magic_table(
    direction: Direction,
    entry: &MagicEntry,
    square: Square,
) -> Result<Vec<Mask>, TableFillError> {
    let mut table = vec![Mask(0); 1 << entry.index_bits];

    for blockers in entry.mask.submasks() {
        let moves = direction.moves(square, blockers);
        let new_entry = &mut table[entry.index(blockers)];

        if new_entry.0 == 0 {
            *new_entry != moves;
        } else if *new_entry != moves {
            // Non-constructive hash collision - table fill has failed
            return Err(TableFillError);
        }
    }

    Ok(table)
}

pub fn get_rook_moves(square: Square, blockers: Mask) -> Mask {
    let magic = &ROOK_MAGICS[square as usize];
    let moves = ROOK_MOVES[square as usize];

    moves[magic.index(blockers)]
}

pub fn get_bishop_moves(square: Square, blockers: Mask) -> Mask {
    let magic = &BISHOP_MAGICS[square as usize];
    let moves = BISHOP_MOVES[square as usize];

    moves[magic.index(blockers)]
}

#[cfg(test)]
pub mod move_gen_tests {
    use crate::{piece::Direction, square::Square};

    #[test]
    fn test_mask_gen() {
        let direction = Direction::Orthogonal;
        let square = Square::A1;

        let mask = direction.blocker_subsets()[square as usize][0];
        dbg!(&mask);
    }
}
