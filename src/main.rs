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
    println!("{:?}", board);
    assert!(check_if_square_empty(&Coord(5, 5), &board))
}

struct Coord(u32, u32);

#[derive(Debug)]
enum Square {
    Bishop,
    Empty,
    King,
    Knight,
    Pawn,
    Rook,
    Queen,
}

// PROBABLY NOT NEEDED
fn check_if_square_empty(coord: &Coord, board: &[[Square; 8]; 8]) -> bool {
    match board[coord.1 as usize][coord.0 as usize] {
        Square::Empty => true,
        _ => false,
    }
}

// TODO
fn move_piece(square: &Square) {
    match square {
        Square::Pawn => println!("test"),
        _ => (),
    }
}
