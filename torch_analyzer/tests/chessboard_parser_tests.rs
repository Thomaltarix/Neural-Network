use my_torch_analyzer::ChessboardParser::{ChessboardParser, ChessBoard, Piece, ChessColor, ChessPiece};

use std::sync::LazyLock;

static BOARD1: LazyLock<ChessBoard> = LazyLock::new(|| {
    let mut board = ChessBoard::new();

    // Set the board to a default state

    //Colone 0
    board.set_case(None, (0, 0));
    board.set_case(None, (0, 1));
    board.set_case(None, (0, 2));
    board.set_case(None, (0, 3));
    board.set_case(None, (0, 4));
    board.set_case(None, (0, 5));
    board.set_case(None, (0, 6));
    board.set_case(None, (0, 7));

    //Colone 1
    board.set_case(None, (1, 0));
    board.set_case(None, (1, 1));
    board.set_case(None, (1, 2));
    board.set_case(None, (1, 3));
    board.set_case(None, (1, 4));
    board.set_case(None, (1, 5));
    board.set_case(Some(Piece::new(ChessPiece::Knight, ChessColor::White)), (1, 6));
    board.set_case(None, (1, 7));

    //Colone 2
    board.set_case(None, (2, 0));
    board.set_case(None, (2, 1));
    board.set_case(None, (2, 2));
    board.set_case(None, (2, 3));
    board.set_case(None, (2, 4));
    board.set_case(None, (2, 5));
    board.set_case(None, (2, 6));
    board.set_case(None, (2, 7));

    //Colone 3
    board.set_case(None, (3, 0));
    board.set_case(None, (3, 1));
    board.set_case(None, (3, 2));
    board.set_case(None, (3, 3));
    board.set_case(Some(Piece::new(ChessPiece::Pawn, ChessColor::White)), (3, 4));
    board.set_case(None, (3, 5));
    board.set_case(None, (3, 6));
    board.set_case(Some(Piece::new(ChessPiece::King, ChessColor::Black)), (3, 7));

    //Colone 4
    board.set_case(None, (4, 0));
    board.set_case(None, (4, 1));
    board.set_case(None, (4, 2));
    board.set_case(Some(Piece::new(ChessPiece::Pawn, ChessColor::White)), (4, 3));
    board.set_case(Some(Piece::new(ChessPiece::King, ChessColor::White)), (4, 4));
    board.set_case(None, (4, 5));
    board.set_case(None, (4, 6));
    board.set_case(None, (4, 7));

    //Colone 5
    board.set_case(None, (5, 0));
    board.set_case(None, (5, 1));
    board.set_case(None, (5, 2));
    board.set_case(None, (5, 3));
    board.set_case(None, (5, 4));
    board.set_case(Some(Piece::new(ChessPiece::Rook, ChessColor::Black)), (5, 5));
    board.set_case(None, (5, 6));
    board.set_case(None, (5, 7));

    //Colone 6
    board.set_case(None, (6, 0));
    board.set_case(None, (6, 1));
    board.set_case(None, (6, 2));
    board.set_case(None, (6, 3));
    board.set_case(None, (6, 4));
    board.set_case(None, (6, 5));
    board.set_case(Some(Piece::new(ChessPiece::Rook, ChessColor::White)), (6, 6));
    board.set_case(None, (6, 7));

    //Colone 7
    board.set_case(None, (7, 0));
    board.set_case(None, (7, 1));
    board.set_case(None, (7, 2));
    board.set_case(None, (7, 3));
    board.set_case(None, (7, 4));
    board.set_case(None, (7, 5));
    board.set_case(None, (7, 6));
    board.set_case(None, (7, 7));


    board
});

#[test]
fn test_chess_color_black() {
    let black: ChessColor = ChessColor::Black;

    assert_eq!(black, ChessColor::Black);
    assert_ne!(black, ChessColor::White);
}

#[test]
fn test_chess_color_white() {
    let white: ChessColor = ChessColor::White;

    assert_eq!(white, ChessColor::White);
    assert_ne!(white, ChessColor::Black);
}

