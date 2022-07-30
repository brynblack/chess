//! A module for the creation and management of a chessboard.

use bevy::prelude::Component;
use std::mem;

type BoardLayout = Vec<Vec<Square>>;
type MoveList = Vec<Move>;
type ValidMoves = Vec<Move>;

/// A struct representing a position on the chessboard.
///
/// Contains two fields, an `x` coordinate and a `y` coordinate,
/// which both are of type `usize`.
///
/// # Examples
///
/// ```
/// use chess::board::Position;
///
/// let example_coord = Position {
///     x: 2,
///     y: 1,
/// };
/// ```
#[derive(Clone, Copy, Component, Debug)]
pub struct Position {
    /// The x-coordinate.
    pub x: usize,
    /// The y-coordinate.
    pub y: usize,
}

/// A struct representing a piece movement.
///
/// Contains two fields, the old and new position of a piece, which are of type `Position`.
///
/// # Examples
///
/// ```
/// use chess::board::{Move, Position};
///
/// let example_move = Move {
///     old_pos: Position { x: 1, y: 2 },
///     new_pos: Position { x: 1, y: 3 },
/// };
/// ```
#[derive(Debug)]
pub struct Move {
    /// The old position of the piece.
    pub old_pos: Position,
    /// The new position of the piece.
    pub new_pos: Position,
}

/// An enum representing the possible colours of a piece.
///
/// Contains two variants, `Black` and `White`.
///
/// # Examples
///
/// ```
/// use chess::board::PieceColour;
///
/// let colour = PieceColour::Black;
/// match colour {
///     PieceColour::Black => println!("The colour is black!"),
///     PieceColour::White => println!("The colour is white!"),
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceColour {
    /// The colour black.
    Black,
    /// The colour white.
    White,
}

/// An enum representing the possible types of a piece.
///
/// Contains six variants, all with their own unique set of valid moves.
///
/// # Examples
///
/// ```
/// use chess::board::PieceType;
///
/// let piece = PieceType::Bishop;
/// println!("{:?}", piece.valid_moves());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    /// A Bishop.
    Bishop,
    /// A King.
    King,
    /// A Knight.
    Knight,
    /// A Pawn.
    Pawn,
    /// A Queen.
    Queen,
    /// A Rook.
    Rook,
}

impl PieceType {
    /// Returns a vector of moves that are valid for the piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::PieceType;
    ///
    /// let piece = PieceType::Pawn;
    /// let valid_moves = piece.valid_moves();
    /// println!("{:?}", valid_moves);
    /// ```
    pub fn valid_moves(&self) -> ValidMoves {
        match self {
            PieceType::Bishop => vec![],
            PieceType::King => vec![],
            PieceType::Knight => vec![],
            PieceType::Pawn => vec![],
            PieceType::Queen => vec![],
            PieceType::Rook => vec![],
        }
    }
}

/// An enum representing the possible state of a square.
///
/// Contains two variants, an `Empty` variant and a `Piece` variant.
///
/// # Examples
///
/// ```
/// use chess::board::{PieceColour, PieceType, Square};
///
/// let square = Square::Piece {
///     piece_type: PieceType::Pawn,
///     piece_colour: PieceColour::Black
/// };
/// match square {
///     Square::Piece { piece_colour, piece_type } => {
///         println!("Colour: {:?}, Type: {:?}", piece_colour, piece_type)
///     }
///     Square::Empty => println!("I'm just a lonely empty square :("),
/// };
/// ```
#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum Square {
    /// An empty square.
    Empty,
    /// A square with a piece on it.
    ///
    /// Contains two fields, a `PieceType` and a `PieceColour`.
    Piece {
        /// The type of the piece.
        piece_type: PieceType,
        /// The colour of the piece.
        piece_colour: PieceColour,
    },
}

impl Square {
    /// Returns an optional value containing the colour and type of the piece, if it is a piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::{PieceColour, PieceType, Square};
    ///
    /// let square = Square::Piece {
    ///     piece_type: PieceType::Pawn,
    ///     piece_colour: PieceColour::Black
    /// };
    /// match square.get_piece() {
    ///     Some(piece) => println!("{:?}", piece),
    ///     None => println!("I'm an empty square!"),
    /// };
    /// ```
    pub fn get_piece(&self) -> Option<(&PieceColour, &PieceType)> {
        match self {
            Square::Empty => None,
            Square::Piece {
                piece_colour,
                piece_type,
            } => Some((&piece_colour, &piece_type)),
        }
    }
}

/// A struct representing a chessboard.
///
/// Contains three fields, a layout, the current player, and the move list.
///
/// # Examples
///
/// ```
/// use chess::board::Board;
///
/// let board = Board::default();
/// ```
pub struct Board {
    layout: BoardLayout,
    player: PieceColour,
    move_list: MoveList,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            layout: Layouts::standard(),
            player: PieceColour::White,
            move_list: vec![],
        }
    }
}

