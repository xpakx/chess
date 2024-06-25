use crate::Color;

use super::BitBoard;

const NOT_A_FILE: u64 =  0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
const NOT_AB_FILE: u64 = 0b00111111_00111111_00111111_00111111_00111111_00111111_00111111_00111111;
const NOT_H_FILE: u64 =  0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
const NOT_GH_FILE: u64 = 0b11111100_11111100_11111100_11111100_11111100_11111100_11111100_11111100;
const RANK_4: u64 = 0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
const RANK_5: u64 = 0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;

pub fn get_possible_moves(board: &BitBoard, color: &Color) -> Vec<u64> {
    vec![]
}

pub fn get_knight_moves(knight: &u64, targets: &u64) -> u64 {
    let mut result: u64 = 0;
    result = result | ((knight << 17) & NOT_A_FILE);
    result = result | ((knight << 10) & NOT_AB_FILE);
    result = result | ((knight >> 6) & NOT_AB_FILE);
    result = result | ((knight >> 15) & NOT_A_FILE);
    result = result | ((knight << 15) & NOT_H_FILE);
    result = result | ((knight << 6) & NOT_GH_FILE);
    result = result | ((knight >> 10) & NOT_GH_FILE);
    result = result | ((knight >> 17) & NOT_H_FILE);
    result & targets
}

pub fn get_king_moves(king: &u64, targets: &u64) -> u64 {
    let mut result: u64 = 0;
    result = result | (king << 8);
    result = result | (king >> 8);
    result = result | ((king << 1) & NOT_A_FILE);
    result = result | ((king << 9) & NOT_A_FILE);
    result = result | ((king >> 7) & NOT_A_FILE);
    result = result | ((king >> 1) & NOT_H_FILE);
    result = result | ((king >> 9) & NOT_H_FILE);
    result = result | ((king << 7) & NOT_H_FILE);
    result & targets
}

pub fn get_white_pawn_single_pushes(pawns: &u64, empty: &u64) -> u64 {
    (pawns << 8) & empty
}

pub fn get_black_pawn_single_pushes(pawns: &u64, empty: &u64) -> u64 {
    (pawns >> 8) & empty
}

pub fn get_white_pawn_double_pushes(pawns: &u64, empty: &u64) -> u64 {
    let single = get_white_pawn_single_pushes(pawns, empty);
    (single << 8) & empty & RANK_4
}

pub fn get_black_pawn_double_pushes(pawns: &u64, empty: &u64) -> u64 {
    let single = get_black_pawn_single_pushes(pawns, empty);
    (single >> 8) & empty & RANK_5
}

pub fn get_white_pawn_east_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns << 9) & NOT_A_FILE & targets
}

pub fn get_white_pawn_west_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns << 7) & NOT_H_FILE & targets
}

pub fn get_black_pawn_east_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns >> 7) & NOT_A_FILE & targets
}

pub fn get_black_pawn_west_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns >> 9) & NOT_H_FILE & targets
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
