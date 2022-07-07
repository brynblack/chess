use chess::layouts::Layouts;
use chess::{Board, Colour, Position, Square};

#[test]
fn piece_move() {
    let mut board = Board::new(Layouts::standard());

    match board.move_piece(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    assert_eq!(
        board.get_layout(),
        [
            [
                Square::Rook(Colour::Black),
                Square::Knight(Colour::Black),
                Square::Bishop(Colour::Black),
                Square::Queen(Colour::Black),
                Square::King(Colour::Black),
                Square::Bishop(Colour::Black),
                Square::Knight(Colour::Black),
                Square::Rook(Colour::Black),
            ],
            [
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Pawn(Colour::White),
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
            ],
            [
                Square::Rook(Colour::White),
                Square::Knight(Colour::White),
                Square::Bishop(Colour::White),
                Square::Queen(Colour::White),
                Square::King(Colour::White),
                Square::Bishop(Colour::White),
                Square::Knight(Colour::White),
                Square::Rook(Colour::White),
            ],
        ]
    )
}
