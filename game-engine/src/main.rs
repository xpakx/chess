mod rabbit;
mod config;
mod engine;

use crate::rabbit::lapin_listen;
use crate::engine::rules::{ROOK_RAYS, BISHOP_RAYS, get_rook_moves, get_bishop_moves, get_possible_moves, string_to_move};
use crate::engine::{BitBoard, generate_bit_board};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    let square = 3 * 8 + 3; // d4 position
    let directions = ["NORTH", "EAST", "SOUTH", "WEST"];

    for (i, direction) in directions.iter().enumerate() {
        println!("\nRays in {} direction from d4:", direction);
        print_bitboard(ROOK_RAYS[i][square]);
    }

    let directions = ["NORTHEAST", "NORTHWEST", "SOUTHWEST", "SOUTHEAST"];
    for (i, direction) in directions.iter().enumerate() {
        println!("\nRays in {} direction from d4:", direction);
        print_bitboard(BISHOP_RAYS[i][square]);
    }
    let rook = 1 << (4 * 8 + 4);
    let occupied = (1 << (6 * 8 + 4)) | (1 << (4 * 8 + 6));

    let moves = get_rook_moves(&rook, &occupied, &0);
    println!("Rook moves:");
    print_bitboard(moves);

    let bishop = 1 << (4 * 8 + 4);
    let occupied = (1 << (6 * 8 + 6)) | (1 << (2 * 8 + 2));
    let moves = get_bishop_moves(&bishop, &occupied, &0);

    println!("Bishop moves:");
    print_bitboard(moves);

    let mut board = generate_bit_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()).unwrap();

    println!("Board:");
    print_board(&board);

    let moves = get_possible_moves(&board, &Color::White);
    println!("{} moves:", moves.len());
    for mov in moves {
        board.apply_move(&mov, &Color::White);
        print_board(&board);
        board.apply_move(&mov, &Color::White);
        println!("---------------");
    }
    let moves = get_possible_moves(&board, &Color::Red);
    println!("{} moves:", moves.len());



    let move_str = String::from("Nxe4");
    string_to_move(&board, move_str);
    print!("");
    string_to_move(&board, String::from("e4"));
    print!("");
    string_to_move(&board, String::from("Nf3"));
    print!("");
    string_to_move(&board, String::from("Bb5+"));
    print!("");
    string_to_move(&board, String::from("exd5"));
    print!("");
    string_to_move(&board, String::from("Qxe6"));
    print!("");
    string_to_move(&board, String::from("a8=Q"));
    print!("");
    string_to_move(&board, String::from("g8=N"));

    let config = config::get_config();
    let mut cfg = deadpool_lapin::Config::default();
    cfg.url = Some(config.rabbit.into());
    let lapin_pool = cfg.create_pool(Some(deadpool_lapin::Runtime::Tokio1)).unwrap();
    lapin_listen(lapin_pool.clone()).await;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Color {
    White,
    Red,
}

fn print_bitboard(bitboard: u64) {
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            if bitboard & (1 << square) != 0 {
                print!("1 ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn print_board(bitboard: &BitBoard) {
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            if bitboard.black_pawns & (1 << square) != 0 {
                print!("♟ ");
            } else if bitboard.black_bishops & (1 << square) != 0 {
                print!("♝ ");
            } else if bitboard.black_knights & (1 << square) != 0 {
                print!("♞ ");
            } else if bitboard.black_rooks & (1 << square) != 0 {
                print!("♜ ");
            } else if bitboard.black_queens & (1 << square) != 0 {
                print!("♛ ");
            } else if bitboard.black_king & (1 << square) != 0 {
                print!("♚ ");
            } else if bitboard.white_pawns & (1 << square) != 0 {
                print!("♙ ");
            } else if bitboard.white_bishops & (1 << square) != 0 {
                print!("♗ ");
            } else if bitboard.white_knights & (1 << square) != 0 {
                print!("♘ ");
            } else if bitboard.white_rooks & (1 << square) != 0 {
                print!("♖ ");
            } else if bitboard.white_queens & (1 << square) != 0 {
                print!("♕ ");
            } else if bitboard.white_king & (1 << square) != 0 {
                print!("♔ ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
