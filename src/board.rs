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
    }

    // Moves a selected piece to a new position on the chess board
    // TODO: Implement checks for valid move based on piece type
    pub fn move_piece(&mut self, prev_coord: &Coord, new_coord: &Coord) -> Result<(), &str> {
        let square = self.layout[prev_coord.y][prev_coord.x];
        match square {
            Square::Empty => Err("Selected square is empty"),
            _ => {
                self.layout[prev_coord.y][prev_coord.x] = Square::Empty;
                self.layout[new_coord.y][new_coord.x] = square;
                Ok(())
            }
        }
    }
}

// A structure defining a coordinate on the chess board
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

// An enum describing the colour of each piece i.e. which player owns it
#[derive(Copy, Clone, Debug)]
pub enum Colour {
    Black,
    White,
}

// An enum defining all the possible states for a square on the chess board
#[derive(Copy, Clone, Debug)]
pub enum Square {
    Bishop(Colour),
    Empty,
    King(Colour),
    Knight(Colour),
    Pawn(Colour),
    Rook(Colour),
    Queen(Colour),
}
