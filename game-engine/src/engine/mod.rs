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

    let mut index: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

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
                'p' => white_pawns = white_pawns & index,
                'n' => white_knights = white_knights & index,
                'b' => white_bishops = white_bishops & index,
                'r' => white_rooks = white_rooks & index,
                'q' => white_queens = white_queens & index,
                'k' => white_king = white_king & index,

                'P' => black_pawns = black_pawns & index,
                'N' => black_knights = black_knights & index,
                'B' => black_bishops = black_bishops & index,
                'R' => black_rooks = black_rooks & index,
                'Q' => black_queens = black_queens & index,
                'K' => black_king = black_king & index,
                _ => return Err("".into()),
            }
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
