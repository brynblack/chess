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
