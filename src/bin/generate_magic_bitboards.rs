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
    let (rook_magics, _rook_table) = create_magics(Direction::Orthogonal, rook_index_bits);

    let bishop_index_bits = 14;
    let (bishop_magics, _bishop_table) = create_magics(Direction::Diagonal, bishop_index_bits);

    let mut buf = String::from("use super::move_gen::MagicEntry;\nuse crate::mask::Mask;\n");

    // Rook magics
    buf.push_str(&format!(
        "pub const ROOK_MAGICS: &[MagicEntry; 64] = &{:#?};\n",
        rook_magics
    ));

    // Bishop magics
    buf.push_str(&format!(
        "pub const BISHOP_MAGICS: &[MagicEntry; 64] = &{:#?};\n",
        bishop_magics
    ));

    fs::create_dir_all(DIR_PATH)?;
    fs::write(format!("{DIR_PATH}/{FILE_NAME}"), buf)?;

    Ok(())
}
