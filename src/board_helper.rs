use crate::mask::Mask;
use crate::piece::{Color, Piece, BISHOP_MOVE_OFFSETS, KNIGHT_MOVE_OFFSETS, ROOK_MOVE_OFFSETS};

pub struct BoardHelper;

impl BoardHelper {
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

        masks.map(|val| Mask(val))
    }

    pub fn generate_black_pawn_capture_masks() -> [Mask; 64] {
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

        masks.map(|val| Mask(val))
    }

    pub fn generate_rook_move_masks() -> [Mask; 64] {
        let mut masks = [0; 64];

        for start in 0..64 {
            for offset in ROOK_MOVE_OFFSETS {
                let mut target = start as i8 + offset;
                let mut prev_rank = Self::rank(start);
                let mut prev_file = Self::file(start);

                while target >= 0 && target < 64 {
                    // If moving by offset wraps you around the board then stop
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

        masks.map(|val| Mask(val))
    }

    pub fn generate_bishop_move_masks() -> [Mask; 64] {
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

        masks.map(|val| Mask(val))
    }

    pub fn generate_knight_move_masks() -> [Mask; 64] {
        let mut masks = [0; 64];

        for start in 0..64 {
            let rank = Self::rank(start);
            let file = Self::file(start);

            for offset in KNIGHT_MOVE_OFFSETS {
                let target = start as i8 + offset;

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

        masks.map(|val| Mask(val))
    }

    pub fn generate_king_move_masks() -> [Mask; 64] {
        let mut masks = [0; 64];

        for start in 0..64 {
            let rank = Self::rank(start);
            let file = Self::file(start);

            for offset_set in [ROOK_MOVE_OFFSETS, BISHOP_MOVE_OFFSETS] {
                for offset in offset_set {
                    let target = start as i8 + offset;

                    if target < 0 || target > 63 {
                        continue;
                    }

                    let target = target as usize;

                    if Self::rank_difference(rank, target) > 1
                        || Self::file_difference(file, target) > 1
                    {
                        continue;
                    }

                    masks[start] |= 1 << target;
                }
            }
        }

        masks.map(|val| Mask(val))
    }

    pub fn char_to_piece(ch: char) -> Option<Piece> {
        match ch {
            'P' => Some(Piece::Pawn(Color::White)),
            'N' => Some(Piece::Knight(Color::White)),
            'B' => Some(Piece::Bishop(Color::White)),
            'R' => Some(Piece::Rook(Color::White)),
            'Q' => Some(Piece::Queen(Color::White)),
            'K' => Some(Piece::King(Color::White)),
            'p' => Some(Piece::Pawn(Color::Black)),
            'n' => Some(Piece::Knight(Color::Black)),
            'b' => Some(Piece::Bishop(Color::Black)),
            'r' => Some(Piece::Rook(Color::Black)),
            'q' => Some(Piece::Queen(Color::Black)),
            'k' => Some(Piece::King(Color::Black)),
            _ => None,
        }
    }

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

            _ => panic!("invalid piece char '{piece}'"),
        }
    }

    pub fn print_mask(mask: &Mask) {
        let string_mask = format!("{:064b}", mask.0.reverse_bits());
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
