use once_cell::sync::Lazy;

use crate::Color;

use super::BitBoard;

const NOT_A_FILE: u64 =  0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
const NOT_AB_FILE: u64 = 0b00111111_00111111_00111111_00111111_00111111_00111111_00111111_00111111;
const NOT_H_FILE: u64 =  0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
const NOT_GH_FILE: u64 = 0b11111100_11111100_11111100_11111100_11111100_11111100_11111100_11111100;
const RANK_4: u64 =      0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
const RANK_5: u64 =      0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;


const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

const NORTHWEST: usize = 4;
const NORTHEAST: usize = 5;
const SOUTHEAST: usize = 6;
const SOUTHWEST: usize = 7;

pub static ROOK_RAYS: Lazy<[[u64; 64]; 4]> = Lazy::new(|| {
    let mut rays = [[0u64; 64]; 4];

    for square in 0..64 {
        rays[NORTH][square] = generate_ray(square, NORTH);
        rays[EAST][square] = generate_ray(square, EAST);
        rays[SOUTH][square] = generate_ray(square, SOUTH);
        rays[WEST][square] = generate_ray(square, WEST);
    }

    rays
});

pub static BISHOP_RAYS: Lazy<[[u64; 64]; 4]> = Lazy::new(|| {
    let mut rays = [[0u64; 64]; 4];

    for square in 0..64 {
        rays[NORTHWEST-4][square] = generate_ray(square, NORTHWEST);
        rays[NORTHEAST-4][square] = generate_ray(square, NORTHEAST);
        rays[SOUTHEAST-4][square] = generate_ray(square, SOUTHEAST);
        rays[SOUTHWEST-4][square] = generate_ray(square, SOUTHWEST);
    }

    rays
});

fn generate_ray(square: usize, direction: usize) -> u64 {
    let mut ray: u64 = 0;
    let mut sq: u64 = 1 << square;
    while sq != 0 {
        sq = match direction {
            NORTH => sq << 8,
            SOUTH => sq >> 8,
            EAST => sq << 1 & NOT_H_FILE,
            WEST => sq >> 1 & NOT_A_FILE,
            NORTHWEST => sq << 7 & NOT_A_FILE,
            NORTHEAST => sq << 9 & NOT_H_FILE,
            SOUTHEAST => sq >> 7 & NOT_H_FILE,
            SOUTHWEST => sq >> 9 & NOT_A_FILE,
            _ => panic!("error while constructing ray"),
        };
        ray = ray | sq;
    }
    ray
}

pub struct Move {
    from: u8,
    to: u8,
    promotion: bool,
    capture: bool,
    castling: bool,
}

pub fn get_possible_moves(board: &BitBoard, color: &Color) -> Vec<Move> {
    let (pawns, knights, bishops, rooks, queens, king, enemy) = match color {
        Color::Red => (board.black_pawns, board.black_knights, board.black_bishops, board.black_rooks, board.black_queens, board.black_king, 
                       board.white_pawns | board.white_knights | board.white_bishops | board.white_rooks | board.white_queens | board.white_king),
        Color::White => (board.white_pawns, board.white_knights, board.white_bishops, board.white_rooks, board.white_queens, board.white_king,
                       board.black_pawns | board.black_knights | board.black_bishops | board.black_rooks | board.black_queens | board.black_king),
    };
    let mut result = Vec::new();
    let mut current = knights;
    let friendly = pawns | knights | bishops | rooks | queens | king;
    let all_pieces = friendly | enemy;
    let empty = !all_pieces;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let knight = 1 << from;
        current = current & !knight;
        let mut moves = get_knight_moves(&knight, &(empty | enemy));
        while moves != 0 {
            let to = moves.trailing_zeros() as u8;
            moves = moves & !(1 << to);
            result.push(Move { from, to, promotion: false, capture: false, castling: false }); // TODO
        }
    }

    current = rooks | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let rook = 1 << from;
        current = current & !rook;
        let mut moves = get_rook_moves(&rook, &all_pieces, &friendly);
        while moves != 0 {
            let to = moves.trailing_zeros() as u8;
            moves = moves & !(1 << to);
            result.push(Move { from, to, promotion: false, capture: false, castling: false }); // TODO
        }
    }

    current = bishops | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let bishop = 1 << from;
        current = current & !bishop;
        let mut moves = get_bishop_moves(&bishop, &all_pieces, &friendly);
        while moves != 0 {
            let to = moves.trailing_zeros() as u8;
            moves = moves & !(1 << to);
            result.push(Move { from, to, promotion: false, capture: false, castling: false }); // TODO
        }
    }

    let from = king.trailing_zeros() as u8;
    let mut moves = get_king_moves(&king, &(empty | enemy));
    while moves != 0 {
        let to = moves.trailing_zeros() as u8;
        moves = moves & !(1 << to);
        result.push(Move { from, to, promotion: false, capture: false, castling: false }); // TODO
    }

    result
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

