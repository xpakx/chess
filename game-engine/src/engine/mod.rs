use std::u64;

use crate::Color;

use self::rules::{Move, Piece};
mod random_engine;
pub mod rules;

#[derive(Debug)]
pub struct BitBoard {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

pub fn generate_bit_board(fen_board: String) -> Result<BitBoard, String> {
    let mut white_pawns = 0;
    let mut white_knights = 0;
    let mut white_bishops = 0;
    let mut white_rooks = 0;
    let mut white_queens = 0;
    let mut white_king = 0;

    let mut black_pawns = 0;
    let mut black_knights = 0;
    let mut black_bishops = 0;
    let mut black_rooks = 0;
    let mut black_queens = 0;
    let mut black_king = 0;

    let mut index: u64 = 1 << 63;

    let board_def = fen_board.split(" ").next();
    let Some(board_def) = board_def else {
        return  Err("".into())
    };
    let ranks = board_def.split("/");
    for rank in ranks {
        for a in rank.chars() {
            if a.is_digit(10) {
                let empty_spaces = a.to_digit(10).unwrap();
                index = index >> empty_spaces;
                continue;
            }
            match a {
                'P' => white_pawns = white_pawns | index,
                'N' => white_knights = white_knights | index,
                'B' => white_bishops = white_bishops | index,
                'R' => white_rooks = white_rooks | index,
                'Q' => white_queens = white_queens | index,
                'K' => white_king = white_king | index,

                'p' => black_pawns = black_pawns | index,
                'n' => black_knights = black_knights | index,
                'b' => black_bishops = black_bishops | index,
                'r' => black_rooks = black_rooks | index,
                'q' => black_queens = black_queens | index,
                'k' => black_king = black_king | index,
                _ => return Err("".into()),
            }
            index = index >> 1;
        }
    }
    Ok(BitBoard {
        white_pawns,
        white_knights,
        white_bishops,
        white_rooks,
        white_queens,
        white_king,
        black_pawns,
        black_knights,
        black_bishops,
        black_rooks,
        black_queens,
        black_king,
    })
}

impl BitBoard {
    pub fn to_fen(self) -> String {
        let mut index: u64 = 1 << 63;
        let mut empty = 0;
        let mut result = String::from("");
        for rank in 0..8 {
            for _file in 0..8 {
                let mut piece = None;
                if self.white_pawns & index != 0  {
                    piece = Some('P');
                } else if self.white_knights & index != 0 {
                    piece = Some('N');
                } else if self.white_bishops & index != 0 {
                    piece = Some('B');
                } else if self.white_rooks & index != 0 {
                    piece = Some('R');
                } else if self.white_queens & index != 0 {
                    piece = Some('Q');
                } else if self.white_king & index != 0 {
                    piece = Some('K');
                } else if self.black_pawns & index != 0  {
                    piece = Some('p');
                } else if self.black_knights & index != 0 {
                    piece = Some('n');
                } else if self.black_bishops & index != 0 {
                    piece = Some('b');
                } else if self.black_rooks & index != 0 {
                    piece = Some('r');
                } else if self.black_queens & index != 0 {
                    piece = Some('q');
                } else if self.black_king & index != 0 {
                    piece = Some('k');
                }

                if let Some(piece) = piece {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                    }
                    empty = 0;
                    result.push(piece);
                } else {
                    empty += 1;
                }

                index = index >> 1;
            }
            if empty > 0 {
                result.push_str(&empty.to_string());
            }
            empty = 0;
            if rank != 7 {
                result.push('/');
            }
        }

        result
    }

    pub fn check_capture(&self, target: &u64, color: &Color) -> Option<Piece> {
        match color {
            Color::Black => {
               if self.white_pawns & target != 0 {
                   return Some(Piece::Pawn)
               }
               if self.white_knights & target != 0 {
                   return Some(Piece::Knight)
               }
               if self.white_bishops & target != 0 {
                   return Some(Piece::Bishop)
               }
               if self.white_rooks & target != 0 {
                   return Some(Piece::Rook)
               }
               if self.white_queens & target != 0 {
                   return Some(Piece::Queen)
               }
               if self.white_king & target != 0 {
                   return Some(Piece::King) // shouldn't happen
               }
               None
            },
            Color::White => {
               if self.black_pawns & target != 0 {
                   return Some(Piece::Pawn)
               }
               if self.black_knights & target != 0 {
                   return Some(Piece::Knight)
               }
               if self.black_bishops & target != 0 {
                   return Some(Piece::Bishop)
               }
               if self.black_rooks & target != 0 {
                   return Some(Piece::Rook)
               }
               if self.black_queens & target != 0 {
                   return Some(Piece::Queen)
               }
               if self.black_king & target != 0 {
                   return Some(Piece::King) // shouldn't happen
               }
               None
            }
        }
    }

