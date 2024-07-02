mod rabbit;
mod config;
mod engine;

use crate::rabbit::lapin_listen;
use crate::engine::rules::{get_possible_moves, string_to_move};
use crate::engine::{BitBoard, generate_bit_board};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let fen = generate_bit_board(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()).unwrap();

    let mut board = fen.board;

    println!("Board:");
    print_board(&board);
    println!("{:?}", fen.color);
    println!("{:?}", fen.castling);
    println!("{:?}", fen.enpassant);
    println!("{:?}", fen.halfmoves);
    println!("{:?}", fen.moves);

    return;

    let moves = get_possible_moves(&board, &Color::White);
    println!("{} moves:", moves.len());
    for mov in moves {
        board.apply_move(&mov, &Color::White);
        print_board(&board);
        board.apply_move(&mov, &Color::White);
        println!("---------------");
    }
    let moves = get_possible_moves(&board, &Color::Black);
    println!("{} moves:", moves.len());


    let mov = string_to_move(&mut board, String::from("e4"), &Color::White).unwrap();
    board.apply_move(&mov, &Color::White);
    print_board(&board);
    board.apply_move(&mov, &Color::White);

    let mov = string_to_move(&mut board, String::from("e3"), &Color::White).unwrap();
    board.apply_move(&mov, &Color::White);
    print_board(&board);
    board.apply_move(&mov, &Color::White);

    let config = config::get_config();
    let mut cfg = deadpool_lapin::Config::default();
    cfg.url = Some(config.rabbit.into());
    let lapin_pool = cfg.create_pool(Some(deadpool_lapin::Runtime::Tokio1)).unwrap();
    lapin_listen(lapin_pool.clone()).await;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn print_board(bitboard: &BitBoard) {
    for rank in (0..8).rev() {
        for file in (0..8).rev() {
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
