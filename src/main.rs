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

use chess::{Board, Coord};

fn main() {
    // Create a new board with default layout
    let mut board = Board::new(Board::default());

    // Move a piece
    board
        .move_piece(&Coord { x: 0, y: 6 }, &Coord { x: 0, y: 5 })
        .unwrap_or_else(|err| eprintln!("{}", err));

    println!("{:?}", &board.layout);
}