#[test]
fn test_chess_piece_pawn() {
    let pawn: ChessPiece = ChessPiece::Pawn;

    assert_eq!(pawn, ChessPiece::Pawn);
    assert_ne!(pawn, ChessPiece::Rook);
    assert_ne!(pawn, ChessPiece::Knight);
    assert_ne!(pawn, ChessPiece::Bishop);
    assert_ne!(pawn, ChessPiece::Queen);
    assert_ne!(pawn, ChessPiece::King);
}

#[test]
fn test_chess_piece_rook() {
    let rook: ChessPiece = ChessPiece::Rook;

    assert_eq!(rook, ChessPiece::Rook);
    assert_ne!(rook, ChessPiece::Pawn);
    assert_ne!(rook, ChessPiece::Knight);
    assert_ne!(rook, ChessPiece::Bishop);
    assert_ne!(rook, ChessPiece::Queen);
    assert_ne!(rook, ChessPiece::King);
}

#[test]
fn test_chess_piece_knight() {
    let knight: ChessPiece = ChessPiece::Knight;

    assert_eq!(knight, ChessPiece::Knight);
    assert_ne!(knight, ChessPiece::Pawn);
    assert_ne!(knight, ChessPiece::Rook);
    assert_ne!(knight, ChessPiece::Bishop);
    assert_ne!(knight, ChessPiece::Queen);
    assert_ne!(knight, ChessPiece::King);
}

#[test]
fn test_chess_piece_bishop() {
    let bishop: ChessPiece = ChessPiece::Bishop;

    assert_eq!(bishop, ChessPiece::Bishop);
    assert_ne!(bishop, ChessPiece::Pawn);
    assert_ne!(bishop, ChessPiece::Rook);
    assert_ne!(bishop, ChessPiece::Knight);
    assert_ne!(bishop, ChessPiece::Queen);
    assert_ne!(bishop, ChessPiece::King);
}

#[test]
fn test_chess_piece_queen() {
    let queen: ChessPiece = ChessPiece::Queen;

    assert_eq!(queen, ChessPiece::Queen);
    assert_ne!(queen, ChessPiece::Pawn);
    assert_ne!(queen, ChessPiece::Rook);
    assert_ne!(queen, ChessPiece::Knight);
    assert_ne!(queen, ChessPiece::Bishop);
    assert_ne!(queen, ChessPiece::King);
}

#[test]
fn test_chess_piece_king() {
    let king: ChessPiece = ChessPiece::King;

    assert_eq!(king, ChessPiece::King);
    assert_ne!(king, ChessPiece::Pawn);
    assert_ne!(king, ChessPiece::Rook);
    assert_ne!(king, ChessPiece::Knight);
    assert_ne!(king, ChessPiece::Bishop);
    assert_ne!(king, ChessPiece::Queen);
}

#[test]
fn test_piece_white_pawn() {
    let piece = Piece::new(ChessPiece::Pawn, ChessColor::White);

    assert_eq!(piece.get_piece(), ChessPiece::Pawn);
    assert_eq!(piece.get_color(), ChessColor::White);
    assert_eq!(piece.get_piece_name(), "Pawn".to_string());
    assert_eq!(piece.get_color_name(), "White".to_string());
}

#[test]
fn test_piece_black_pawn() {
    let piece = Piece::new(ChessPiece::Pawn, ChessColor::Black);

    assert_eq!(piece.get_piece(), ChessPiece::Pawn);
    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_piece_name(), "Pawn".to_string());
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_white_rook() {
    let piece = Piece::new(ChessPiece::Rook, ChessColor::White);

    assert_eq!(piece.get_piece(), ChessPiece::Rook);
    assert_eq!(piece.get_color(), ChessColor::White);
    assert_eq!(piece.get_piece_name(), "Rook".to_string());
    assert_eq!(piece.get_color_name(), "White".to_string());
}

#[test]
fn test_piece_black_rook() {
    let piece = Piece::new(ChessPiece::Rook, ChessColor::Black);

    assert_eq!(piece.get_piece(), ChessPiece::Rook);
    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_piece_name(), "Rook".to_string());
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_white_knight() {
    let piece = Piece::new(ChessPiece::Knight, ChessColor::White);

    assert_eq!(piece.get_piece(), ChessPiece::Knight);
    assert_eq!(piece.get_color(), ChessColor::White);
    assert_eq!(piece.get_piece_name(), "Knight".to_string());
    assert_eq!(piece.get_color_name(), "White".to_string());
}

