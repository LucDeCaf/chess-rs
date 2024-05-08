pub struct BoardHelper;

const KNIGHT_MOVE_OFFSETS: [i8; 8] = [15, 17, 6, 10, -10, -6, -17, -15];
const BISHOP_MOVE_OFFSETS: [i8; 4] = [7, 9, -7, -9];
const ROOK_MOVE_OFFSETS: [i8; 4] = [8, 1, -8, -1];

impl BoardHelper {
    pub fn generate_white_pawn_masks() -> [u64; 64] {
        let mut masks = [0; 64];

        for i in 8..56 {
            masks[i] |= 1 << (i + 8);

            if i < 16 {
                masks[i] |= 1 << (i + 16);
            }
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
        }

        masks
    }

    pub fn generate_white_pawn_capture_masks() -> [u64; 64] {
        let mut masks = [0; 64];
        let (mut rank, mut file): (usize, usize);
        let mut mask: usize;

        for i in 8..56 {
            mask = 0;

            rank = Self::rank(i);
            file = Self::file(i);

            if Self::rank_difference(rank, i + 9) == 1 && Self::file_difference(file, i + 9) == 1 {
                mask |= 1 << (i + 9);
            }

            if Self::rank_difference(rank, i + 7) == 1 && Self::file_difference(file, i + 7) == 1 {
                mask |= 1 << (i + 7);
            }

            masks[i] = mask as u64;
        }

        masks
    }

    pub fn generate_black_pawn_capture_masks() -> [u64; 64] {
        let mut masks = [0; 64];
        let mut rank;
        let mut file;
        let mut mask: usize;

        for i in 8..56 {
            mask = 0;

            rank = Self::rank(i);
            file = Self::file(i);

            if i >= 9
                && Self::rank_difference(rank, i - 9) == 1
                && Self::file_difference(file, i - 9) == 1
            {
                mask |= (1 << i) >> 9;
            }

            if i >= 7
                && Self::rank_difference(rank, i - 7) == 1
                && Self::file_difference(file, i - 7) == 1
            {
                mask |= (1 << i) >> 7;
            }

            masks[i] = mask as u64;
        }

        masks
    }

    pub fn generate_rook_masks() -> [u64; 64] {
        let mut masks = [0; 64];

        for start in 0..64 {
            for offset in ROOK_MOVE_OFFSETS {
                let mut target = start as i8 + offset;
                let mut prev_rank = Self::rank(start as usize);
                let mut prev_file = Self::file(start as usize);

                while target >= 0 && target < 64 {
                    if Self::rank_difference(prev_rank, target as usize) > 1
                        || Self::file_difference(prev_file, target as usize) > 1
                    {
                        break;
                    }

                    prev_rank = Self::rank(target as usize);
                    prev_file = Self::file(target as usize);

                    masks[start] |= 1 << target;
                    target += offset;
                }
            }
        }

        masks
    }

    pub fn generate_bishop_masks() -> [u64; 64] {
        let mut masks = [0; 64];

        for start in 0..64 {
            for offset in BISHOP_MOVE_OFFSETS {
                let mut target = start as i8 + offset;
                let mut prev_rank = Self::rank(start as usize);
                let mut prev_file = Self::file(start as usize);

                while target >= 0 && target < 64 {
                    if Self::rank_difference(prev_rank, target as usize) > 1
                        || Self::file_difference(prev_file, target as usize) > 1
                    {
                        break;
                    }

                    prev_rank = Self::rank(target as usize);
                    prev_file = Self::file(target as usize);

                    masks[start] |= 1 << target;
                    target += offset;
                }
            }
        }

        masks
    }

    pub fn generate_knight_masks() -> [u64; 64] {
        let mut masks = [0; 64];

        for start in 0..64 {
            let rank = Self::rank(start);
            let file = Self::file(start);

            for offset in KNIGHT_MOVE_OFFSETS {
                let target = start as i8 + offset;

                if start == 0 {
                    println!("target: {target}");
                }

                if target < 0 || target > 63 {
                    continue;
                }

                let target = target as usize;

                let rank_diff = Self::rank_difference(rank, target);
                let file_diff = Self::file_difference(file, target);

                if !(rank_diff == 1 && file_diff == 2 || rank_diff == 2 && file_diff == 1) {
                    continue;
                }

                masks[start] |= 1 << target;
            }
        }

        masks
    }

    pub fn rank(i: usize) -> usize {
        i / 8
    }

    pub fn file(i: usize) -> usize {
        i % 8
    }

    fn rank_difference(rank: usize, tile: usize) -> usize {
        rank.abs_diff(tile / 8)
    }

    fn file_difference(file: usize, tile: usize) -> usize {
        file.abs_diff(tile % 8)
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
        let string_mask = format!("{:064b}", mask.reverse_bits());
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
