use crate::Color;

use super::{BitBoard, Engine};

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

    fn get_move(&mut self, _board: &BitBoard, _color: &Color) -> u64 {
        0
    }
}
