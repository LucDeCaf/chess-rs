use crate::square::Square;

#[derive(Debug, Clone)]
pub struct Mask(pub u64);

impl Mask {
    pub fn squares(&self) -> Vec<Square> {
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
}

#[cfg(test)]
mod mask_tests {
    use super::*;

    #[test]
    fn test_mask_squares() {
        assert_eq!(
            Mask(0b00000000000000000000000000000001).squares(),
            vec![Square::A1]
        );
        assert_eq!(
            Mask(0b00000000000000000000000100000001).squares(),
            vec![Square::A1, Square::A2]
        );
        assert_eq!(
            Mask(0b00000000000000000101000100000001).squares(),
            vec![Square::A1, Square::A2, Square::E2, Square::G2]
        );
    }
}
