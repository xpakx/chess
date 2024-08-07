use std::u64;

use crate::Color;

use self::rules::{field_to_num, Move, Piece};
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

pub struct FEN {
    pub board: BitBoard,
    pub color: Color,
    pub castling: CastlingAvailability,
    pub enpassant: Option<u8>,
    pub halfmoves: usize,
    pub moves: usize,
}

impl CastlingAvailability {
    pub fn after_move(&self, mov: &Move, color: &Color) -> CastlingAvailability {
        let white_kingside_rook = 0;
        let white_queenside_rook = 7;
        let black_kingside_rook = 56;
        let black_queenside_rook = 63;

        let black_queenside = match self.black_queenside {
            false => false,
            true => match (&mov.piece, color) {
                (Piece::King, Color::Black) => false,
                (Piece::Rook, Color::Black) => !(mov.from == black_queenside_rook),
                (_, Color::White) => !(mov.capture.is_some() && mov.to == black_queenside_rook),
                _  => true,
            },
        };
        let black_kingside = match self.black_kingside {
            false => false,
            true => match (&mov.piece, color) {
                (Piece::King, Color::Black) => false,
                (Piece::Rook, Color::Black) => !(mov.from == black_kingside_rook),
                (_, Color::White) => !(mov.capture.is_some() && mov.to == black_kingside_rook),
                _  => true,
            },
        };

        let white_queenside = match self.white_queenside {
            false => false,
            true => match (&mov.piece, color) {
                (Piece::King, Color::White) => false,
                (Piece::Rook, Color::White) => !(mov.from == white_queenside_rook),
                (_, Color::Black) => !(mov.capture.is_some() && mov.to == white_queenside_rook),
                _  => true,
            },
        };
        let white_kingside = match self.white_kingside {
            false => false,
            true => match (&mov.piece, color) {
                (Piece::King, Color::White) => false,
                (Piece::Rook, Color::White) => !(mov.from == white_kingside_rook),
                (_, Color::Black) => !(mov.capture.is_some() && mov.to == white_kingside_rook),
                _  => true,
            },
        };

        CastlingAvailability { 
            black_queenside, 
            black_kingside, 
            white_queenside, 
            white_kingside
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        if self.white_kingside {
            fen.push('K');
        }
        if self.white_queenside {
            fen.push('Q');
        }
        if self.black_kingside {
            fen.push('k');
        }
        if self.black_queenside {
            fen.push('q');
        }
        if fen == "" {
            fen.push('-');
        }
        fen
    }
}

#[derive(Debug)]
pub struct CastlingAvailability {
    pub black_queenside: bool,
    pub black_kingside: bool,
    pub white_queenside: bool,
    pub white_kingside: bool,
}

pub fn generate_bit_board(fen_board: &String) -> Result<FEN, String> {
    let mut fen_notation = fen_board.split(" ");

    // board
    let board_def = fen_notation.next();
    let Some(board_def) = board_def else {
        return  Err("No board representation".into())
    };

    let board = generate_board_from_fen(board_def);
    let Ok(board) = board else {
        return Err(board.err().unwrap())
    };

    // color
    let color = fen_notation.next();
    let Some(color) = color else {
        return  Err("No color information".into())
    };
    let color = match color {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err("Incorrect color!".into()),
    };

    // castling
    let castling = fen_notation.next();
    let Some(castling) = castling else {
        return  Err("No castling information".into())
    };
    let castling = match castling {
        "-" => CastlingAvailability {black_kingside: false, black_queenside: false, white_kingside: false, white_queenside: false},
        castling => CastlingAvailability {
                black_kingside: castling.contains("k"), 
                black_queenside: castling.contains("q"),
                white_kingside: castling.contains("K"),
                white_queenside: castling.contains("Q"),
            },
    };

    // enpassant
    let enpassant = fen_notation.next();
    let Some(enpassant) = enpassant else {
        return  Err("No enpassant information".into())
    };
    let enpassant = match enpassant {
        "-" => None,
        enpassant => Some(field_to_num(enpassant)),
    };

    // halfmove clock
    let halfmove_clock = fen_notation.next();
    let Some(halfmove_clock) = halfmove_clock else {
        return  Err("No halfmove count".into())
    };
    let halfmoves = halfmove_clock.parse::<usize>();
    let Ok(halfmoves) = halfmoves else {
        return  Err("Corrupted halfmove count".into())
    };

    // move clock
    let move_clock = fen_notation.next();
    let Some(move_clock) = move_clock else {
        return  Err("No move count".into())
    };
    let moves = move_clock.parse::<usize>();
    let Ok(moves) = moves else {
        return  Err("Corrupted move count".into())
    };

   
    Ok(FEN { board, color, castling, enpassant, halfmoves, moves })
}


fn generate_board_from_fen(board_def: &str) -> Result<BitBoard, String> {
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

    // TODO: castling, enpassant
    pub fn apply_move(&mut self, mov: &Move, color: &Color) -> () {
        if let Some(promotion) = mov.promotion {
            self.apply_promotion(mov, color, &promotion);
            return;
        }
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

    fn apply_promotion(&mut self, mov: &Move, color: &Color, promotion: &Piece) -> () {
        if color == &Color::White {
            self.white_pawns = self.white_pawns ^ 1<<mov.from;
            match promotion {
                Piece::Knight => self.white_knights = self.white_knights ^ 1<<mov.to,
                Piece::Bishop => self.white_bishops = self.white_bishops ^ 1<<mov.to,
                Piece::Rook => self.white_rooks = self.white_rooks ^ 1<<mov.to,
                Piece::Queen => self.white_queens = self.white_queens ^ 1<<mov.to,
                _ => {},
            }
            match mov.capture {
                None => {},
                Some(Piece::Pawn) => self.black_pawns = self.black_pawns ^ 1<<mov.to,
                Some(Piece::Knight) => self.black_knights = self.black_knights ^ 1<<mov.to,
                Some(Piece::Bishop) => self.black_bishops = self.black_bishops ^ 1<<mov.to,
                Some(Piece::Rook) => self.black_rooks = self.black_rooks ^ 1<<mov.to,
                Some(Piece::Queen) => self.black_queens = self.black_queens ^ 1<<mov.to,
                Some(Piece::King) => self.black_king = self.black_king ^ 1<<mov.to,
            }
        } else {
            self.black_pawns = self.black_pawns ^ 1<<mov.from;
            match promotion {
                Piece::Knight => self.black_knights = self.black_knights ^ 1<<mov.to,
                Piece::Bishop => self.black_bishops = self.black_bishops ^ 1<<mov.to,
                Piece::Rook => self.black_rooks = self.black_rooks ^ 1<<mov.to,
                Piece::Queen => self.black_queens = self.black_queens ^ 1<<mov.to,
                _ => {},
            }
            match mov.capture {
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

    pub fn get_white(&self) -> u64 {
        self.white_pawns | self.white_knights | self.white_bishops | self.white_rooks | self.white_queens | self.white_king
    }

    pub fn get_black(&self) -> u64 {
        self.black_pawns | self.black_knights | self.black_bishops | self.black_rooks | self.black_queens | self.black_king
    }
}

pub trait Engine {
    fn get_name(&self) -> String;
    fn get_move(&mut self, board: &mut BitBoard, color: &Color) -> Move;
}

pub enum EngineType {
    Random,
}

pub fn get_engine(engine: EngineType) -> Box<dyn Engine> {
    match engine {
        EngineType::Random => Box::new(random_engine::RandomEngine::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bit_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(fen_obj.color, Color::White);
        assert_eq!(fen_obj.castling.to_fen(), "KQkq");
        assert_eq!(fen_obj.enpassant, None);
        assert_eq!(fen_obj.halfmoves, 0);
        assert_eq!(fen_obj.moves, 1);
    }

    #[test]
    fn test_generate_bit_board_black_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.color, Color::Black);
    }

    #[test]
    fn test_generate_bit_board_wrong_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR t KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen);
        assert!(fen_obj.is_err());
        let err = fen_obj.err().unwrap();
        assert!(err.contains("Incorrect color"));
    }

    #[test]
    fn test_generate_bit_board_castling() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.castling.to_fen(), "Kkq");
    }

