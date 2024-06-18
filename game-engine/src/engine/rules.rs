use crate::Color;

use super::BitBoard;

pub fn get_possible_moves(board: &BitBoard, color: &Color) -> Vec<u64> {
    vec![]
}

pub fn verify_move(board: &BitBoard, color: &Color, mov: u64) -> bool {
    true
}

pub fn is_game_won(board: &BitBoard, color: &Color) -> bool {
    false
}

pub fn is_game_drawn(board: &BitBoard, color: &Color) -> bool {
    false
}

pub fn move_to_string(board: &BitBoard, mov: u64) -> String {
    "e4".into()
}

pub fn string_to_move(board: &BitBoard, mov: String) -> u64 {
    0
}
