// chess - A Rust implementation of the famous game Chess.
// Copyright (C) 2022  Brynley Llewellyn-Roux and Aryan Jassal
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

fn main() {
    let mut board: [[Square; 8]; 8] = [
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
    ];
    move_piece(&Coord(1, 1), &Coord(2, 3), &mut board);
    println!("{:?}", board)
}

struct Coord(usize, usize);

#[derive(Copy, Clone, Debug)]
enum Square {
    Bishop,
    Empty,
    King,
    Knight,
    Pawn,
    Rook,
    Queen,
}

fn move_piece(prev_coord: &Coord, new_coord: &Coord, board: &mut[[Square; 8]; 8]) {
    let square = board[prev_coord.1][prev_coord.0];
    match square {
        Square::Empty => (),
        Square::Pawn => {
            if new_coord.1 == prev_coord.1 + 1 {
                board[prev_coord.1][prev_coord.0] = Square::Empty;
                board[new_coord.1][new_coord.0] = square;
            }
        },
        _ => (),
    }
}
