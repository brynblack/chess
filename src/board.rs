// A structure defining a chess board
pub struct Board {
    pub layout: [[Square; 8]; 8],
}

// The implementation for a chess board
impl Board {
    // Constructor for a new chess board
    pub fn new(layout: [[Square; 8]; 8]) -> Board {
        Board { layout: layout }
    }

    // Returns the standard chess board layout
    pub fn default() -> [[Square; 8]; 8] {
        [
            [
                Square::Rook,
                Square::Knight,
                Square::Bishop,
                Square::Queen,
                Square::King,
                Square::Bishop,
                Square::Knight,
                Square::Rook,
            ],
            [
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
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
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
                Square::Pawn,
            ],
            [
                Square::Rook,
                Square::Knight,
                Square::Bishop,
                Square::Queen,
                Square::King,
                Square::Bishop,
                Square::Knight,
                Square::Rook,
            ],
        ]
    }

    // Moves a selected piece to a new position on the chess board
    pub fn move_piece(&mut self, prev_coord: &Coord, new_coord: &Coord) {
        let square = self.layout[prev_coord.y][prev_coord.x];
        match square {
            Square::Empty => (),
            _ => {
                self.layout[prev_coord.y][prev_coord.x] = Square::Empty;
                self.layout[new_coord.y][new_coord.x] = square;
            }
        }
    }
}

// A structure defining a coordinate on the chess board
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

// An enum defining all the possible states for a square on the chess board
#[derive(Copy, Clone, Debug)]
pub enum Square {
    Bishop,
    Empty,
    King,
    Knight,
    Pawn,
    Rook,
    Queen,
}
