use once_cell::sync::Lazy;
use regex::Regex;

use crate::Color;

use super::BitBoard;

const NOT_A_FILE: u64 =  0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
const NOT_AB_FILE: u64 = 0b00111111_00111111_00111111_00111111_00111111_00111111_00111111_00111111;
const NOT_H_FILE: u64 =  0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
const NOT_GH_FILE: u64 = 0b11111100_11111100_11111100_11111100_11111100_11111100_11111100_11111100;
const RANK_4: u64 =      0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
const RANK_5: u64 =      0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
const WHITE_PROMOTION: u64 =      0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
const BLACK_PROMOTION: u64 =      0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;


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


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn, Knight, Rook, Bishop, Queen, King,
}

pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<Piece>,
    pub capture: Option<Piece>,
    pub castling: bool,
    pub piece: Piece,
}

pub fn get_possible_moves(board: &BitBoard, color: &Color) -> Vec<Move> {
    let (pawns, knights, bishops, rooks, queens, king, enemy, promotion_rank) = match color {
        Color::Black => (board.black_pawns, board.black_knights, board.black_bishops, board.black_rooks, board.black_queens, board.black_king, 
                       board.get_white(), BLACK_PROMOTION), 
        Color::White => (board.white_pawns, board.white_knights, board.white_bishops, board.white_rooks, board.white_queens, board.white_king,
                       board.get_black(), WHITE_PROMOTION),
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
            let target = 1 << to;
            moves = moves & !target;
            let capture = board.check_capture(&target, color);
            result.push(Move { from, to, promotion: None, capture, castling: false, piece: Piece::Knight });
        }
    }

    current = rooks | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let rook = 1 << from;
        let piece = match rook & rooks != 0 {
            true => Piece::Rook,
            false => Piece::Queen,
        };
        current = current & !rook;
        let mut moves = get_rook_moves(&rook, &all_pieces, &friendly);
        while moves != 0 {
            let to = moves.trailing_zeros() as u8;
            let target = 1 << to;
            moves = moves & !target;
            let capture = board.check_capture(&target, color);
            result.push(Move { from, to, promotion: None, capture, castling: false, piece });
        }
    }

    current = bishops | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let bishop = 1 << from;
        let piece = match bishop & bishops != 0 {
            true => Piece::Bishop,
            false => Piece::Queen,
        };
        current = current & !bishop;
        let mut moves = get_bishop_moves(&bishop, &all_pieces, &friendly);
        while moves != 0 {
            let to = moves.trailing_zeros() as u8;
            let target = 1 << to;
            moves = moves & !target;
            let capture = board.check_capture(&target, color);
            result.push(Move { from, to, promotion: None, capture, castling: false, piece });
        }
    }

    let from = king.trailing_zeros() as u8;
    let mut moves = get_king_moves(&king, &(empty | enemy));
    while moves != 0 {
        let to = moves.trailing_zeros() as u8;
        let target = 1 << to;
        moves = moves & !target;
        let capture = board.check_capture(&target, color);
        result.push(Move { from, to, promotion: None, capture, castling: false, piece: Piece::King });
    }

    if color == &Color::White {
        let mut single = get_white_pawn_single_pushes(&pawns, &empty);
        while single != 0 {
            let to = single.trailing_zeros() as u8;
            let from = to - 8;
            let pawn = 1 << to;
            single = single & !pawn;
            if pawn & promotion_rank == 0 {
                result.push(Move { from, to, promotion: None, capture: None, castling: false, piece: Piece::Pawn }); // TODO: promotion
            } else {
                result.push(Move { from, to, promotion: Some(Piece::Knight), capture: None, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Bishop), capture: None, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Rook), capture: None, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Queen), capture: None, castling: false, piece: Piece::Pawn });
            }
        }

        let mut double = get_white_pawn_double_pushes(&pawns, &empty);
        while double != 0 {
            let to = double.trailing_zeros() as u8;
            let from = to - 16;
            let pawn = 1 << to;
            double = double & !pawn;
            result.push(Move { from, to, promotion: None, capture: None, castling: false, piece: Piece::Pawn });
        }

        // TODO: enpassant
        let mut east_captures = get_white_pawn_east_attacks(&pawns, &enemy);
        while east_captures != 0 {
            let to = east_captures.trailing_zeros() as u8;
            let from = to - 9;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            east_captures = east_captures & !pawn;
            if pawn & promotion_rank == 0 {
                result.push(Move { from, to, promotion: None, capture, castling: false, piece: Piece::Pawn });
            } else {
                result.push(Move { from, to, promotion: Some(Piece::Knight), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Bishop), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Rook), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Queen), capture, castling: false, piece: Piece::Pawn });
            }
        }

        let mut west_captures = get_white_pawn_west_attacks(&pawns, &enemy);
        while west_captures != 0 {
            let to = west_captures.trailing_zeros() as u8;
            let from = to - 7;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            west_captures = west_captures & !pawn;
            if pawn & promotion_rank == 0 {
                result.push(Move { from, to, promotion: None, capture, castling: false, piece: Piece::Pawn });
            } else {
                result.push(Move { from, to, promotion: Some(Piece::Knight), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Bishop), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Rook), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Queen), capture, castling: false, piece: Piece::Pawn });
            }
        }
    } else {
        let mut single = get_black_pawn_single_pushes(&pawns, &empty);
        while single != 0 {
            let to = single.trailing_zeros() as u8;
            let from = to + 8;
            let pawn = 1 << to;
            single = single & !pawn;
            if pawn & promotion_rank == 0 {
                result.push(Move { from, to, promotion: None, capture: None, castling: false, piece: Piece::Pawn });
            } else {
                result.push(Move { from, to, promotion: Some(Piece::Knight), capture: None, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Bishop), capture: None, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Rook), capture: None, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Queen), capture: None, castling: false, piece: Piece::Pawn });
            }
        }

        let mut double = get_black_pawn_double_pushes(&pawns, &empty);
        while double != 0 {
            let to = double.trailing_zeros() as u8;
            let from = to + 16;
            let pawn = 1 << to;
            double = double & !pawn;
            result.push(Move { from, to, promotion: None, capture: None, castling: false, piece: Piece::Pawn });
        }

        // TODO: enpassant
        let mut east_captures = get_black_pawn_east_attacks(&pawns, &enemy);
        while east_captures != 0 {
            let to = east_captures.trailing_zeros() as u8;
            let from = to + 9;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            east_captures = east_captures & !pawn;
            if pawn & promotion_rank == 0 {
                result.push(Move { from, to, promotion: None, capture, castling: false, piece: Piece::Pawn });
            } else {
                result.push(Move { from, to, promotion: Some(Piece::Knight), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Bishop), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Rook), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Queen), capture, castling: false, piece: Piece::Pawn });
            }
        }

        let mut west_captures = get_black_pawn_west_attacks(&pawns, &enemy);
        while west_captures != 0 {
            let to = west_captures.trailing_zeros() as u8;
            let from = to + 7;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            west_captures = west_captures & !pawn;
            if pawn & promotion_rank == 0 {
                result.push(Move { from, to, promotion: None, capture, castling: false, piece: Piece::Pawn });
            } else {
                result.push(Move { from, to, promotion: Some(Piece::Knight), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Bishop), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Rook), capture, castling: false, piece: Piece::Pawn });
                result.push(Move { from, to, promotion: Some(Piece::Queen), capture, castling: false, piece: Piece::Pawn });
            }
        }
    }
    // TODO: castling


    result
}

