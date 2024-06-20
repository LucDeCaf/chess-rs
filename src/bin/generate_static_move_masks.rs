use std::{collections::HashMap, fs};

use chess::{mask::Mask, move_gen::move_gen};

fn main() -> std::io::Result<()> {
    let mut move_masks: HashMap<String, [Mask; 64]> = HashMap::new();

    move_masks.insert(
        "white_pawn_move_masks".to_owned(),
        move_gen::generate_white_pawn_move_masks(),
    );
    move_masks.insert(
        "white_pawn_capture_masks".to_owned(),
        move_gen::generate_white_pawn_capture_masks(),
    );
    move_masks.insert(
        "black_pawn_move_masks".to_owned(),
        move_gen::generate_black_pawn_move_masks(),
    );
    move_masks.insert(
        "black_pawn_capture_masks".to_owned(),
        move_gen::generate_black_pawn_capture_masks(),
    );
    move_masks.insert(
        "knight_move_masks".to_owned(),
        move_gen::generate_knight_move_masks(),
    );
    move_masks.insert(
        "bishop_move_masks".to_owned(),
        move_gen::generate_bishop_move_masks(),
    );
    move_masks.insert(
        "rook_move_masks".to_owned(),
        move_gen::generate_rook_move_masks(),
    );
    move_masks.insert(
        "king_move_masks".to_owned(),
        move_gen::generate_king_move_masks(),
    );

    const DIR_PATH: &str = "src";
    const FILE_NAME: &str = "move_masks.rs";

    fs::create_dir_all(DIR_PATH)?;
    let mut buf = String::from("use crate::mask::Mask;\n");

    for (key, value) in move_masks.iter() {
        buf.push_str(&format!(
            "pub const {}: [Mask; 64] = {:?};\n",
            key.to_uppercase(),
            value
        ));
    }

    fs::write(format!("{DIR_PATH}/{FILE_NAME}"), buf)?;

    Ok(())
}
