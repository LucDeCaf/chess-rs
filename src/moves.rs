use crate::{mask::Mask, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn from_long_algebraic(input: &str) -> Option<Move> {
        Some(Move {
            from: Square::from_str(&input[..2])?,
            to: Square::from_str(&input[2..])?,
        })
    }

    pub fn from_move_mask(from: Square, move_mask: Mask) -> Vec<Move> {
        let targets = move_mask.ones();
        let mut moves = Vec::with_capacity(targets.len());

        for to in targets {
            moves.push(Move { from, to });
        }

        moves
    }
}

#[derive(Debug, Clone)]
pub struct MoveError;
