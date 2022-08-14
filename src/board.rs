// Copyright (C) 2022  Brynley Llewellyn-Roux and Aryan Jassal
//
// This file is part of chess.
//
// chess is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// chess is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! A module for the creation and management of a chessboard.

use bevy::prelude::Component;
use std::mem;

type BoardLayout = Vec<Vec<Square>>;
type MoveList = Vec<Move>;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColour {
    /// The colour black.
    Black,
    /// The colour white.
    White,
}

/// An enum representing the possible kinds of a piece.
///
/// Contains six variants, all with their own unique set of valid moves.
///
/// # Examples
///
/// ```
/// use chess::board::PieceKind;
///
/// let piece = PieceKind::Bishop;
/// println!("{:?}", piece.valid_moves());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
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

impl PieceKind {
    /// Checks if a move is valid or not.
    ///
    /// Returns true if the move is valid, or false if it is invalid.
    pub fn is_move_valid(&self, pos: &Move) -> bool {
        match self {
            PieceKind::King => {
                ((pos.old_pos.x as i8 - pos.new_pos.x as i8).abs() == 1
                    && (pos.old_pos.y == pos.new_pos.y))
                    || ((pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 1
                        && (pos.old_pos.x == pos.new_pos.x))
                    || ((pos.old_pos.x as i8 - pos.new_pos.x as i8).abs() == 1
                        && (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 1)
            }
            PieceKind::Knight => {
                ((pos.old_pos.x as i8 - pos.new_pos.x as i8).abs() == 2
                    && (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 1)
                    || ((pos.old_pos.x as i8 - pos.new_pos.x as i8).abs() == 1
                        && (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 2)
            }
            PieceKind::Pawn => {
                ((pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 1)
                    && (pos.old_pos.x == pos.new_pos.x)
                    || ((pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 2)
                        && (pos.old_pos.x == pos.new_pos.x)
            }
            _ => true,
            // PieceKind::Queen => {
            //     is_path_empty((pos.old_pos.x, pos.old_pos.y), new_position, &pieces)
            //         && ((pos.old_pos.x as i8 - pos.new_pos.x as i8).abs()
            //             == (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs()
            //             || ((pos.old_pos.x == pos.new_pos.x && pos.old_pos.y != pos.new_pos.y)
            //                 || (pos.old_pos.y == pos.new_pos.y && pos.old_pos.x != pos.new_pos.x)))
            // }
            // PieceKind::Bishop => {
            //     is_path_empty((pos.old_pos.x, pos.old_pos.y), new_position, &pieces)
            //         && (pos.old_pos.x as i8 - pos.new_pos.x as i8).abs()
            //             == (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs()
            // }
            // PieceKind::Rook => {
            //     is_path_empty((pos.old_pos.x, pos.old_pos.y), new_position, &pieces)
            //         && ((pos.old_pos.x == pos.new_pos.x && pos.old_pos.y != pos.new_pos.y)
            //             || (pos.old_pos.y == pos.new_pos.y && pos.old_pos.x != pos.new_pos.x))
            // }
            // PieceKind::Pawn => {
            //     if colour == PieceColour::White {
            //         // Normal move
            //         if pos.new_pos.x as i8 - pos.old_pos.x as i8 == 1
            //             && (pos.old_pos.y == pos.new_pos.y)
            //         {
            //             if color_of_square(new_position, &pieces).is_none() {
            //                 return true;
            //             }
            //         }

            //         // Move 2 squares
            //         if pos.old_pos.x == 1
            //             && pos.new_pos.x as i8 - pos.old_pos.x as i8 == 2
            //             && (pos.old_pos.y == pos.new_pos.y)
            //             && is_path_empty((pos.old_pos.x, pos.old_pos.y), new_position, &pieces)
            //         {
            //             if color_of_square(new_position, &pieces).is_none() {
            //                 return true;
            //             }
            //         }

            //         // Take piece
            //         if pos.new_pos.x as i8 - pos.old_pos.x as i8 == 1
            //             && (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 1
            //         {
            //             if color_of_square(new_position, &pieces) == Some(PieceColour::Black) {
            //                 return true;
            //             }
            //         }
            //     } else {
            //         // Normal move
            //         if pos.new_pos.x as i8 - pos.old_pos.x as i8 == -1
            //             && (pos.old_pos.y == pos.new_pos.y)
            //         {
            //             if color_of_square(new_position, &pieces).is_none() {
            //                 return true;
            //             }
            //         }

            //         // Move 2 squares
            //         if pos.old_pos.x == 6
            //             && pos.new_pos.x as i8 - pos.old_pos.x as i8 == -2
            //             && (pos.old_pos.y == pos.new_pos.y)
            //             && is_path_empty((pos.old_pos.x, pos.old_pos.y), new_position, &pieces)
            //         {
            //             if color_of_square(new_position, &pieces).is_none() {
            //                 return true;
            //             }
            //         }

            //         // Take piece
            //         if pos.new_pos.x as i8 - pos.old_pos.x as i8 == -1
            //             && (pos.old_pos.y as i8 - pos.new_pos.y as i8).abs() == 1
            //         {
            //             if color_of_square(new_position, &pieces) == Some(PieceColour::White) {
            //                 return true;
            //             }
            //         }
            //     }

            //     false
            // }
        }
    }

    /// Returns the value of the piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::PieceKind;
    ///
    /// let piece = PieceKind::Pawn;
    /// let value = piece.value();
    /// println!("{}", value);
    /// ```
    pub fn value(&self) -> u8 {
        match self {
            PieceKind::Bishop => 3,
            PieceKind::King => 0,
            PieceKind::Knight => 3,
            PieceKind::Pawn => 1,
            PieceKind::Queen => 9,
            PieceKind::Rook => 5,
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
/// use chess::board::{PieceColour, PieceKind, Square};
///
/// let square = Square::Piece {
///     piece_colour: PieceColour::Black,
///     piece_kind: PieceKind::Pawn,
/// };
///
/// match square {
///     Square::Piece { piece_colour, piece_kind } => {
///         println!("Colour: {:?}, Type: {:?}", piece_colour, piece_kind)
///     }
///     Square::Empty => println!("I'm just a lonely empty square :("),
/// };
/// ```
#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub enum Square {
    /// An empty square.
    Empty,
    /// A square with a piece on it.
    ///
    /// Contains two fields, a `PieceColour` and a `PieceKind`.
    Piece {
        /// The colour of the piece.
        piece_colour: PieceColour,
        /// The kind of the piece.
        piece_kind: PieceKind,
    },
}

impl Square {
    /// Returns an optional value containing the colour of the piece, if it is a piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::{PieceColour, PieceKind, Square};
    ///
    /// let square = Square::Piece {
    ///     piece_colour: PieceColour::Black,
    ///     piece_kind: PieceKind::Pawn,
    /// };
    ///
    /// match square.colour() {
    ///     Some(piece) => println!("{:?}", piece),
    ///     None => println!("I'm an empty square!"),
    /// };
    /// ```
    pub fn colour(&self) -> Option<&PieceColour> {
        match self {
            Square::Empty => None,
            Square::Piece { piece_colour, .. } => Some(piece_colour),
        }
    }

    /// Returns an optional value containing the kind of the piece, if it is a piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::{PieceColour, PieceKind, Square};
    ///
    /// let square = Square::Piece {
    ///     piece_colour: PieceColour::Black,
    ///     piece_kind: PieceKind::Pawn,
    /// };
    ///
    /// match square.kind() {
    ///     Some(piece) => println!("{:?}", piece),
    ///     None => println!("I'm an empty square!"),
    /// };
    /// ```
    pub fn kind(&self) -> Option<&PieceKind> {
        match self {
            Square::Empty => None,
            Square::Piece { piece_kind, .. } => Some(piece_kind),
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
    move_list: MoveList,
    player: PieceColour,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            layout: Layouts::standard(),
            move_list: vec![],
            player: PieceColour::White,
        }
    }
}

impl Board {
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
    /// match board.check_valid(&example_move) {
    ///     Ok(_) => (),
    ///     Err(err) => eprintln!("{}", err),
    /// };
    /// ```
    pub fn check_valid<'a>(&self, piece_move: &Move) -> Result<(), &'a str> {
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
        let piece_colour = match old_square.colour() {
            Some(piece) => piece,
            None => return Err("Error: You cannot move an empty square!"),
        };

        // Player trying to move opponent pieces
        if &self.player != piece_colour {
            return Err("Error: You cannot move your opponent's pieces!");
        }

        // Player trying to destroy their own pieces
        if let Some(colour) = new_square.colour() {
            if &self.player == colour {
                return Err("Error: You cannot capture your own pieces!");
            }
        }

        // TODO: Valid move checks
        // piece_kind.valid_moves();
        if !old_square.kind().unwrap().is_move_valid(piece_move) {
            return Err("Error: Move is invalid!");
        };

        Ok(())
    }

    /// Returns a reference to the board layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    ///
    /// let board = Board::default();
    /// println!("{:?}", board.layout());
    /// ```
    pub fn layout(&self) -> &BoardLayout {
        &self.layout
    }

    /// Returns a reference to the move list.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    ///
    /// let board = Board::default();
    /// println!("{:?}", board.move_list());
    /// ```
    pub fn move_list(&self) -> &MoveList {
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
        if let Err(err) = self.check_valid(&piece_move) {
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

    /// Creates a new chessboard with the given configuration.
    pub fn new(layout: BoardLayout, move_list: MoveList, player: PieceColour) -> Self {
        Self {
            layout,
            move_list,
            player,
        }
    }

    /// Switches to the next player.
    fn next_turn(&mut self) {
        self.player = match self.player {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }

    /// Returns a reference to the current player.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    ///
    /// let board = Board::default();
    /// println!("{:?}", board.player());
    /// ```
    pub fn player(&self) -> &PieceColour {
        &self.player
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
    }
}
