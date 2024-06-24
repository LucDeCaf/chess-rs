use crate::board::mask::Mask;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub fn mask(&self) -> Mask {
        Mask(1 << *self as u8)
    }

    pub fn rank(&self) -> Rank {
        Rank::from_u8(*self as u8 / 8).unwrap()
    }

    pub fn file(&self) -> File {
        File::from_u8(*self as u8 % 8).unwrap()
    }

    pub fn coords(&self) -> (Rank, File) {
        (self.rank(), self.file())
    }

    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::A1),
            1 => Some(Self::B1),
            2 => Some(Self::C1),
            3 => Some(Self::D1),
            4 => Some(Self::E1),
            5 => Some(Self::F1),
            6 => Some(Self::G1),
            7 => Some(Self::H1),
            8 => Some(Self::A2),
            9 => Some(Self::B2),
            10 => Some(Self::C2),
            11 => Some(Self::D2),
            12 => Some(Self::E2),
            13 => Some(Self::F2),
            14 => Some(Self::G2),
            15 => Some(Self::H2),
            16 => Some(Self::A3),
            17 => Some(Self::B3),
            18 => Some(Self::C3),
            19 => Some(Self::D3),
            20 => Some(Self::E3),
            21 => Some(Self::F3),
            22 => Some(Self::G3),
            23 => Some(Self::H3),
            24 => Some(Self::A4),
            25 => Some(Self::B4),
            26 => Some(Self::C4),
            27 => Some(Self::D4),
            28 => Some(Self::E4),
            29 => Some(Self::F4),
            30 => Some(Self::G4),
            31 => Some(Self::H4),
            32 => Some(Self::A5),
            33 => Some(Self::B5),
            34 => Some(Self::C5),
            35 => Some(Self::D5),
            36 => Some(Self::E5),
            37 => Some(Self::F5),
            38 => Some(Self::G5),
            39 => Some(Self::H5),
            40 => Some(Self::A6),
            41 => Some(Self::B6),
            42 => Some(Self::C6),
            43 => Some(Self::D6),
            44 => Some(Self::E6),
            45 => Some(Self::F6),
            46 => Some(Self::G6),
            47 => Some(Self::H6),
            48 => Some(Self::A7),
            49 => Some(Self::B7),
            50 => Some(Self::C7),
            51 => Some(Self::D7),
            52 => Some(Self::E7),
            53 => Some(Self::F7),
            54 => Some(Self::G7),
            55 => Some(Self::H7),
            56 => Some(Self::A8),
            57 => Some(Self::B8),
            58 => Some(Self::C8),
            59 => Some(Self::D8),
            60 => Some(Self::E8),
            61 => Some(Self::F8),
            62 => Some(Self::G8),
            63 => Some(Self::H8),
            _ => None,
        }
    }

    pub fn from_u16(val: u16) -> Option<Self> {
        if val > u8::MAX as u16 {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        if val > u8::MAX as u32 {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_u64(val: u64) -> Option<Self> {
        if val > u8::MAX as u64 {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_usize(val: usize) -> Option<Self> {
        if val > u8::MAX as usize {
            return None;
        }

        Self::from_u8(val as u8)
    }

    pub fn from_mask(mask: Mask) -> Option<Self> {
        let ones = mask.ones();

        if ones.len() != 1 {
            return None;
        }

        Self::from_u8(ones[0].to_shift() as u8)
    }

    pub fn from_coords(rank: Rank, file: File) -> Self {
        Self::from_u8(rank as u8 * 8 + file as u8).unwrap()
    }

    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "a1" => Some(Square::A1),
            "b1" => Some(Square::B1),
            "c1" => Some(Square::C1),
            "d1" => Some(Square::D1),
            "e1" => Some(Square::E1),
            "f1" => Some(Square::F1),
            "g1" => Some(Square::G1),
            "h1" => Some(Square::H1),
            "a2" => Some(Square::A2),
            "b2" => Some(Square::B2),
            "c2" => Some(Square::C2),
            "d2" => Some(Square::D2),
            "e2" => Some(Square::E2),
            "f2" => Some(Square::F2),
            "g2" => Some(Square::G2),
            "h2" => Some(Square::H2),
            "a3" => Some(Square::A3),
            "b3" => Some(Square::B3),
            "c3" => Some(Square::C3),
            "d3" => Some(Square::D3),
            "e3" => Some(Square::E3),
            "f3" => Some(Square::F3),
            "g3" => Some(Square::G3),
            "h3" => Some(Square::H3),
            "a4" => Some(Square::A4),
            "b4" => Some(Square::B4),
            "c4" => Some(Square::C4),
            "d4" => Some(Square::D4),
            "e4" => Some(Square::E4),
            "f4" => Some(Square::F4),
            "g4" => Some(Square::G4),
            "h4" => Some(Square::H4),
            "a5" => Some(Square::A5),
            "b5" => Some(Square::B5),
            "c5" => Some(Square::C5),
            "d5" => Some(Square::D5),
            "e5" => Some(Square::E5),
            "f5" => Some(Square::F5),
            "g5" => Some(Square::G5),
            "h5" => Some(Square::H5),
            "a6" => Some(Square::A6),
            "b6" => Some(Square::B6),
            "c6" => Some(Square::C6),
            "d6" => Some(Square::D6),
            "e6" => Some(Square::E6),
            "f6" => Some(Square::F6),
            "g6" => Some(Square::G6),
            "h6" => Some(Square::H6),
            "a7" => Some(Square::A7),
            "b7" => Some(Square::B7),
            "c7" => Some(Square::C7),
            "d7" => Some(Square::D7),
            "e7" => Some(Square::E7),
            "f7" => Some(Square::F7),
            "g7" => Some(Square::G7),
            "h7" => Some(Square::H7),
            "a8" => Some(Square::A8),
            "b8" => Some(Square::B8),
            "c8" => Some(Square::C8),
            "d8" => Some(Square::D8),
            "e8" => Some(Square::E8),
            "f8" => Some(Square::F8),
            "g8" => Some(Square::G8),
            "h8" => Some(Square::H8),
            _ => None,
        }
    }

    pub fn to_shift(&self) -> usize {
        match self {
            Self::A1 => 0,
            Self::B1 => 1,
            Self::C1 => 2,
            Self::D1 => 3,
            Self::E1 => 4,
            Self::F1 => 5,
            Self::G1 => 6,
            Self::H1 => 7,
            Self::A2 => 8,
            Self::B2 => 9,
            Self::C2 => 10,
            Self::D2 => 11,
            Self::E2 => 12,
            Self::F2 => 13,
            Self::G2 => 14,
            Self::H2 => 15,
            Self::A3 => 16,
            Self::B3 => 17,
            Self::C3 => 18,
            Self::D3 => 19,
            Self::E3 => 20,
            Self::F3 => 21,
            Self::G3 => 22,
            Self::H3 => 23,
            Self::A4 => 24,
            Self::B4 => 25,
            Self::C4 => 26,
            Self::D4 => 27,
            Self::E4 => 28,
            Self::F4 => 29,
            Self::G4 => 30,
            Self::H4 => 31,
            Self::A5 => 32,
            Self::B5 => 33,
            Self::C5 => 34,
            Self::D5 => 35,
            Self::E5 => 36,
            Self::F5 => 37,
            Self::G5 => 38,
            Self::H5 => 39,
            Self::A6 => 40,
            Self::B6 => 41,
            Self::C6 => 42,
            Self::D6 => 43,
            Self::E6 => 44,
            Self::F6 => 45,
            Self::G6 => 46,
            Self::H6 => 47,
            Self::A7 => 48,
            Self::B7 => 49,
            Self::C7 => 50,
            Self::D7 => 51,
            Self::E7 => 52,
            Self::F7 => 53,
            Self::G7 => 54,
            Self::H7 => 55,
            Self::A8 => 56,
            Self::B8 => 57,
            Self::C8 => 58,
            Self::D8 => 59,
            Self::E8 => 60,
            Self::F8 => 61,
            Self::G8 => 62,
            Self::H8 => 63,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Square::A1 => String::from("a1"),
            Square::B1 => String::from("b1"),
            Square::C1 => String::from("c1"),
            Square::D1 => String::from("d1"),
            Square::E1 => String::from("e1"),
            Square::F1 => String::from("f1"),
            Square::G1 => String::from("g1"),
            Square::H1 => String::from("h1"),
            Square::A2 => String::from("a2"),
            Square::B2 => String::from("b2"),
            Square::C2 => String::from("c2"),
            Square::D2 => String::from("d2"),
            Square::E2 => String::from("e2"),
            Square::F2 => String::from("f2"),
            Square::G2 => String::from("g2"),
            Square::H2 => String::from("h2"),
            Square::A3 => String::from("a3"),
            Square::B3 => String::from("b3"),
            Square::C3 => String::from("c3"),
            Square::D3 => String::from("d3"),
            Square::E3 => String::from("e3"),
            Square::F3 => String::from("f3"),
            Square::G3 => String::from("g3"),
            Square::H3 => String::from("h3"),
            Square::A4 => String::from("a4"),
            Square::B4 => String::from("b4"),
            Square::C4 => String::from("c4"),
            Square::D4 => String::from("d4"),
            Square::E4 => String::from("e4"),
            Square::F4 => String::from("f4"),
            Square::G4 => String::from("g4"),
            Square::H4 => String::from("h4"),
            Square::A5 => String::from("a5"),
            Square::B5 => String::from("b5"),
            Square::C5 => String::from("c5"),
            Square::D5 => String::from("d5"),
            Square::E5 => String::from("e5"),
            Square::F5 => String::from("f5"),
            Square::G5 => String::from("g5"),
            Square::H5 => String::from("h5"),
            Square::A6 => String::from("a6"),
            Square::B6 => String::from("b6"),
            Square::C6 => String::from("c6"),
            Square::D6 => String::from("d6"),
            Square::E6 => String::from("e6"),
            Square::F6 => String::from("f6"),
            Square::G6 => String::from("g6"),
            Square::H6 => String::from("h6"),
            Square::A7 => String::from("a7"),
            Square::B7 => String::from("b7"),
            Square::C7 => String::from("c7"),
            Square::D7 => String::from("d7"),
            Square::E7 => String::from("e7"),
            Square::F7 => String::from("f7"),
            Square::G7 => String::from("g7"),
            Square::H7 => String::from("h7"),
            Square::A8 => String::from("a8"),
            Square::B8 => String::from("b8"),
            Square::C8 => String::from("c8"),
            Square::D8 => String::from("d8"),
            Square::E8 => String::from("e8"),
            Square::F8 => String::from("f8"),
            Square::G8 => String::from("g8"),
            Square::H8 => String::from("h8"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Rank::One),
            1 => Some(Rank::Two),
            2 => Some(Rank::Three),
            3 => Some(Rank::Four),
            4 => Some(Rank::Five),
            5 => Some(Rank::Six),
            6 => Some(Rank::Seven),
            7 => Some(Rank::Eight),
            _ => None,
        }
    }

    pub fn from_index(i: usize) -> Option<Self> {
        if i > 63 {
            return None;
        }

        let normalised = i / 8;
        Self::from_u8(normalised as u8)
    }

    pub fn plus(&self, val: u8) -> Option<Self> {
        Self::from_u8(*self as u8 + val)
    }

    pub fn minus(&self, val: u8) -> Option<Self> {
        Self::from_u8(*self as u8 - val)
    }

    pub fn diff(&self, rhs: Rank) -> u8 {
        (*self as u8).abs_diff(rhs as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }

    pub fn from_index(i: usize) -> Option<Self> {
        if i > 63 {
            return None;
        }

        let normalised = i / 8;
        Self::from_u8(normalised as u8)
    }

    pub fn plus(&self, val: u8) -> Option<Self> {
        Self::from_u8(*self as u8 + val)
    }

    pub fn minus(&self, val: u8) -> Option<Self> {
        Self::from_u8(*self as u8 - val)
    }

    pub fn diff(&self, rhs: File) -> u8 {
        (*self as u8).abs_diff(rhs as u8)
    }
}
#[cfg(test)]
mod square_tests {
    use super::*;

    #[test]
    fn test_ranks_and_files() {
        let ranks = vec![
            Some(Rank::One),
            Some(Rank::Two),
            Some(Rank::Three),
            Some(Rank::Four),
            Some(Rank::Five),
            Some(Rank::Six),
            Some(Rank::Seven),
            Some(Rank::Eight),
        ];

        let files = vec![
            Some(File::A),
            Some(File::B),
            Some(File::C),
            Some(File::D),
            Some(File::E),
            Some(File::F),
            Some(File::G),
            Some(File::H),
        ];

        for i in 0..8 {
            assert_eq!(Rank::from_u8(i as u8), ranks[i]);
            assert_eq!(File::from_u8(i as u8), files[i]);
        }
    }

    #[test]
    fn test_square_to_rank() {
        let square = Square::E4;
        assert_eq!(square.file(), File::E);
        assert_eq!(square.rank(), Rank::Four);

        let square = Square::H8;
        assert_eq!(square.file(), File::H);
        assert_eq!(square.rank(), Rank::Eight);

        let square = Square::C1;
        assert_eq!(square.file(), File::C);
        assert_eq!(square.rank(), Rank::One);
    }

    #[test]
    fn square_from_coords() {
        let pairs = vec![
            (Rank::One, File::A),
            (Rank::Three, File::B),
            (Rank::One, File::C),
            (Rank::Four, File::D),
            (Rank::Eight, File::E),
            (Rank::Eight, File::F),
            (Rank::Five, File::G),
            (Rank::Two, File::H),
        ];

        let expected = vec![
            Square::A1,
            Square::B3,
            Square::C1,
            Square::D4,
            Square::E8,
            Square::F8,
            Square::G5,
            Square::H2,
        ];

        for (i, (rank, file)) in pairs.into_iter().enumerate() {
            assert_eq!(Square::from_coords(rank, file), expected[i]);
        }
    }

    #[test]
    fn square_from_mask() {
        let squares = vec![
            Square::A1,
            Square::E3,
            Square::H1,
            Square::F4,
            Square::A8,
            Square::C8,
            Square::E7,
            Square::F7,
        ];

        for square in squares {
            let mask = square.mask();
            assert_eq!(square, Square::from_mask(mask).unwrap());
        }
    }
}