    #[test]
    fn test_generate_bit_board_no_castling() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.castling.to_fen(), "-");
    }

    #[test]
    fn test_generate_bit_board_enpassant() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert!(fen_obj.enpassant.is_some());
        let enpassant = fen_obj.enpassant.unwrap();
        assert_eq!(enpassant, 19);
    }

    #[test]
    fn test_generate_bit_board_empty_board() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "8/8/8/8/8/8/8/8");
    }

    #[test]
    fn test_generate_bit_board_middle_game_position() {
        let fen = "r1bqkbnr/pppppppp/n7/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "r1bqkbnr/pppppppp/n7/8/8/N7/PPPPPPPP/R1BQKBNR");
    }

    #[test]
    fn test_generate_bit_board_end_game_position() {
        let fen = "8/8/8/8/8/8/2k5/3K4 w - - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "8/8/8/8/8/8/2k5/3K4");
    }

    #[test]
    fn test_generate_bit_board_castling_rights() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "r3k2r/8/8/8/8/8/8/R3K2R");
    }

    #[test]
    fn test_generate_bit_board_en_passant_target_square() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR");
    }

    #[test]
    fn test_generate_bit_board_halfmove_and_fullmove_clock() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 5 10".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_generate_bit_board_complex_position() {
        let fen = "rnbq1bnr/pppp1ppp/4k3/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQ - 2 6".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.to_fen(), "rnbq1bnr/pppp1ppp/4k3/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R");
    }

    #[test]
    fn test_apply_move_no_capture() {
        let fen = "8/8/8/8/8/8/P7/R3K2R w KQ - 0 1".to_string();
        let mut fen_obj = generate_bit_board(&fen).unwrap();
        let mov = Move { from: 7, to: 4, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        fen_obj.board.apply_move(&mov, &Color::White);
        assert_eq!(fen_obj.board.to_fen(), "8/8/8/8/8/8/P7/3RK2R");
    }

    #[test]
    fn test_unmove_no_capture() {
        let fen = "8/8/8/8/8/8/P7/R3K2R w KQ - 0 1".to_string();
        let mut fen_obj = generate_bit_board(&fen).unwrap();
        let mov = Move { from: 7, to: 4, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        fen_obj.board.apply_move(&mov, &Color::White);
        fen_obj.board.apply_move(&mov, &Color::White);
        assert_eq!(fen_obj.board.to_fen(), "8/8/8/8/8/8/P7/R3K2R");
    }

    #[test]
    fn test_apply_move_capture() {
        let fen = "8/8/8/8/8/8/P7/R2rK2R w KQ - 0 1".to_string();
        let mut fen_obj = generate_bit_board(&fen).unwrap();
        let mov = Move { from: 7, to: 4, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        fen_obj.board.apply_move(&mov, &Color::White);
        assert_eq!(fen_obj.board.to_fen(), "8/8/8/8/8/8/P7/3RK2R");
    }

    #[test]
    fn test_unmove_capture() {
        let fen = "8/8/8/8/8/8/P7/R2rK2R w KQ - 0 1".to_string();
        let mut fen_obj = generate_bit_board(&fen).unwrap();
        let mov = Move { from: 7, to: 4, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        fen_obj.board.apply_move(&mov, &Color::White);
        fen_obj.board.apply_move(&mov, &Color::White);
        assert_eq!(fen_obj.board.to_fen(), "8/8/8/8/8/8/P7/R2rK2R");
    }

    #[test]
    fn test_castling_availability_white_king_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 3, to: 4, piece: Piece::King, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::White);
        assert_eq!(updated_castling.to_fen(), "kq");
    }

    #[test]
    fn test_castling_availability_black_king_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 59, to: 60, piece: Piece::King, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "KQ");
    }

    #[test]
    fn test_castling_no_castling() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: false,
            white_kingside: false,
        };
        let mov = Move { from: 59, to: 60, piece: Piece::King, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "-");
    }

    #[test]
    fn test_castling_availability_white_kingside_rook_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 0, to: 60, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::White);
        assert_eq!(updated_castling.to_fen(), "Qkq");
    }

    #[test]
    fn test_castling_availability_white_queenside_rook_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 7, to: 60, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::White);
        assert_eq!(updated_castling.to_fen(), "Kkq");
    }

    #[test]
    fn test_castling_availability_white_free_rook_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 5, to: 60, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::White);
        assert_eq!(updated_castling.to_fen(), "KQkq");
    }


    #[test]
    fn test_castling_availability_black_kingside_rook_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 56, to: 60, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "KQq");
    }

    #[test]
    fn test_castling_availability_black_queenside_rook_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 63, to: 60, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "KQk");
    }

    #[test]
    fn test_castling_availability_black_free_rook_moves() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 5, to: 60, piece: Piece::Rook, capture: None, promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "KQkq");
    }

    #[test]
    fn test_castling_availability_white_kingside_rook_captured() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 60, to: 0, piece: Piece::Queen, capture: Some(Piece::Rook), promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "Qkq");
    }

    #[test]
    fn test_castling_availability_white_queenside_rook_captured() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 60, to: 7, piece: Piece::Queen, capture: Some(Piece::Rook), promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::Black);
        assert_eq!(updated_castling.to_fen(), "Kkq");
    }

    #[test]
    fn test_castling_availability_black_kingside_rook_captured() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 60, to: 56, piece: Piece::Queen, capture: Some(Piece::Rook), promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::White);
        assert_eq!(updated_castling.to_fen(), "KQq");
    }

    #[test]
    fn test_castling_availability_black_queenside_rook_captured() {
        let castling = CastlingAvailability {
            black_queenside: true,
            black_kingside: true,
            white_queenside: true,
            white_kingside: true,
        };
        let mov = Move { from: 60, to: 63, piece: Piece::Queen, capture: Some(Piece::Rook), promotion: None, castling: false };
        let updated_castling = castling.after_move(&mov, &Color::White);
        assert_eq!(updated_castling.to_fen(), "KQk");
    }

    #[test]
    fn test_check_capture_white() {
        let fen = "8/8/8/8/8/8/8/R3K3 w Q - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        let target = 1 << 7;
        assert_eq!(fen_obj.board.check_capture(&target, &Color::Black), Some(Piece::Rook));
    }

    #[test]
    fn test_check_capture_white_no_capture() {
        let fen = "8/8/8/8/8/8/8/R3K3 w Q - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        let target = 1 << 6;
        assert_eq!(fen_obj.board.check_capture(&target, &Color::Black), None);
    }

    #[test]
    fn test_check_capture_black() {
        let fen = "8/8/8/8/8/8/8/r3k3 w Q - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        let target = 1 << 7;
        assert_eq!(fen_obj.board.check_capture(&target, &Color::White), Some(Piece::Rook));
    }

    #[test]
    fn test_check_capture_black_no_capture() {
        let fen = "8/8/8/8/8/8/8/r3k3 w Q - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        let target = 1 << 6;
        assert_eq!(fen_obj.board.check_capture(&target, &Color::White), None);
    }

    #[test]
    fn test_field_to_num() {
        assert_eq!(field_to_num("a1"), 7);
        assert_eq!(field_to_num("a8"), 63);
        assert_eq!(field_to_num("h1"), 0);
        assert_eq!(field_to_num("h8"), 56);
    }

    #[test]
    fn test_get_bitboard_by_piece_white() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();

        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Pawn, &Color::White), 0x0000_0000_0000_FF00);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Knight, &Color::White), 0x0000_0000_0000_0042);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Bishop, &Color::White), 0x0000_0000_0000_0024);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Rook, &Color::White), 0x0000_0000_0000_0081);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Queen, &Color::White), 0x0000_0000_0000_0010);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::King, &Color::White), 0x0000_0000_0000_0008);
    }

    #[test]
    fn test_get_bitboard_by_piece_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();

        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Pawn, &Color::Black), 0x00FF_0000_0000_0000);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Knight, &Color::Black), 0x4200_0000_0000_0000);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Bishop, &Color::Black), 0x2400_0000_0000_0000);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Rook, &Color::Black), 0x8100_0000_0000_0000);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::Queen, &Color::Black), 0x1000_0000_0000_0000);
        assert_eq!(fen_obj.board.get_bitboard_by_piece(&Piece::King, &Color::Black), 0x0800_0000_0000_0000);
    }

    #[test]
    fn test_get_king_by_color_white() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.get_king_by_color(&Color::White), 0x0000_0000_0000_0008);
    }

    #[test]
    fn test_get_king_by_color_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_obj = generate_bit_board(&fen).unwrap();
        assert_eq!(fen_obj.board.get_king_by_color(&Color::Black), 0x0800_0000_0000_0000);
    }
}
