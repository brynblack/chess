// A structure defining a chess board
#[derive(PartialEq)]
pub struct Board {
    pub layout: [[Square; 8]; 8],
}

impl Board {
    // Returns a new chessboard with specified layout
    pub fn new(layout: [[Square; 8]; 8]) -> Board {
        Board { layout }
    }

    // Returns standard chessboard layout
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

    // Sets the value of a given square
    fn set_square(&mut self, coord: &Coord, square: Square) {
        self.layout[self.layout.len() - 1 - coord.y][coord.x] = square;
    }

    // Gets the value of a given square
    pub fn get_square(&self, coord: &Coord) -> Square {
        self.layout[self.layout.len() - 1 - coord.y][coord.x]
    }

    // Moves a selected piece to a new position on the chess board
    // TODO: Implement checks for valid move based on piece type
    pub fn move_piece(&mut self, prev_coord: &Coord, new_coord: &Coord) -> Result<(), &str> {
        if (prev_coord.x | prev_coord.y | new_coord.x | new_coord.y) > self.layout.len() - 1 {
            return Err("Specified coordinates are out of bounds");
        }
        match self.get_square(prev_coord) {
            Square::Empty => Err("Selected square is empty"),
            square => {
                self.set_square(prev_coord, Square::Empty);
                self.set_square(new_coord, square);
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    Black,
    White,
}

// An enum defining all the possible states for a square on the chess board
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Square {
    Empty,
    Bishop(Colour),
    King(Colour),
    Knight(Colour),
    Pawn(Colour),
    Queen(Colour),
    Rook(Colour),
}