pub fn get_knight_moves(knight: &u64, targets: &u64) -> u64 {
    let mut result: u64 = 0;
    result = result | ((knight << 17) & NOT_H_FILE);
    result = result | ((knight << 10) & NOT_GH_FILE);
    result = result | ((knight >> 6) & NOT_GH_FILE);
    result = result | ((knight >> 15) & NOT_H_FILE);
    result = result | ((knight << 15) & NOT_A_FILE);
    result = result | ((knight << 6) & NOT_AB_FILE);
    result = result | ((knight >> 10) & NOT_AB_FILE);
    result = result | ((knight >> 17) & NOT_A_FILE);
    result & targets
}

pub fn get_king_moves(king: &u64, targets: &u64) -> u64 {
    let mut result: u64 = 0;
    result = result | (king << 8);
    result = result | (king >> 8);
    result = result | ((king << 1) & NOT_H_FILE);
    result = result | ((king << 9) & NOT_H_FILE);
    result = result | ((king >> 7) & NOT_H_FILE);
    result = result | ((king >> 1) & NOT_A_FILE);
    result = result | ((king >> 9) & NOT_A_FILE);
    result = result | ((king << 7) & NOT_A_FILE);
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
    (pawns << 9) & NOT_H_FILE & targets
}

pub fn get_white_pawn_west_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns << 7) & NOT_A_FILE & targets
}

pub fn get_black_pawn_east_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns >> 7) & NOT_H_FILE & targets
}

