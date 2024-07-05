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
    pub promotion: bool,
    pub capture: Option<Piece>,
    pub castling: bool,
    pub piece: Piece,
}

pub fn get_possible_moves(board: &BitBoard, color: &Color) -> Vec<Move> {
    let (pawns, knights, bishops, rooks, queens, king, enemy) = match color {
        Color::Black => (board.black_pawns, board.black_knights, board.black_bishops, board.black_rooks, board.black_queens, board.black_king, 
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
            let target = 1 << to;
            moves = moves & !target;
            let capture = board.check_capture(&target, color);
            result.push(Move { from, to, promotion: false, capture, castling: false, piece: Piece::Knight }); // TODO
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
            result.push(Move { from, to, promotion: false, capture, castling: false, piece }); // TODO
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
            result.push(Move { from, to, promotion: false, capture, castling: false, piece }); // TODO
        }
    }

    let from = king.trailing_zeros() as u8;
    let mut moves = get_king_moves(&king, &(empty | enemy));
    while moves != 0 {
        let to = moves.trailing_zeros() as u8;
        let target = 1 << to;
        moves = moves & !target;
        let capture = board.check_capture(&target, color);
        result.push(Move { from, to, promotion: false, capture, castling: false, piece: Piece::King }); // TODO
    }

    if color == &Color::White {
        let mut single = get_white_pawn_single_pushes(&pawns, &empty);
        while single != 0 {
            let to = single.trailing_zeros() as u8;
            let from = to - 8;
            let pawn = 1 << to;
            single = single & !pawn;
            result.push(Move { from, to, promotion: false, capture: None, castling: false, piece: Piece::Pawn }); // TODO
        }

        let mut double = get_white_pawn_double_pushes(&pawns, &empty);
        while double != 0 {
            let to = double.trailing_zeros() as u8;
            let from = to - 16;
            let pawn = 1 << to;
            double = double & !pawn;
            result.push(Move { from, to, promotion: false, capture: None, castling: false, piece: Piece::Pawn }); // TODO
        }

        // TODO: enpassant
        let mut east_captures = get_white_pawn_east_attacks(&pawns, &enemy);
        while east_captures != 0 {
            let to = east_captures.trailing_zeros() as u8;
            let from = to - 9;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            east_captures = east_captures & !pawn;
            result.push(Move { from, to, promotion: false, capture, castling: false, piece: Piece::Pawn }); // TODO
        }

        let mut west_captures = get_white_pawn_west_attacks(&pawns, &enemy);
        while west_captures != 0 {
            let to = west_captures.trailing_zeros() as u8;
            let from = to - 7;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            west_captures = west_captures & !pawn;
            result.push(Move { from, to, promotion: false, capture, castling: false, piece: Piece::Pawn }); // TODO
        }
    } else {
        let mut single = get_black_pawn_single_pushes(&pawns, &empty);
        while single != 0 {
            let to = single.trailing_zeros() as u8;
            let from = to + 8;
            let pawn = 1 << to;
            single = single & !pawn;
            result.push(Move { from, to, promotion: false, capture: None, castling: false, piece: Piece::Pawn }); // TODO
        }

        let mut double = get_black_pawn_double_pushes(&pawns, &empty);
        while double != 0 {
            let to = double.trailing_zeros() as u8;
            let from = to + 16;
            let pawn = 1 << to;
            double = double & !pawn;
            result.push(Move { from, to, promotion: false, capture: None, castling: false, piece: Piece::Pawn }); // TODO
        }

        // TODO: enpassant
        let mut east_captures = get_black_pawn_east_attacks(&pawns, &enemy);
        while east_captures != 0 {
            let to = east_captures.trailing_zeros() as u8;
            let from = to + 9;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            east_captures = east_captures & !pawn;
            result.push(Move { from, to, promotion: false, capture, castling: false, piece: Piece::Pawn }); // TODO
        }

        let mut west_captures = get_black_pawn_west_attacks(&pawns, &enemy);
        while west_captures != 0 {
            let to = west_captures.trailing_zeros() as u8;
            let from = to + 7;
            let pawn = 1 << to;
            let capture = board.check_capture(&pawn, color);
            west_captures = west_captures & !pawn;
            result.push(Move { from, to, promotion: false, capture, castling: false, piece: Piece::Pawn }); // TODO
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

    // TODO: add promotion (=[QRBN])
   
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

            let mut candidates = match (from_file, from_rank) {
                (Some(file), Some(rank)) => {
                    1 << field_to_num(format!("{}{}", file, rank).as_str())
                },
                (Some(file), None) => {
                    let file = file_to_num(file.chars().next().unwrap());
                    let move_map = get_moves_from(board, &piece, have_capture, to, color);
                    let file_map = ROOK_RAYS[NORTH][0+file as usize] | (1 << (0+file));
                    move_map & file_map
                },
                (None, Some(rank)) => {
                    let rank = file_to_num(rank.chars().next().unwrap());
                    let move_map = get_moves_from(board, &piece, have_capture, to, color);
                    let rank_map = ROOK_RAYS[WEST][(rank as usize)*8 + 0] | (1 << (0+rank*8));
                    move_map & rank_map
                },
                (None, None) => {
                    get_moves_from(board, &piece, have_capture, to, color)
                },
            };

            let mut current = candidates;
            let capture = board.get_capture(&(1<<to), color);
            if (capture.is_none() && have_capture) || (capture.is_some() && !have_capture) {
                return Err("Capture flag corrupted!".into())
            };

            let opp_color = color.opposite();

            let capture = board.get_capture(&(1<<to), &opp_color);
            if capture.is_some() {
                return Err("Cannot move to field occupied by your own piece!".into())
            }

            while current != 0 {
                let from = current.trailing_zeros() as u8;
                let piece_from = 1 << from as u8;
                current = current & !piece_from;
                let mov = Move { from, to, promotion: false, capture, castling: false, piece }; // TODO
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
                from, to, promotion: false, capture, castling: false, piece,
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
                       board.white_pawns | board.white_knights | board.white_bishops | board.white_rooks | board.white_queens | board.white_king),
        Color::White => (board.white_pawns, board.white_knights, board.white_bishops, board.white_rooks, board.white_queens, board.white_king,
                       board.black_pawns | board.black_knights | board.black_bishops | board.black_rooks | board.black_queens | board.black_king),
    };
    let my = pawns | knights | bishops | rooks | queens | king;
    match (piece, capture) {
        (Piece::Pawn, true) => {
            match color {
                Color::White => {
                    let empty = !(my|enemy);
                    let west = get_white_pawn_west_attacks(&pawns, &empty);
                    let west = (west & (1<<to)) >> 7;
                    let east = get_white_pawn_east_attacks(&pawns, &empty);
                    let east = (east & (1<<to)) >> 9;
                    east | west
                },
                Color::Black => {
                    let empty = !(my|enemy);
                    let west = get_black_pawn_west_attacks(&pawns, &empty);
                    let west = (west & (1<<to)) << 9;
                    let east = get_black_pawn_east_attacks(&pawns, &empty);
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
            get_bishop_moves(&(1<<to), &(enemy|my), &enemy)
        },
        (Piece::Rook, _) => {
            get_rook_moves(&(1<<to), &(enemy|my), &enemy)
        },
        (Piece::Queen, _) => {
            let bishop = get_bishop_moves(&(1<<to), &(enemy|my), &enemy);
            let rook = get_rook_moves(&(1<<to), &(enemy|my), &enemy);
            bishop | rook
        },
        (Piece::King, _) => {
            get_king_moves(&(1<<to), &king)
        },
    }
}

pub fn get_capture_map(board: &BitBoard, color: &Color) -> u64 {
    let (pawns, knights, bishops, rooks, queens, king, enemy) = match color {
        Color::Black => (board.black_pawns, board.black_knights, board.black_bishops, board.black_rooks, board.black_queens, board.black_king, 
                       board.white_pawns | board.white_knights | board.white_bishops | board.white_rooks | board.white_queens | board.white_king),
        Color::White => (board.white_pawns, board.white_knights, board.white_bishops, board.white_rooks, board.white_queens, board.white_king,
                       board.black_pawns | board.black_knights | board.black_bishops | board.black_rooks | board.black_queens | board.black_king),
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
        current = current & !rook;
        result |= get_rook_moves(&rook, &all_pieces, &friendly);
    }

    current = bishops | queens;
    while current != 0 {
        let from = current.trailing_zeros() as u8;
        let bishop = 1 << from;
        current = current & !bishop;
        result |= get_bishop_moves(&bishop, &all_pieces, &friendly);
    }

    result
}
