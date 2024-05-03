pub struct BoardHelper;

impl BoardHelper {
    pub fn generate_white_pawn_masks() -> [u64; 64] {
        let mut masks = [0; 64];

        for i in 8..56 {
            masks[i] |= 1 << (i + 8);

            if i < 16 {
                masks[i] |= 1 << (i + 16);
            }

            // Let binding required for reverse_bits
            let m: u64 = masks[i];
            masks[i] = m.reverse_bits();
        }

        masks
    }

    pub fn generate_black_pawn_masks() -> [u64; 64] {
        let mut masks = [0; 64];

        for i in 8..56 {
            masks[i] |= 1 << (i - 8);

            if i >= 48 {
                masks[i] |= 1 << (i - 16);
            }

            // Let binding required for reverse_bits
            let m: u64 = masks[i];
            masks[i] = m.reverse_bits();
        }

        masks
    }

    pub fn piece_to_bitboard_index(piece: char) -> usize {
        match piece {
            // White
            'P' => 0,
            'N' => 1,
            'B' => 2,
            'R' => 3,
            'Q' => 4,
            'K' => 5,

            // Black
            'p' => 6,
            'n' => 7,
            'b' => 8,
            'r' => 9,
            'q' => 10,
            'k' => 11,

            _ => panic!("Invalid piece char '{piece}'"),
        }
    }

    pub fn print_mask(mask: u64) {
        let string_mask = format!("{:064b}", mask);
        let mut lines = [""; 8];

        let mut i = 7;
        while i < 64 {
            lines[(i + 1) / 8 - 1] = &string_mask[i - 7..=i];
            i += 8;
        }

        for line in lines.iter().rev() {
            println!("{}", line);
        }
    }
}