#[test]
fn test_piece_black_knight() {
    let piece = Piece::new(ChessPiece::Knight, ChessColor::Black);

    assert_eq!(piece.get_piece(), ChessPiece::Knight);
    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_piece_name(), "Knight".to_string());
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_white_bishop() {
    let piece = Piece::new(ChessPiece::Bishop, ChessColor::White);

    assert_eq!(piece.get_piece(), ChessPiece::Bishop);
    assert_eq!(piece.get_color(), ChessColor::White);
    assert_eq!(piece.get_piece_name(), "Bishop".to_string());
    assert_eq!(piece.get_color_name(), "White".to_string());
}

#[test]
fn test_piece_black_bishop() {
    let piece = Piece::new(ChessPiece::Bishop, ChessColor::Black);

    assert_eq!(piece.get_piece(), ChessPiece::Bishop);
    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_piece_name(), "Bishop".to_string());
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_white_queen() {
    let piece = Piece::new(ChessPiece::Queen, ChessColor::White);

    assert_eq!(piece.get_piece(), ChessPiece::Queen);
    assert_eq!(piece.get_color(), ChessColor::White);
    assert_eq!(piece.get_piece_name(), "Queen".to_string());
    assert_eq!(piece.get_color_name(), "White".to_string());
}

#[test]
fn test_piece_black_queen() {
    let piece = Piece::new(ChessPiece::Queen, ChessColor::Black);

    assert_eq!(piece.get_piece(), ChessPiece::Queen);
    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_piece_name(), "Queen".to_string());
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_white_king() {
    let piece = Piece::new(ChessPiece::King, ChessColor::White);

    assert_eq!(piece.get_piece(), ChessPiece::King);
    assert_eq!(piece.get_color(), ChessColor::White);
    assert_eq!(piece.get_piece_name(), "King".to_string());
    assert_eq!(piece.get_color_name(), "White".to_string());
}

#[test]
fn test_piece_black_king() {
    let piece = Piece::new(ChessPiece::King, ChessColor::Black);

    assert_eq!(piece.get_piece(), ChessPiece::King);
    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_piece_name(), "King".to_string());
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_set_piece() {
    let mut piece = Piece::new(ChessPiece::Pawn, ChessColor::White);

    piece.set_piece(ChessPiece::Rook);

    assert_eq!(piece.get_piece(), ChessPiece::Rook);
    assert_eq!(piece.get_piece_name(), "Rook".to_string());
}

#[test]
fn test_piece_set_color() {
    let mut piece = Piece::new(ChessPiece::Pawn, ChessColor::White);

    piece.set_color(ChessColor::Black);

    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_piece_set_piece_name() {
    let mut piece = Piece::new(ChessPiece::Pawn, ChessColor::White);

    piece.set_piece_name("Rook".to_string());

    assert_eq!(piece.get_piece(), ChessPiece::Rook);
    assert_eq!(piece.get_piece_name(), "Rook".to_string());
}

#[test]
fn test_piece_set_color_name() {
    let mut piece = Piece::new(ChessPiece::Pawn, ChessColor::White);

    piece.set_color_name("Black".to_string());

    assert_eq!(piece.get_color(), ChessColor::Black);
    assert_eq!(piece.get_color_name(), "Black".to_string());
}

#[test]
fn test_chessboard_parser_new() {
    let parser = ChessboardParser::new();

    assert_eq!(parser.get_board(), ChessBoard::new());
}

#[test]
fn test_chessboard_parser_set_board() {
    let mut parser = ChessboardParser::new();
    let board = ChessBoard::new();

    parser.set_board(board.clone());

    assert_eq!(parser.get_board(), board);
}

#[test]
fn test_chessboard_parser_get_case() {
    let mut parser = ChessboardParser::new();

    parser.parse_fen("8/6N1/8/4P2k/3PK3/5r2/6R1/8".to_string());
    println!("{}", parser.get_board());
    println!("{}", BOARD1.clone());
    assert_eq!(parser.get_board(), BOARD1.clone());
}