pub fn get_black_pawn_west_attacks(pawns: &u64, targets: &u64) -> u64 {
    (pawns >> 9) & NOT_A_FILE & targets
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

pub fn is_game_won(_board: &BitBoard, _color: &Color) -> bool {
    false
}

pub fn is_game_drawn(_board: &BitBoard, _color: &Color) -> bool {
    false
}

pub enum GameState {
    Normal, Check, Checkmate, Stalemate,
}

pub fn game_state(board: &mut BitBoard, color: &Color) -> GameState {
    let opp_color = color.opposite(); 

    let captures = get_capture_map(&board, color);
    let king = board.get_bitboard_by_piece(&Piece::King, &opp_color);
    let check = king & captures != 0;

    let moves = get_possible_moves(board, &opp_color);
    let mut no_moves = true;
    for mov in moves {
        board.apply_move(&mov, &opp_color);
        let king = board.get_bitboard_by_piece(&Piece::King, &opp_color);
        let captures = get_capture_map(board, color);
        let no_check = king & captures == 0;
        board.apply_move(&mov, &opp_color);
        if no_check {
            no_moves = false;
            break;
        }
    }

    match (check, no_moves) {
        (true, true) => GameState::Checkmate,
        (true, false) => GameState::Check,
        (false, true) => GameState::Stalemate,
        (false, false) => GameState::Normal,
    }
}

pub fn move_to_string(board: &mut BitBoard, mov: &Move, color: &Color, check: bool, checkmate:bool) -> String {
    // TODO: castling
    let have_capture = mov.capture.is_some();

    let mut candidates = get_moves_from(board, &mov.piece, have_capture, mov.to, color);

    let opp_color = color.opposite();

    let mut cand = Vec::new();
    while candidates != 0 {
        let from = candidates.trailing_zeros() as u8;
        let piece_from = 1 << from as u8;
        candidates = candidates & !piece_from;
        let mov = Move { from, to: mov.to, promotion: mov.promotion, capture: mov.capture, castling: false, piece: mov.piece }; // TODO
        board.apply_move(&mov, color);
        let captures = get_capture_map(&board, &opp_color);
        let king = board.get_king_by_color(color);
        board.apply_move(&mov, color);
        if from == mov.from {
            continue;
        }
        if king & captures == 0 {
            cand.push(from)
        };
    }
    let need_file = cand.iter().find(|&&m| m % 8 == mov.from % 8).is_some();
    let need_rank = cand.iter().find(|&&m| m / 8 == mov.from / 8).is_some();

    let mut move_str = String::new();

    if mov.piece != Piece::Pawn {
        move_str.push(piece_to_letter(mov.piece));
    }

    if need_file {
        move_str.push(num_to_file(mov.from));
    }
    if need_rank {
        move_str.push(num_to_rank(mov.from));
    }

    if have_capture {
        move_str.push('x');
    }

    move_str.push(num_to_file(mov.to));
    move_str.push(num_to_rank(mov.to));

    if let Some(promotion) = mov.promotion {
        move_str.push('=');
        move_str.push(piece_to_letter(promotion));
    }
   
    if check & !checkmate {
        move_str.push('+');
    }
    if checkmate {
        move_str.push('#');
    }
    // TODO: add enpassant ( e.p.)

    move_str
}

fn piece_to_letter(piece: Piece) -> char {
    match piece {
        Piece::Pawn => 'P',
        Piece::Knight => 'N',
        Piece::Bishop => 'B',
        Piece::Rook => 'R',
        Piece::Queen => 'Q',
        Piece::King => 'K',
    }
}

fn num_to_rank(pos: u8) -> char {
    (b'1' + (pos / 8)) as char
}

fn num_to_file(pos: u8) -> char {
    (b'a' + 7 - (pos % 8)) as char
}

pub fn string_to_move(board: &mut BitBoard, mov: String, color: &Color) -> Result<Move, String> {
    let pattern = r"([KQRBN]?)([a-h])?([1-8])?(x)?([a-h][1-8])(=[QRBN])?( e\.p\.)?";

    let re = Regex::new(pattern).unwrap();

    match re.captures(mov.as_str()) {
        Some(caps) => {
            let piece = caps.get(1).map_or(Piece::Pawn, |m| letter_to_piece(m.as_str()));
            let from_file = caps.get(2).map_or(None, |m| Some(m.as_str()));
            let from_rank = caps.get(3).map_or(None, |m| Some(m.as_str()));
            let have_capture = caps.get(4).map_or(false, |_m| true);
            let to = caps.get(5).map_or(0, |m| field_to_num(m.as_str()));
            let promotion = caps.get(6).map_or(None, |m| Some(letter_to_piece(&m.as_str()[1..])));
            let enpassant = caps.get(7).map_or(false, |_m| true);

            if let Some(promote_to) = promotion {
                if piece != Piece::Pawn {
                        return Err("Cannot promote piece".into());
                }
                if promote_to == Piece::Pawn || promote_to == Piece::King {
                        return Err("Cannot promote to this piece".into());
                }
                let correct_rank = match color {
                    Color::White => 1 << to as u8 & WHITE_PROMOTION != 0,
                    Color::Black => 1 << to as u8  & BLACK_PROMOTION != 0,
                };
                if !correct_rank {
                        return Err("Pawn not on a promotion rank".into());
                }
            }

            let mut candidates = match (from_file, from_rank) {
                (Some(file), Some(rank)) => {
                    let cand = 1 << field_to_num(format!("{}{}", file, rank).as_str());
                    let orig = board.check_capture(&cand, &color.opposite());
                    if orig != Some(piece) {
                        return Err("Wrong start square".into());
                    }
                    cand
                },
                (Some(file), None) => {
                    let file = file_to_num(file.chars().next().unwrap());
                    let move_map = get_moves_from(board, &piece, have_capture, to, color);
                    let file_map = ROOK_RAYS[NORTH][0+file as usize] | (1 << (0+file));
                    move_map & file_map
                },
                (None, Some(rank)) => {
                    let rank = rank_to_num(rank.chars().next().unwrap());
                    let move_map = get_moves_from(board, &piece, have_capture, to, color);
                    let rank_map = ROOK_RAYS[EAST][(rank as usize)*8 + 0] | (1 << (0+rank*8));
                    move_map & rank_map
                },
                (None, None) => {
                    get_moves_from(board, &piece, have_capture, to, color)
                },
            };

            let mut current = candidates;
            let capture = board.check_capture(&(1<<to), color);
            if (capture.is_none() && have_capture) || (capture.is_some() && !have_capture) {
                return Err("Capture flag corrupted!".into())
            };

            let opp_color = color.opposite();

            let capture_my = board.check_capture(&(1<<to), &opp_color);
            if capture_my.is_some() {
                return Err("Cannot move to field occupied by your own piece!".into())
            }

            while current != 0 {
                let from = current.trailing_zeros() as u8;
                let piece_from = 1 << from as u8;
                current = current & !piece_from;
                let mov = Move { from, to, promotion, capture, castling: false, piece }; // TODO
                board.apply_move(&mov, color);
                let captures = get_capture_map(&board, &opp_color);
                let king = board.get_king_by_color(color);
                board.apply_move(&mov, color);
                if king & captures != 0 {
                    candidates ^= piece_from;
                };
            }

            if candidates.count_ones() > 1 {
                return Err("Ambiguous starting position!".into())
            }
            if candidates == 0 {
                return Err("No such piece!".into())
            }
            let from = candidates.trailing_zeros() as u8;

            println!("({}) {:?}, from {} to {}, capture: {:?}, promotion: {:?}, enpassant: {}", mov, piece, from, to, capture, promotion, enpassant);
            Ok(Move {
                from, to, promotion, capture, castling: false, piece,
            })
        },
        None => {
            println!("No match found.");
            Err("Cannot parse move".into())
        },
    }
}

fn letter_to_piece(letter: &str) -> Piece {
    match letter {
        "K" => Piece::King,
        "Q" => Piece::Queen,
        "R" => Piece::Rook,
        "B" => Piece::Bishop,
        "N" => Piece::Knight,
        _ => Piece::Pawn,
    }
}

fn rank_to_num(rank: char) -> u8 {
    rank as u8 - b'1'
}

fn file_to_num(file: char) -> u8 {
    7 - (file as u8 - b'a')
}

pub fn field_to_num(field: &str) -> u8 {
    let mut c = field.chars();
    let file = c.next().unwrap();
    let rank = c.next().unwrap();
    let file = file_to_num(file);
    let rank = rank_to_num(rank);
    rank*8 + file
}

fn get_moves_from(board: &BitBoard, piece: &Piece, capture: bool, to: u8, color: &Color) -> u64 {
    let (pawns, knights, bishops, rooks, queens, king, enemy) = match color {
        Color::Black => (board.black_pawns, board.black_knights, board.black_bishops, board.black_rooks, board.black_queens, board.black_king, 
                       board.get_white()),
        Color::White => (board.white_pawns, board.white_knights, board.white_bishops, board.white_rooks, board.white_queens, board.white_king,
                       board.get_black()),
    };
    let my = pawns | knights | bishops | rooks | queens | king;
    match (piece, capture) {
        (Piece::Pawn, true) => {
            match color {
                Color::White => {
                    let west = get_white_pawn_west_attacks(&pawns, &enemy);
                    let west = (west & (1<<to)) >> 7;
                    let east = get_white_pawn_east_attacks(&pawns, &enemy);
                    let east = (east & (1<<to)) >> 9;
                    east | west
                },
                Color::Black => {
                    let west = get_black_pawn_west_attacks(&pawns, &enemy);
                    let west = (west & (1<<to)) << 9;
                    let east = get_black_pawn_east_attacks(&pawns, &enemy);
                    let east = (east & (1<<to)) << 7;
                    east | west
                },
            }
        },
        (Piece::Pawn, false) => {
            match color {
                Color::White => {
                    let empty = !(my|enemy);
                    let single = get_white_pawn_single_pushes(&pawns, &empty);
                    let single = (single & (1<<to)) >> 8;
                    let double = get_white_pawn_double_pushes(&pawns, &empty);
                    let double = (double & (1<<to)) >> 16;
                    single | double
                },
                Color::Black => {
                    let empty = !(my|enemy);
                    let single = get_black_pawn_single_pushes(&pawns, &empty);
                    let single = (single & (1<<to)) << 8;
                    let double = get_black_pawn_double_pushes(&pawns, &empty);
                    let double = (double & (1<<to)) << 16;
                    single | double
                },
            }
        }, 
        (Piece::Knight, _) => {
            get_knight_moves(&(1<<to), &knights)
        },
        (Piece::Bishop, _) => {
            get_bishop_moves(&(1<<to), &(enemy|my), &enemy) & bishops
        },
        (Piece::Rook, _) => {
            get_rook_moves(&(1<<to), &(enemy|my), &enemy) & rooks
        },
        (Piece::Queen, _) => {
            let bishop = get_bishop_moves(&(1<<to), &(enemy|my), &enemy);
            let rook = get_rook_moves(&(1<<to), &(enemy|my), &enemy);
            (bishop | rook) & queens
        },
        (Piece::King, _) => {
            get_king_moves(&(1<<to), &king)
        },
    }
}

pub fn get_capture_map(board: &BitBoard, color: &Color) -> u64 {
    let (pawns, knights, bishops, rooks, queens, king, enemy) = match color {
        Color::Black => (board.black_pawns, board.black_knights, board.black_bishops, board.black_rooks, board.black_queens, board.black_king, board.get_white()),
        Color::White => (board.white_pawns, board.white_knights, board.white_bishops, board.white_rooks, board.white_queens, board.white_king, board.get_black()),
    };
    let friendly = pawns | knights | bishops | rooks | queens | king;
    let all_pieces = friendly | enemy;
    let empty = !all_pieces;

    let mut result = 0;

    result |= get_knight_moves(&knights, &(empty | enemy));
    result |= get_king_moves(&king, &(empty | enemy));

    if color == &Color::White {
        // TODO: enpassant
        result |= get_white_pawn_east_attacks(&pawns, &enemy);
        result |= get_white_pawn_west_attacks(&pawns, &enemy);
    } else {
        // TODO: enpassant
        result |= get_black_pawn_east_attacks(&pawns, &enemy);
        result |= get_black_pawn_west_attacks(&pawns, &enemy);
    }

    let mut current = rooks | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let rook = 1 << from;
        current = current ^ rook;
        result |= get_rook_moves(&rook, &all_pieces, &friendly);
    }

    current = bishops | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let bishop = 1 << from;
        current = current ^ bishop;
        result |= get_bishop_moves(&bishop, &all_pieces, &friendly);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{engine::generate_board_from_fen, print_bitboard, print_board};

    use super::*;

    fn get_for_knight(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let targets = !board.get_black() | board.get_white();
        let knights = match color {
            Color::White => board.white_knights,
            Color::Black => board.black_knights,
        };
        let moves = get_knight_moves(&knights, &targets);
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_knight_moves_no_capture() {
        assert_eq!(get_for_knight("8/8/8/3N4/8/8/8/8", Color::White), 0x0028440044280000);
        assert_eq!(get_for_knight("8/8/8/8/3n4/8/8/8", Color::Black), 0x0000284400442800);
        assert_eq!(get_for_knight("8/3N4/8/8/8/8/8/8", Color::White), 0x4400442800000000);
        assert_eq!(get_for_knight("8/1N6/8/8/8/8/8/8", Color::White), 0x100010a000000000);
        assert_eq!(get_for_knight("8/6N1/8/8/8/8/8/8", Color::White), 0x0800080500000000);
    }

    fn get_for_king(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let targets = !board.get_black() | board.get_white();
        let king = match color {
            Color::White => board.white_king,
            Color::Black => board.black_king,
        };
        let moves = get_king_moves(&king, &targets);
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_king_moves_no_capture() {
        assert_eq!(get_for_king("8/8/8/3K4/8/8/8/8", Color::White), 0x0000382838000000);
        assert_eq!(get_for_king("8/8/8/8/3k4/8/8/8", Color::Black), 0x0000003828380000);
        assert_eq!(get_for_king("8/3K4/8/8/8/8/8/8", Color::White), 0x3828380000000000);
        assert_eq!(get_for_king("8/K7/8/8/8/8/8/8", Color::White), 0xc040c00000000000);
        assert_eq!(get_for_king("8/7K/8/8/8/8/8/8", Color::White), 0x0302030000000000);
    }

    fn get_single_pushes(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let targets = !board.get_black() & !board.get_white();
        let moves = match color {
            Color::White => get_white_pawn_single_pushes(&board.white_pawns, &targets),
            Color::Black => get_black_pawn_single_pushes(&board.black_pawns, &targets),
        };
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_pawn_single_pushes() {
        assert_eq!(get_single_pushes("8/8/8/8/8/8/P7/8", Color::White), 0x0000000000800000);
        assert_eq!(get_single_pushes("8/p7/8/8/8/8/8/8", Color::Black), 0x0000800000000000);
        assert_eq!(get_single_pushes("8/8/8/8/8/K7/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_single_pushes("8/8/8/8/8/k7/P7/8", Color::White), 0x0000000000000000);
    }

    fn get_double_pushes(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let targets = !board.get_black() & !board.get_white();
        let moves = match color {
            Color::White => get_white_pawn_double_pushes(&board.white_pawns, &targets),
            Color::Black => get_black_pawn_double_pushes(&board.black_pawns, &targets),
        };
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_pawn_double_pushes() {
        assert_eq!(get_double_pushes("8/8/8/8/8/8/P7/8", Color::White), 0x0000000080000000);
        assert_eq!(get_double_pushes("8/p7/8/8/8/8/8/8", Color::Black), 0x0000008000000000);
        assert_eq!(get_double_pushes("8/8/8/8/8/K7/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_double_pushes("8/8/8/8/8/k7/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_double_pushes("8/8/8/8/K7/8/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_double_pushes("8/8/8/8/k7/8/P7/8", Color::White), 0x0000000000000000);
    }

    fn get_east_captures(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let targets = match color {
            Color::White => board.get_black(),
            Color::Black => board.get_white(),
        };
        let moves = match color {
            Color::White => get_white_pawn_east_attacks(&board.white_pawns, &targets),
            Color::Black => get_black_pawn_east_attacks(&board.black_pawns, &targets),
        };
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_pawn_east_captures() {
        assert_eq!(get_east_captures("8/8/8/8/8/4n3/3P4/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/2n5/3P4/8", Color::White), 0x0000000000200000);
        assert_eq!(get_east_captures("8/8/8/8/8/1n6/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/7n/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/4N3/3P4/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/2N5/3P4/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/8/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/3P4/4n3/8", Color::White), 0x0000000000000000);
        assert_eq!(get_east_captures("8/8/8/8/8/3P4/2n5/8", Color::White), 0x0000000000000000);

        assert_eq!(get_east_captures("8/3p4/4N3/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/2p4/2N5/8/8/8/8/8", Color::Black), 0x0000400000000000);
        assert_eq!(get_east_captures("8/p7/1N6/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/p7/7N/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/3p4/4n3/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/3p4/2n5/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/p7/8/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/4N3/3p4/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_east_captures("8/2N5/3p4/8/8/3P8/8/8", Color::Black), 0x0000000000000000);
    }

    fn get_west_captures(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let targets = match color {
            Color::White => board.get_black(),
            Color::Black => board.get_white(),
        };
        let moves = match color {
            Color::White => get_white_pawn_west_attacks(&board.white_pawns, &targets),
            Color::Black => get_black_pawn_west_attacks(&board.black_pawns, &targets),
        };
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_pawn_west_captures() {
        assert_eq!(get_west_captures("8/8/8/8/8/4n3/3P4/8", Color::White), 0x0000000000080000);
        assert_eq!(get_west_captures("8/8/8/8/8/2n5/3P4/8", Color::White), 0x0000000000000000);
        assert_eq!(get_west_captures("8/8/8/8/8/1n6/P7/8", Color::White), 0x0000000000400000);
        assert_eq!(get_west_captures("8/8/8/8/8/7n/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_west_captures("8/8/8/8/8/4N3/3P4/8", Color::White), 0x0000000000000000);
        assert_eq!(get_west_captures("8/8/8/8/8/2N5/3P4/8", Color::White), 0x0000000000000000);
        assert_eq!(get_west_captures("8/8/8/8/8/8/P7/8", Color::White), 0x0000000000000000);
        assert_eq!(get_west_captures("8/8/8/8/8/3P4/4n3/8", Color::White), 0x0000000000000000);
        assert_eq!(get_west_captures("8/8/8/8/8/3P4/2n5/8", Color::White), 0x0000000000000000);

        assert_eq!(get_west_captures("8/3p4/4N3/8/8/8/8/8", Color::Black), 0x0000080000000000);
        assert_eq!(get_west_captures("8/2p4/2N5/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_west_captures("8/p7/1N6/8/8/8/8/8", Color::Black), 0x0000400000000000);
        assert_eq!(get_west_captures("8/p7/7N/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_west_captures("8/3p4/4n3/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_west_captures("8/3p4/2n5/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_west_captures("8/p7/8/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_west_captures("8/4N3/3p4/8/8/8/8/8", Color::Black), 0x0000000000000000);
        assert_eq!(get_west_captures("8/2N5/3p4/8/8/3P8/8/8", Color::Black), 0x0000000000000000);
    }

    fn get_for_rook(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let occupied = board.get_white() | board.get_black();
        let friendly = match color {
            Color::White => board.get_white(),
            Color::Black => board.get_black(),
        };
        let rooks = match color {
            Color::White => board.white_rooks,
            Color::Black => board.black_rooks,
        };
        let moves = get_rook_moves(&rooks, &occupied, &friendly);
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_rook_moves() {
        assert_eq!(get_for_rook("8/8/8/3R4/8/8/8/8", Color::White), 0x101010ef10101010);
        assert_eq!(get_for_rook("8/8/8/8/3r4/8/8/8", Color::Black), 0x10101010ef101010);
        assert_eq!(get_for_rook("8/3R4/8/8/8/8/8/8", Color::White), 0x10ef101010101010);
        assert_eq!(get_for_rook("8/R7/8/8/8/8/8/8", Color::White), 0x807f808080808080);
        assert_eq!(get_for_rook("8/7R/8/8/8/8/8/8", Color::White), 0x01fe010101010101);

        assert_eq!(get_for_rook("8/8/8/2nR4/8/8/8/8", Color::White), 0x1010102f10101010);
        assert_eq!(get_for_rook("8/3n4/8/3R4/8/8/8/8", Color::White), 0x001010ef10101010);
        assert_eq!(get_for_rook("8/8/3n4/3R4/8/8/8/8", Color::White), 0x000010ef10101010);
        assert_eq!(get_for_rook("8/8/8/3R4/8/2N5/8/8", Color::White), 0x101010ef10101010);
        assert_eq!(get_for_rook("8/8/8/3R4/8/3N4/8/8", Color::White), 0x101010ef10000000);

        assert_eq!(get_for_rook("8/8/8/2Nr4/8/8/8/8", Color::Black), 0x1010102f10101010);
        assert_eq!(get_for_rook("8/3N4/8/3r4/8/8/8/8", Color::Black), 0x001010ef10101010);
        assert_eq!(get_for_rook("8/8/3N4/3r4/8/8/8/8", Color::Black), 0x000010ef10101010);
        assert_eq!(get_for_rook("8/8/8/3r4/8/2n5/8/8", Color::Black), 0x101010ef10101010);
        assert_eq!(get_for_rook("8/8/8/3r4/8/3n4/8/8", Color::Black), 0x101010ef10000000);
    }

    fn get_for_bishop(fen: &str, color: Color) -> u64 {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let occupied = board.get_white() | board.get_black();
        let friendly = match color {
            Color::White => board.get_white(),
            Color::Black => board.get_black(),
        };
        let bishops = match color {
            Color::White => board.white_bishops,
            Color::Black => board.black_bishops,
        };
        let moves = get_bishop_moves(&bishops, &occupied, &friendly);
        print_board(&board);
        println!("");
        print_bitboard(moves);
        println!("{:#018x}", moves);
        moves
    }

    #[test]
    fn test_bishop_moves() {
        assert_eq!(get_for_bishop("8/8/8/3B4/8/8/8/8", Color::White), 0x8244280028448201);
        assert_eq!(get_for_bishop("8/8/8/8/3b4/8/8/8", Color::Black), 0x0182442800284482);
        assert_eq!(get_for_bishop("8/3B4/8/8/8/8/8/8", Color::White), 0x2800284482010000);
        assert_eq!(get_for_bishop("8/B7/8/8/8/8/8/8", Color::White), 0x4000402010080402);
        assert_eq!(get_for_bishop("8/7B/8/8/8/8/8/8", Color::White), 0x0200020408102040);

        assert_eq!(get_for_bishop("8/8/2n5/3B4/8/8/8/8", Color::White), 0x0204280028448201);
        assert_eq!(get_for_bishop("8/5n2/8/3B4/8/8/8/8", Color::White), 0x8044280028448201);
        assert_eq!(get_for_bishop("8/8/2N5/3B4/8/8/8/8", Color::White), 0x0204080028448201);
        assert_eq!(get_for_bishop("8/5N2/8/3B4/8/8/8/8", Color::White), 0x8040280028448201);

        assert_eq!(get_for_bishop("8/8/2n5/3b4/8/8/8/8", Color::Black), 0x0204080028448201);
        assert_eq!(get_for_bishop("8/5n2/8/3b4/8/8/8/8", Color::Black), 0x8040280028448201);
        assert_eq!(get_for_bishop("8/8/2N5/3b4/8/8/8/8", Color::Black), 0x0204280028448201);
        assert_eq!(get_for_bishop("8/5N2/8/3b4/8/8/8/8", Color::Black), 0x8044280028448201);
    }

    #[test]
    fn test_possible_moves_count() {
        assert_eq!(get_possible_moves_count("8/8/8/3Q4/8/8/8/8", Color::White), 27);
        assert_eq!(get_possible_moves_count("8/8/8/8/3q4/8/8/8", Color::Black), 27);
        assert_eq!(get_possible_moves_count("8/8/8/3R4/8/8/8/8", Color::White), 14);
        assert_eq!(get_possible_moves_count("8/8/8/8/3r4/8/8/8", Color::Black), 14);
        assert_eq!(get_possible_moves_count("8/8/8/3B4/8/8/8/8", Color::White), 13);
        assert_eq!(get_possible_moves_count("8/8/8/8/3b4/8/8/8", Color::Black), 13);
        assert_eq!(get_possible_moves_count("8/8/8/3N4/8/8/8/8", Color::White), 8);
        assert_eq!(get_possible_moves_count("8/8/8/8/3n4/8/8/8", Color::Black), 8);
        assert_eq!(get_possible_moves_count("8/8/8/3K4/8/8/8/8", Color::White), 8);
        assert_eq!(get_possible_moves_count("8/8/8/8/3k4/8/8/8", Color::Black), 8);
        assert_eq!(get_possible_moves_count("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", Color::Black), 20);
        assert_eq!(get_possible_moves_count("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", Color::White), 20);
    }

    fn get_possible_moves_count(fen: &str, color: Color) -> usize {
        let board = generate_board_from_fen(&fen.to_string()).unwrap();
        let moves = get_possible_moves(&board, &color);
        moves.len()
    }

    #[test]
    fn test_pawn_move() {
        let mut board = generate_board_from_fen(&"8/8/8/8/8/8/4P3/8".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "e4".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("e2"));
        assert_eq!(mov.to, field_to_num("e4"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, None);
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Pawn);
    }

    #[test]
    fn test_knight_move() {
        let mut board = generate_board_from_fen(&"8/8/8/8/8/8/8/6N1".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "Ne2".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("g1"));
        assert_eq!(mov.to, field_to_num("e2"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, None);
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Knight);
    }

    #[test]
    fn test_capture_move() {
        let mut board = generate_board_from_fen(&"8/8/3p4/4P3/8/8/8/8".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "xd6".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("e5"));
        assert_eq!(mov.to, field_to_num("d6"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, Some(Piece::Pawn));
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Pawn);
    }

    #[test]
    fn test_knight_capture_move() {
        let mut board = generate_board_from_fen(&"8/8/8/3p4/8/4N3/8/8".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "Nxd5".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("e3"));
        assert_eq!(mov.to, field_to_num("d5"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, Some(Piece::Pawn));
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Knight);
    }

    #[test]
    fn test_knight_disambiguation_file() {
        let mut board = generate_board_from_fen(&"8/8/8/3N4/8/8/8/3N4".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "N5e3".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("d5"));
        assert_eq!(mov.to, field_to_num("e3"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, None);
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Knight);
    }

    #[test]
    fn test_knight_disambiguation_rank() {
        let mut board = generate_board_from_fen(&"8/8/8/8/2N3N1/8/8/8".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "Nce3".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("c4"));
        assert_eq!(mov.to, field_to_num("e3"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, None);
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Knight);
    }

    #[test]
    fn test_knight_disambiguation_file_and_rank() {
        let mut board = generate_board_from_fen(&"8/8/8/5N2/8/8/8/3N4".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "Nf5e3".to_string(), &Color::White);
        assert!(result.is_ok());
        let mov = result.unwrap();
        assert_eq!(mov.from, field_to_num("f5"));
        assert_eq!(mov.to, field_to_num("e3"));
        assert_eq!(mov.promotion, None);
        assert_eq!(mov.capture, None);
        assert_eq!(mov.castling, false);
        assert_eq!(mov.piece, Piece::Knight);
    }

    #[test]
    fn test_knight_nonsensical_move() {
        let mut board = generate_board_from_fen(&"8/8/8/5N2/8/8/8/3N4".to_string()).unwrap();
        print_board(&board);
        let result = string_to_move(&mut board, "Nh1h2".to_string(), &Color::White);
        assert!(result.is_err());
    }

    fn perft(depth: usize, board: &mut BitBoard, color: &Color) -> usize {
        if depth == 0 {
            return 1;
        }
        let mut nodes = 0;
        let moves = get_possible_moves(&board, &color);
        let opp_color = color.opposite();
        for mov in moves {
            board.apply_move(&mov, color);
            let captures = get_capture_map(board, &opp_color);
            let king = board.get_king_by_color(color);
            let no_check = king & captures == 0;
            if no_check {
                nodes += perft(depth-1, board, &opp_color);
            }
            board.apply_move(&mov, color);
        }
        return nodes;
    }

    fn perft_divide(depth: usize, board: &mut BitBoard, color: &Color) -> usize {
        assert!(depth > 0);
        let mut nodes = 0;
        let moves = get_possible_moves(board, color);
        let opp_color = color.opposite();

        for mov in moves {
            board.apply_move(&mov, color);
            let captures = get_capture_map(board, &opp_color);
            let king = board.get_king_by_color(color);
            let no_check = king & captures == 0;
            if no_check {
                let curr = perft(depth-1, board, &opp_color);
                nodes += curr;
                let from = format!("{}{}", num_to_file(mov.from), num_to_rank(mov.from));
                let to = format!("{}{}", num_to_file(mov.to), num_to_rank(mov.to));
                println!("Move: {}{}, Nodes: {}", from, to, curr);
            }
            board.apply_move(&mov, color);
        }
        return nodes;
    }

    #[test]
    fn test_perft1_for_initial_position() {
        let mut board = generate_board_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();
        let perft = perft(1, &mut board, &Color::White);
        let target = 20;
        assert_eq!(perft, target, "perft1 should be {}, but is {}", target, perft);
    }

    #[test]
    fn test_perft2_for_initial_position() {
        let mut board = generate_board_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();
        let perft = perft(2, &mut board, &Color::White);
        let target = 400;
        assert_eq!(perft, target, "perft2 should be {}, but is {}", target, perft);
    }

    #[test]
    fn test_perft3_for_initial_position() {
        let mut board = generate_board_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();
        let perft = perft(3, &mut board, &Color::White);
        let target = 8902;
        assert_eq!(perft, target, "perft3 should be {}, but is {}", target, perft);
    }

    #[test]
    fn test_perft4_for_initial_position() {
        let mut board = generate_board_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();
        let perft = perft_divide(4, &mut board, &Color::White);
        let target = 197281;
        assert_eq!(perft, target, "perft4 should be {}, but is {}", target, perft);
    }

    #[test]
    #[ignore = "enpassant is not yet implemented"]
    fn test_perft5_for_initial_position() {
        let mut board = generate_board_from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap();
        let perft = perft(5, &mut board, &Color::White);
        let target = 4865609 ;
        assert_eq!(perft, target, "perft5 should be {}, but is {}", target, perft);
    }
}
