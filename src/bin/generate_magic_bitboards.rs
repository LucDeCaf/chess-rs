use std::fs;

use chess::{
    mask::Mask,
    move_gen::move_gen::{direction::Direction, try_fill_magic_table, MagicEntry},
    square::Square,
};

use rand::{thread_rng, Rng};

fn random_u64() -> u64 {
    thread_rng().gen()
}

fn create_magics(direction: Direction, index_bits: u8) -> (Vec<MagicEntry>, Vec<Vec<Mask>>) {
    let mut magics: Vec<MagicEntry> = Vec::with_capacity(64);
    let mut masks: Vec<Vec<Mask>> = Vec::with_capacity(64);

    for i in 0..64 {
        let square = Square::from_usize(i).unwrap();

        let (new_magics, new_masks) = generate_magic(direction, square, index_bits);
        magics.push(new_magics);
        masks.push(new_masks);
    }

    (magics, masks)
}

fn generate_magic(direction: Direction, square: Square, index_bits: u8) -> (MagicEntry, Vec<Mask>) {
    let mask = direction.all_blocker_subsets()[square as usize][0];

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

fn main() -> std::io::Result<()> {
    const DIR_PATH: &str = "src/move_gen";
    const FILE_NAME: &str = "magics.rs";

    let rook_index_bits = 16;
    let (rook_magics, _) = create_magics(Direction::Orthogonal, rook_index_bits);

    let bishop_index_bits = 14;
    let (bishop_magics, _) = create_magics(Direction::Orthogonal, bishop_index_bits);

    let mut buf = String::new();

    // Rook magics
    buf.push_str(&format!(
        "pub const ROOK_INDEX_BITS: u8 = {};\n",
        rook_index_bits,
    ));
    buf.push_str(&format!(
        "pub const ROOK_MAGICS: &[u64; 64] = &{:#?};\n",
        rook_magics.into_iter().map(|m| m.magic).collect::<Vec<u64>>()
    ));

    // Bishop magics
    buf.push_str(&format!(
        "pub const BISHOP_INDEX_BITS: u8 = {};\n",
        bishop_index_bits
    ));
    buf.push_str(&format!(
        "pub const BISHOP_MAGICS: &[u64; 64] = &{:#?};\n",
        bishop_magics.into_iter().map(|m| m.magic).collect::<Vec<u64>>()
    ));

    fs::create_dir_all(DIR_PATH)?;
    fs::write(format!("{DIR_PATH}/{FILE_NAME}"), buf)?;

    Ok(())
}

#[cfg(test)]
pub mod magic_gen_tests {
    use chess::move_gen::move_gen::direction::Direction;

    use super::*;

    #[test]
    fn debug_magic_generation() {
        let direction = Direction::Orthogonal;
        let square = Square::A1;
        let index_bits = 16;
        let (magic, _) = generate_magic(direction, square, index_bits);

        dbg!(magic);
    }

    #[test]
    fn debug_magic_index_usage() {
        let direction = Direction::Diagonal;
        let square = Square::A1;
        let index_bits = 16;

        let (magic, moves) = generate_magic(direction, square, index_bits);

        let blockers = Mask(random_u64() & random_u64());

        println!("Blockers:");
        blockers.print();
        println!();

        println!("Relevant blockers:");
        (blockers & direction.all_blockers()[square as usize]).print();
        println!();

        println!("Moves from A1:");
        moves[magic.index(blockers)].print();
    }
}
