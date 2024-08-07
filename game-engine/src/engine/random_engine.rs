use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::Color;

use super::{rules::{get_capture_map, get_possible_moves, Move}, BitBoard, Engine};

pub struct RandomEngine {
    rng: ThreadRng,
}

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine {
            rng: thread_rng(),
        }
    }
}

impl Engine for RandomEngine {
    fn get_name(&self) -> String {
        String::from("Random Engine")
    }

    fn get_move(&mut self, board: &mut BitBoard, color: &Color) -> Move {
        let moves = get_possible_moves(board, color);
        let opp_color = color.opposite();

        let mut moves: Vec<Move> = moves.into_iter().filter(|mv| {
            board.apply_move(mv, color);
            let captures = get_capture_map(board, &opp_color);
            let king = board.get_king_by_color(color);
            let no_check = king & captures == 0;
            board.apply_move(mv, color);
            no_check
        }).collect();

        let index = self.rng.gen_range(0..moves.len());
        moves.swap_remove(index)
    }
}
