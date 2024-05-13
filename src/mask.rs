use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Mask(pub u64);

impl Mask {
    pub fn ones(&self) -> Vec<Square> {
        let mut squares = vec![];

        let mut square = self.0;
        let mut i = 0;

        while square > 0 {
            if square & 1 == 1 {
                squares.push(Square::from_u8(i).unwrap());
            }

            square >>= 1;
            i += 1;
        }

        squares
    }

    pub fn zeroes(&self) -> Vec<Square> {
        let mut squares = vec![];

        let mut square = !self.0;
        let mut i = 0;

        while square > 0 {
            if square & 1 == 1 {
                squares.push(Square::from_u8(i).unwrap());
            }

            square >>= 1;
            i += 1;
        }

        squares
    }
}

impl BitAnd for Mask {
    fn bitand(self, rhs: Self) -> Self::Output {
        Mask(self.0 & rhs.0)
    }

    type Output = Self;
}

impl BitAndAssign for Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for Mask {
    fn bitor(self, rhs: Self) -> Self::Output {
        Mask(self.0 | rhs.0)
    }

    type Output = Self;
}

impl BitOrAssign for Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for Mask {
    fn bitxor(self, rhs: Self) -> Self::Output {
        Mask(self.0 ^ rhs.0)
    }

    type Output = Self;
}

impl BitXorAssign for Mask {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for Mask {
    fn not(self) -> Self::Output {
        Mask(!self.0)
    }

    type Output = Self;
}

#[cfg(test)]
mod mask_tests {
    use super::*;

    #[test]
    fn test_mask_squares() {
        assert_eq!(
            Mask(0b00000000000000000000000000000001).ones(),
            vec![Square::A1]
        );
        assert_eq!(
            Mask(0b00000000000000000000000100000001).ones(),
            vec![Square::A1, Square::A2]
        );
        assert_eq!(
            Mask(0b00000000000000000101000100000001).ones(),
            vec![Square::A1, Square::A2, Square::E2, Square::G2]
        );
    }
}
