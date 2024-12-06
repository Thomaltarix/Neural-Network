use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece = match self {
            ChessPiece::Pawn => "Pawn",
            ChessPiece::Rook => "Rook",
            ChessPiece::Knight => "Knight",
            ChessPiece::Bishop => "Bishop",
            ChessPiece::Queen => "Queen",
            ChessPiece::King => "King",
        };
        write!(f, "{}", piece)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChessColor {
    White,
    Black,
}

impl fmt::Display for ChessColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            ChessColor::White => "White",
            ChessColor::Black => "Black",
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    piece: ChessPiece,
    color: ChessColor,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.piece)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChessBox {
    piece: Option<Piece>,
    position: (u8, u8),
}

impl fmt::Display for ChessBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.piece {
            Some(piece) => write!(f, "{} at position {:?}", piece, self.position),
            None => write!(f, "Empty case at position {:?}", self.position),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChessBoard {
    board: Vec<Vec<ChessBox>>,
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str = String::new();
        board_str.push_str("Chessboard: {\n");
        for row in self.board.iter() {
            board_str.push_str(&format!("\tColone {}: {{\n", row[0].position.0));
            for case in row.iter() {
                match case.piece {
                    Some(piece) => board_str.push_str(&format!("\t\tCase {}: {},\n", case.get_position().1, piece)),
                    None => board_str.push_str(&format!("\t\tCase {}: Empty case,\n", case.get_position().1)),
                }
            }
            board_str.push_str("\t},\n");
        }
        board_str.push_str("}");
        write!(f, "{}", board_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChessboardParser {
    board: ChessBoard,
}

impl Piece {
    pub fn new(piece: ChessPiece, color: ChessColor) -> Piece {
        Piece {
            piece,
            color,
        }
    }

    //getters
    pub fn get_piece(&self) -> ChessPiece {
        self.piece
    }

    pub fn get_color(&self) -> ChessColor {
        self.color
    }

    pub fn get_piece_name(&self) -> String {
        match self.piece {
            ChessPiece::Pawn => "Pawn".to_string(),
            ChessPiece::Rook => "Rook".to_string(),
            ChessPiece::Knight => "Knight".to_string(),
            ChessPiece::Bishop => "Bishop".to_string(),
            ChessPiece::Queen => "Queen".to_string(),
            ChessPiece::King => "King".to_string(),
        }
    }

    pub fn get_color_name(&self) -> String {
        match self.color {
            ChessColor::White => "White".to_string(),
            ChessColor::Black => "Black".to_string(),
        }
    }

    //setters
    pub fn set_piece(&mut self, piece: ChessPiece) {
        self.piece = piece;
    }

    pub fn set_color(&mut self, color: ChessColor) {
        self.color = color;
    }

    pub fn set_piece_name(&mut self, piece_name: String) {
        self.piece = match piece_name.as_str() {
            "Pawn" => ChessPiece::Pawn,
            "Rook" => ChessPiece::Rook,
            "Knight" => ChessPiece::Knight,
            "Bishop" => ChessPiece::Bishop,
            "Queen" => ChessPiece::Queen,
            "King" => ChessPiece::King,
            _ => panic!("Invalid piece name"),
        };
    }

    pub fn set_color_name(&mut self, color_name: String) {
        self.color = match color_name.as_str() {
            "White" => ChessColor::White,
            "Black" => ChessColor::Black,
            _ => panic!("Invalid color name"),
        };
    }
}

impl ChessBox {
    pub fn new(piece: Option<Piece>, position: (u8, u8)) -> ChessBox {
        ChessBox {
            piece,
            position,
        }
    }

    //getters
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }

    pub fn get_position(&self) -> (u8, u8) {
        self.position
    }

    //setters
    pub fn set_piece(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }

    pub fn set_position(&mut self, position: (u8, u8)) {
        self.position = position;
    }
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut board = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                row.push(ChessBox::new(None, (i, j)));
            }
            board.push(row);
        }
        ChessBoard { board }
    }

    //getters
    pub fn get_board(&self) -> Vec<Vec<ChessBox>> {
        self.board.clone()
    }

    //setters
    pub fn set_board(&mut self, board: Vec<Vec<ChessBox>>) {
        self.board = board;
    }

    pub fn set_case(&mut self, piece: Option<Piece>, position: (u8, u8)) {
        self.board[position.0 as usize][position.1 as usize] = ChessBox::new(piece, position);
    }
}

impl ChessboardParser {
    pub fn new() -> ChessboardParser {
        ChessboardParser {
            board: ChessBoard::new(),
        }
    }

    //getters
    pub fn get_board(&self) -> ChessBoard {
        self.board.clone()
    }

    //setters
    pub fn set_board(&mut self, board: ChessBoard) {
        self.board = board;
    }

    pub fn parse_fen(&mut self, fen: String) {
        let mut fen = fen.split_whitespace();
        let mut board = ChessBoard::new();
        let mut row = 0;
        let mut col = 0;
        for c in fen.next().unwrap().chars() {
            match c {
                'p' => {
                    board.set_case(Some(Piece::new(ChessPiece::Pawn, ChessColor::Black)), (row, col));
                    col += 1;
                }
                'r' => {
                    board.set_case(Some(Piece::new(ChessPiece::Rook, ChessColor::Black)), (row, col));
                    col += 1;
                }
                'n' => {
                    board.set_case(Some(Piece::new(ChessPiece::Knight, ChessColor::Black)), (row, col));
                    col += 1;
                }
                'b' => {
                    board.set_case(Some(Piece::new(ChessPiece::Bishop, ChessColor::Black)), (row, col));
                    col += 1;
                }
                'q' => {
                    board.set_case(Some(Piece::new(ChessPiece::Queen, ChessColor::Black)), (row, col));
                    col += 1;
                }
                'k' => {
                    board.set_case(Some(Piece::new(ChessPiece::King, ChessColor::Black)), (row, col));
                    col += 1;
                }
                'P' => {
                    board.set_case(Some(Piece::new(ChessPiece::Pawn, ChessColor::White)), (row, col));
                    col += 1;
                }
                'R' => {
                    board.set_case(Some(Piece::new(ChessPiece::Rook, ChessColor::White)), (row, col));
                    col += 1;
                }
                'N' => {
                    board.set_case(Some(Piece::new(ChessPiece::Knight, ChessColor::White)), (row, col));
                    col += 1;
                }
                'B' => {
                    board.set_case(Some(Piece::new(ChessPiece::Bishop, ChessColor::White)), (row, col));
                    col += 1;
                }
                'Q' => {
                    board.set_case(Some(Piece::new(ChessPiece::Queen, ChessColor::White)), (row, col));
                    col += 1;
                }
                'K' => {
                    board.set_case(Some(Piece::new(ChessPiece::King, ChessColor::White)), (row, col));
                    col += 1;
                }
                '/' => {
                    row += 1;
                    col = 0;
                }
                '1'..='8' => {
                    for _ in 0..(c as u8 - '0' as u8) {
                        board.set_case(None, (row, col));
                        col += 1
                    }
                }
                ' ' => {
                    break;
                }
                _ => {
                    panic!("Unexpected character encountered: {}", c);
                }
            }
        }
        self.board = board;
    }
}

