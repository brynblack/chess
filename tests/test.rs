use chess::board::{Board, Move, PieceColour, PieceKind, Position, Square};

#[test]
fn piece_move() {
    let mut board = Board::default();

    if let Err(err) = board.move_piece(Move {
        old_pos: Position { x: 0, y: 6 },
        new_pos: Position { x: 0, y: 5 },
    }) {
        eprintln!("{}", err)
    }

    assert_eq!(
        board.layout(),
        &vec![
            vec![
                Square::Piece {
                    piece_kind: PieceKind::Rook,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Knight,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Bishop,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Queen,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::King,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Bishop,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Knight,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Rook,
                    piece_colour: PieceColour::Black,
                },
            ],
            vec![
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::Black,
                },
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Pawn,
                    piece_colour: PieceColour::White,
                },
            ],
            vec![
                Square::Piece {
                    piece_kind: PieceKind::Rook,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Queen,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::King,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_kind: PieceKind::Rook,
                    piece_colour: PieceColour::White,
                },
            ],
        ]
    )
}
