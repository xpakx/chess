mod rabbit;
mod config;
mod engine;

use crate::rabbit::lapin_listen;
use crate::engine::rules::{ROOK_RAYS, BISHOP_RAYS};
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

    let config = config::get_config();
    let mut cfg = deadpool_lapin::Config::default();
    cfg.url = Some(config.rabbit.into());
    let lapin_pool = cfg.create_pool(Some(deadpool_lapin::Runtime::Tokio1)).unwrap();
    lapin_listen(lapin_pool.clone()).await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    White,
    Red,
}


fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
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
