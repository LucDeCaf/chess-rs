use crate::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub to: Square,
    pub from: Square,
}

impl Move {
    pub fn from_long_algebraic(input: &str) -> Option<Self> {
        Some(Self {
            from: Square::from_str(&input[..2])?,
            to: Square::from_str(&input[2..])?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MoveError;
