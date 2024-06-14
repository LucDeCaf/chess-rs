use std::io::{stdout, Write};

use rand::{thread_rng, Rng};

use crate::{mask::Mask, piece::Direction, square::Square};

const INDEX_BITS: u8 = 16;
// const ROOK_MOVE_DATA: (Vec<MagicEntry>, Vec<Vec<Mask>>) =
//     create_magics(Direction::Orthogonal, INDEX_BITS);
// const BISHOP_MOVE_DATA: (Vec<MagicEntry>, Vec<Vec<Mask>>) =
//     create_magics(Direction::Diagonal, INDEX_BITS);

#[derive(Debug, Clone)]
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

    loop {
        let magic = random_u64() & random_u64() & random_u64();
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

    // Debugging variable
    let mut iters: u128 = 0;

    for blockers in entry.mask.submasks() {
        iters += 1;
        let moves = direction.moves(square, blockers);
        let new_entry = &mut table[entry.index(blockers)];

        if new_entry.0 == 0 {
            *new_entry = moves;
        } else if *new_entry != moves {
            // Non-constructive hash collision - table fill has failed
            return Err(TableFillError);
        }
    }

    // println!("tried {iters} different magics");
    // println!("magic created successfully");
    Ok(table)
}

fn create_magics(direction: Direction, index_bits: u8) -> (Vec<MagicEntry>, Vec<Vec<Mask>>) {
    let mut magics: Vec<MagicEntry> = Vec::with_capacity(64);
    let mut masks: Vec<Vec<Mask>> = Vec::with_capacity(64);

    for i in 0..64 {
        let square = Square::from_usize(i).unwrap();

        // println!("generating magic {i}...");
        let (new_magics, new_masks) = generate_magic(direction, square, index_bits);
        magics.push(new_magics);
        masks.push(new_masks);
    }

    (magics, masks)
}

// pub fn get_rook_moves(square: Square, blockers: Mask) -> Mask {
//     let magic = &ROOK_MOVE_DATA.0[square as usize];
//     let moves = &ROOK_MOVE_DATA.1[square as usize];

//     moves[magic.index(blockers)]
// }

// pub fn get_bishop_moves(square: Square, blockers: Mask) -> Mask {
//     let magic = &BISHOP_MOVE_DATA.0[square as usize];
//     let moves = &BISHOP_MOVE_DATA.1[square as usize];

//     moves[magic.index(blockers)]
// }

#[cfg(test)]
pub mod move_gen_tests {
    use crate::{board_helper::BoardHelper, piece::Direction};

    use super::*;

    #[test]
    fn debug_magic_generation() {
        let direction = Direction::Orthogonal;
        let square = Square::A1;
        let index_bits = 16;
        let (magic, moves) = generate_magic(direction, square, index_bits);

        dbg!(magic);
    }

    #[test]
    fn debug_magic_using() {
        let direction = Direction::Diagonal;
        let square = Square::A1;
        let index_bits = 16;

        let (magic, moves) = generate_magic(direction, square, index_bits);

        let blockers = Mask(random_u64() & random_u64());

        println!("Blockers:");
        BoardHelper::print_mask(&blockers);
        println!();

        println!("Relevant blockers:");
        BoardHelper::print_mask(&(blockers & direction.blockers()[square as usize]));
        println!();

        println!("Moves from A1:");
        BoardHelper::print_mask(&moves[magic.index(blockers)]);
    }

    #[test]
    fn try_create_magics() {
        let (rook_magics, rook_moves) = create_magics(Direction::Orthogonal, INDEX_BITS);
    }
}
