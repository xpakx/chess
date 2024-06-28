use crate::Color;

use super::{rules::{Move, Piece}, BitBoard, Engine};

pub struct RandomEngine {
}

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine {}
    }
}

impl Engine for RandomEngine {
    fn get_name(&self) -> String {
        String::from("Random Engine")
    }

    fn get_move(&mut self, _board: &BitBoard, _color: &Color) -> Move {
        Move {
            from: 0, to: 0, promotion: false, capture: None, castling: false, piece: Piece::Pawn,
        }
    }
}