    // TODO: captures, castling, enpassant
    pub fn apply_move(&mut self, mov: &Move, color: &Color) -> () {
        if color == &Color::White {
            match mov.piece {
                Piece::Pawn => self.white_pawns = self.white_pawns ^ (1<<mov.from | 1<<mov.to),
                Piece::Knight => self.white_knights = self.white_knights ^ (1<<mov.from | 1<<mov.to),
                Piece::Bishop => self.white_bishops = self.white_bishops ^ (1<<mov.from | 1<<mov.to),
                Piece::Rook => self.white_rooks = self.white_rooks ^ (1<<mov.from | 1<<mov.to),
                Piece::Queen => self.white_queens = self.white_queens ^ (1<<mov.from | 1<<mov.to),
                Piece::King => self.white_king = self.white_king ^ (1<<mov.from | 1<<mov.to),
            }
            match mov.capture { // TODO: enpassant
                None => {},
                Some(Piece::Pawn) => self.black_pawns = self.black_pawns ^ 1<<mov.to,
                Some(Piece::Knight) => self.black_knights = self.black_knights ^ 1<<mov.to,
                Some(Piece::Bishop) => self.black_bishops = self.black_bishops ^ 1<<mov.to,
                Some(Piece::Rook) => self.black_rooks = self.black_rooks ^ 1<<mov.to,
                Some(Piece::Queen) => self.black_queens = self.black_queens ^ 1<<mov.to,
                Some(Piece::King) => self.black_king = self.black_king ^ 1<<mov.to,
            }
        } else {
            match mov.piece {
                Piece::Pawn => self.black_pawns = self.black_pawns ^ (1<<mov.from | 1<<mov.to),
                Piece::Knight => self.black_knights = self.black_knights ^ (1<<mov.from | 1<<mov.to),
                Piece::Bishop => self.black_bishops = self.black_bishops ^ (1<<mov.from | 1<<mov.to),
                Piece::Rook => self.black_rooks = self.black_rooks ^ (1<<mov.from | 1<<mov.to),
                Piece::Queen => self.black_queens = self.black_queens ^ (1<<mov.from | 1<<mov.to),
                Piece::King => self.black_king = self.black_king ^ (1<<mov.from | 1<<mov.to),
            }
            match mov.capture { // TODO: enpassant
                None => {},
                Some(Piece::Pawn) => self.white_pawns = self.white_pawns ^ 1<<mov.to,
                Some(Piece::Knight) => self.white_knights = self.white_knights ^ 1<<mov.to,
                Some(Piece::Bishop) => self.white_bishops = self.white_bishops ^ 1<<mov.to,
                Some(Piece::Rook) => self.white_rooks = self.white_rooks ^ 1<<mov.to,
                Some(Piece::Queen) => self.white_queens = self.white_queens ^ 1<<mov.to,
                Some(Piece::King) => self.white_king = self.white_king ^ 1<<mov.to,
            }
        }
    }

    pub fn get_capture(&self, to: &u64, color: &Color) -> Option<Piece> {
        let (pawns, knights, bishops, rooks, queens, king) = match color {
            Color::White => (self.black_pawns, self.black_knights, self.black_bishops, self.black_rooks, self.black_queens, self.black_king),
            Color::Black => (self.white_pawns, self.white_knights, self.white_bishops, self.white_rooks, self.white_queens, self.white_king),
        };
        if pawns & to != 0 {
            return Some(Piece::Pawn)
        }
        if knights & to != 0 {
            return Some(Piece::Knight)
        }
        if bishops & to != 0 {
            return Some(Piece::Bishop)
        }
        if rooks & to != 0 {
            return Some(Piece::Rook)
        }
        if queens & to != 0 {
            return Some(Piece::Queen)
        }
        if king & to != 0 {
            return Some(Piece::King) // should't happen?
        }

        None
    }

    pub fn get_bitboard_by_piece(&self, piece: &Piece, color: &Color) -> u64 {
        match color {
            Color::Black => match piece {
                    Piece::Pawn => self.black_pawns,
                    Piece::Knight => self.black_knights,
                    Piece::Bishop => self.black_bishops,
                    Piece::Rook => self.black_rooks,
                    Piece::Queen => self.black_queens,
                    Piece::King => self.black_king,
                }
            ,
            Color::White => match piece {
                Piece::Pawn => self.white_pawns,
                Piece::Knight => self.white_knights,
                Piece::Bishop => self.white_bishops,
                Piece::Rook => self.white_rooks,
                Piece::Queen => self.white_queens,
                Piece::King => self.white_king,
            }
        }
    }

    pub fn get_king_by_color(&self, color: &Color) -> u64 {
        match color {
            Color::Black => self.black_king,
            Color::White => self.white_king,
        }
    }
}

pub trait Engine {
    fn get_name(&self) -> String;
    fn get_move(&mut self, board: &BitBoard, color: &Color) -> Move;
}

pub enum EngineType {
    Random,
}

pub fn get_engine(engine: EngineType) -> Box<dyn Engine> {
    match engine {
        EngineType::Random => Box::new(random_engine::RandomEngine::new()),
    }
}
