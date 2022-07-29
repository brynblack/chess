//! A module for the creation and management of a chessboard.

use bevy::prelude::Component;
use std::mem;

type BoardLayout = Vec<Vec<Square>>;
type MoveList = Vec<Move>;

/// A struct representing a position.
#[derive(Clone, Copy, Component)]
pub struct Position {
    /// The x-coordinate.
    pub x: usize,
    /// The y-coordinate.
    pub y: usize,
}

/// A struct representing a move, with an old and new position of a piece.
pub struct Move {
    /// The old position of the piece.
    pub old_pos: Position,
    /// The new position of the piece.
    pub new_pos: Position,
}

/// An enum representing the possible colours of a piece.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceColour {
    /// The colour black.
    Black,
    /// The colour white.
    White,
}

/// An enum representing the possible types of a piece.
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
    /// Returns the valid moves for the piece.
    pub fn valid_moves(&self) {
        match self {
            PieceType::Bishop => todo!(),
            PieceType::King => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Pawn => todo!(),
            PieceType::Queen => todo!(),
            PieceType::Rook => todo!(),
        }
    }
}

/// An enum representing the possible state of a square.
#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum Square {
    /// An empty square.
    Empty,
    /// A square with a piece on it.
    Piece {
        /// The type of the piece.
        piece_type: PieceType,
        /// The colour of the piece.
        piece_colour: PieceColour,
    },
}

impl Square {
    /// Returns an optional value containing the colour of the piece, if it is a piece.
    pub fn get_colour(&self) -> Option<&PieceColour> {
        match self {
            Square::Empty => None,
            Square::Piece { piece_colour, .. } => Some(&piece_colour),
        }
    }

    /// Returns an optional value containing the type of the piece, if it is a piece.
    pub fn get_type(&self) -> Option<&PieceType> {
        match self {
            Square::Empty => None,
            Square::Piece { piece_type, .. } => Some(&piece_type),
        }
    }
}

/// A struct representing a chessboard.
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
    pub fn get_layout(&self) -> &BoardLayout {
        &self.layout
    }

    /// Returns a reference to the current player.
    pub fn get_player(&self) -> &PieceColour {
        &self.player
    }

    /// Returns a reference to the move list.
    pub fn get_move_list(&self) -> &MoveList {
        &self.move_list
    }

    /// Moves a piece from one position to another, returning a result based upon whether it was
    /// successful or not.
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
    fn valid_move<'a>(&self, piece_move: &Move) -> Result<(), &'a str> {
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

        // Player trying to move empty square
        if self.layout[piece_move.old_pos.y][piece_move.old_pos.x] == Square::Empty {
            return Err("Error: You cannot move an empty square!");
        }

        // Player trying to move opponent pieces
        if &self.player
            != self.layout[piece_move.old_pos.y][piece_move.old_pos.x]
                .get_colour()
                .unwrap()
        {
            return Err("Error: You cannot move your opponent's pieces!");
        }

        // Player trying to destroy their own pieces
        if let Some(colour) = self.layout[piece_move.new_pos.y][piece_move.new_pos.x].get_colour() {
            if &self.player == colour {
                return Err("Error: You cannot capture your own pieces!");
            }
        }

        // Valid move checks

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