impl Board {
    /// Creates a new chessboard with the given configuration.
    pub fn new(layout: BoardLayout, player: PieceColour, move_list: MoveList) -> Self {
        Self {
            layout,
            player,
            move_list,
        }
    }

    /// Returns a reference to the board layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    ///
    /// let board = Board::default();
    /// println!("{:?}", board.get_layout());
    /// ```
    pub fn get_layout(&self) -> &BoardLayout {
        &self.layout
    }

    /// Returns a reference to the current player.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    ///
    /// let board = Board::default();
    /// println!("{:?}", board.get_player());
    /// ```
    pub fn get_player(&self) -> &PieceColour {
        &self.player
    }

    /// Returns a reference to the move list.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    ///
    /// let board = Board::default();
    /// println!("{:?}", board.get_move_list());
    /// ```
    pub fn get_move_list(&self) -> &MoveList {
        &self.move_list
    }

    /// Moves a piece from one position to another
    ///
    /// Takes in a `Move` and returns a result, based upon whether it was
    /// successful or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::{Board, Move, Position};
    ///
    /// let mut board = Board::default();
    /// board.move_piece(Move {
    ///     old_pos: Position { x: 1, y: 1 },
    ///     new_pos: Position { x: 1, y: 2 },
    /// }).unwrap_or_else(|err| eprintln!("{}", err));
    /// ```
    pub fn move_piece(&mut self, piece_move: Move) -> Result<(), &str> {
        if let Err(err) = self.valid_move(&piece_move) {
            return Err(err);
        }
        let moved_piece = mem::replace(
            &mut self.layout[piece_move.old_pos.y][piece_move.old_pos.x],
            Square::Empty,
        );
        self.layout[piece_move.new_pos.y][piece_move.new_pos.x] = moved_piece;
        self.move_list.push(piece_move);
        self.next_turn();
        Ok(())
    }

    /// Checks if a move is valid.
    ///
    /// Takes in a `Move` and returns a result on whether it is valid or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::{Board, Move, Position};
    ///
    /// let board = Board::default();
    /// let example_move = Move {
    ///     old_pos: Position { x: 1, y: 5 },
    ///     new_pos: Position { x: 1, y: 200 },
    /// };
    /// match board.valid_move(&example_move) {
    ///     Ok(_) => (),
    ///     Err(err) => eprintln!("{}", err),
    /// };
    /// ```
    pub fn valid_move<'a>(&self, piece_move: &Move) -> Result<(), &'a str> {
        // Player trying to move out of bounds square
        if piece_move.old_pos.y > self.layout.len() {
            return Err("Error: Origin square is out of bounds!");
        }
        if piece_move.old_pos.x > self.layout[piece_move.old_pos.y].len() {
            return Err("Error: Origin square is out of bounds!");
        }
        if piece_move.new_pos.y > self.layout.len() {
            return Err("Error: Destination square is out of bounds!");
        }
        if piece_move.new_pos.x > self.layout[piece_move.new_pos.y].len() {
            return Err("Error: Destination square is out of bounds!");
        }

        let old_square = self.layout[piece_move.old_pos.y][piece_move.old_pos.x];
        let new_square = self.layout[piece_move.new_pos.y][piece_move.new_pos.x];

        // Player trying to move empty square
        let (piece_colour, piece_type) = match old_square.get_piece() {
            Some(piece) => piece,
            None => return Err("Error: You cannot move an empty square!"),
        };

        // Player trying to move opponent pieces
        if &self.player != piece_colour {
            return Err("Error: You cannot move your opponent's pieces!");
        }

        // Player trying to destroy their own pieces
        if let Some(piece) = new_square.get_piece() {
            if &self.player == piece.0 {
                return Err("Error: You cannot capture your own pieces!");
            }
        }

        // Valid move checks
        // TODO
        piece_type.valid_moves();

        Ok(())
    }

    /// Switches to the next player.
    fn next_turn(&mut self) {
        self.player = match self.player {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }
}

/// Contains pre-made layouts that can be used when configuring a custom chessobard.
pub struct Layouts;

impl Layouts {
    /// The standard chessboard layout.
    ///
    /// Looks like this:
    /// ```text
    /// R N B K Q B N R
    /// P P P P P P P P
    /// ~ ~ ~ ~ ~ ~ ~ ~
    /// ~ ~ ~ ~ ~ ~ ~ ~
    /// ~ ~ ~ ~ ~ ~ ~ ~
    /// ~ ~ ~ ~ ~ ~ ~ ~
    /// P P P P P P P P
    /// R N B K Q B N R
    /// ```
    pub fn standard() -> BoardLayout {
        vec![
            vec![
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Queen,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::King,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::Black,
                },
            ],
            vec![
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
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
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
            ],
            vec![
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Queen,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::King,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::White,
                },
            ],
        ]
    }
}