pub fn get_rook_moves(rook: &u64, occupied: &u64, friendly: &u64) -> u64 {
    let mut result: u64 = 0;
    let sq = rook.trailing_zeros() as usize;

    let north_attack = ROOK_RAYS[NORTH][sq];
    let north_blocker = north_attack & occupied;
    if north_blocker != 0 {
        let block_sq = north_blocker.trailing_zeros() as usize;
        let attacks = north_attack ^ ROOK_RAYS[NORTH][block_sq];
        result |= attacks;
    } else {
        result |= north_attack;
    }
    
    let east_attack = ROOK_RAYS[EAST][sq];
    let east_blocker = east_attack & occupied;
    if east_blocker != 0 {
        let block_sq = east_blocker.trailing_zeros() as usize;
        let attacks = east_attack ^ ROOK_RAYS[EAST][block_sq];
        result |= attacks;
    } else {
        result |= east_attack;
    }

    let south_attack = ROOK_RAYS[SOUTH][sq];
    let south_blocker = south_attack & occupied;
    if south_blocker != 0 {
        let block_sq = (63 - south_blocker.leading_zeros()) as usize;
        let attacks = south_attack ^ ROOK_RAYS[SOUTH][block_sq];
        result |= attacks;
    } else {
        result |= south_attack;
    }

    let west_attack = ROOK_RAYS[WEST][sq];
    let west_blocker = west_attack & occupied;
    if west_blocker != 0 {
        let block_sq = (63 - west_blocker.leading_zeros()) as usize;
        let attacks = west_attack ^ ROOK_RAYS[WEST][block_sq];
        result |= attacks;
    } else {
        result |= west_attack;
    }
    result ^ (result & friendly)
}

pub fn get_bishop_moves(bishop: &u64, occupied: &u64, friendly: &u64) -> u64 {
    let mut result: u64 = 0;
    let sq = bishop.trailing_zeros() as usize;

    let ne_attack = BISHOP_RAYS[NORTHEAST-4][sq];
    let ne_blocker = ne_attack & occupied;
    if ne_blocker != 0 {
        let block_sq = ne_blocker.trailing_zeros() as usize;
        let attacks = ne_attack ^ BISHOP_RAYS[NORTHEAST-4][block_sq];
        result |= attacks;
    } else {
        result |= ne_attack;
    }
    
    let nw_attack = BISHOP_RAYS[NORTHWEST-4][sq];
    let ne_blocker = nw_attack & occupied;
    if ne_blocker != 0 {
        let block_sq = ne_blocker.trailing_zeros() as usize;
        let attacks = nw_attack ^ BISHOP_RAYS[NORTHWEST-4][block_sq];
        result |= attacks;
    } else {
        result |= nw_attack;
    }

    let se_attack = BISHOP_RAYS[SOUTHEAST-4][sq];
    let se_blocker = se_attack & occupied;
    if se_blocker != 0 {
        let block_sq = (63 - se_blocker.leading_zeros()) as usize;
        let attacks = se_attack ^ BISHOP_RAYS[SOUTHEAST-4][block_sq];
        result |= attacks;
    } else {
        result |= se_attack;
    }

    let sw_attack = BISHOP_RAYS[SOUTHWEST-4][sq];
    let sw_blocker = sw_attack & occupied;
    if sw_blocker != 0 {
        let block_sq = (63 - sw_blocker.leading_zeros()) as usize;
        let attacks = sw_attack ^ BISHOP_RAYS[SOUTHWEST-4][block_sq];
        result |= attacks;
    } else {
        result |= sw_attack;
    }
    result ^ (result & friendly)
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
