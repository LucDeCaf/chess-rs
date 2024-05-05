pub struct BoardHelper;

const KNIGHT_MOVE_OFFSETS: [i8; 8] = [15, 17, 6, 10, -10, -6, -17, -15];

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

    pub fn generate_white_pawn_capture_masks() -> [u64; 64] {
        let mut masks = [0; 64];
        let (mut rank, mut file): (usize, usize);
        let mut mask: usize;

        for i in 8..56 {
            mask = 0;

            rank = i / 8;
            file = i % 8;

            if i == 47 {
                println!(
                    "rankdiff: {}\nfilediff: {}",
                    rank.abs_diff((i + 7) / 8),
                    file.abs_diff((i + 7) % 8)
                );
            }

            if rank.abs_diff((i + 9) / 8) == 1 && file.abs_diff((i + 9) % 8) == 1 {
                mask |= 1 << (i + 9);
            }

            if rank.abs_diff((i + 7) / 8) == 1 && file.abs_diff((i + 7) % 8) == 1 {
                mask |= 1 << (i + 7);
            }

            masks[i] = mask.reverse_bits() as u64;
        }

        masks
    }

    pub fn generate_black_pawn_capture_masks() -> [u64; 64] {
        let mut masks = [0; 64];
        let mut rank: usize;
        let mut mask: usize;

        for i in 8..56 {
            mask = 0;
            rank = i / 8;

            if rank.abs_diff((i - 9) / 8) <= 1 {
                mask |= 1 << (i - 9);
            }

            if rank.abs_diff((i - 7) / 8) <= 1 {
                mask |= 1 << (i - 7);
            }

            masks[i] = mask.reverse_bits() as u64;
        }

        masks
    }

    pub fn generate_knight_masks() -> [u64; 64] {
        let mut masks = [0; 64];
        let (mut rank, mut file): (usize, usize);
        let (mut rank_diff, mut file_diff): (usize, usize);
        let mut mask: u64;

        for i in 0..64 {
            rank = i / 8;
            file = i % 8;

            mask = 0;

            for offset in KNIGHT_MOVE_OFFSETS {
                let target = (i as i8) + offset;

                // Prevent overflow
                if target > 63 || target < 0 {
                    continue;
                }

                let shift_right = offset > 0;
                let offset = offset.abs() as usize;

                rank_diff = rank.abs_diff((i + offset) / 8);
                file_diff = file.abs_diff((i + offset) % 8);

                // Prevent rank / file wrapping
                if rank_diff > 2 || file_diff > 2 {
                    continue;
                }

                let submask = 1 << i;
                if shift_right {
                    mask |= submask >> offset;
                } else {
                    mask |= submask << offset;
                }
            }

            masks[i] = mask.reverse_bits();
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
