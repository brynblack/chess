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

use bevy::prelude::*;
use chess::{Board, Colour, Coord, Square};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initial_setup)
        .run();
    // Everything after this function call is unreachable!
}

// This function runs only once on startup
fn initial_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Create a new board with default layout
    let mut board = Board::new(Board::default());

    // Move a piece (For testing purposes only)
    board
        .move_piece(&Coord { x: 0, y: 6 }, &Coord { x: 0, y: 5 })
        .unwrap_or_else(|err| eprintln!("{}", err));

    let square_size = 60.0;
    let temp_piece_size = 40.0;

    // Render the board
    // TODO: Center the board on the screen
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for row in 0..board.layout.len() {
        for square in 0..board.layout.len() {
            // Alternate the square colour
            let square_colour = if (row + square) % 2 == 0 {
                Color::rgb(0.46, 0.59, 0.34)
            } else {
                Color::rgb(0.93, 0.93, 0.82)
            };

            // Render each square
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: square_colour,
                    custom_size: Some(Vec2::new(square_size, square_size)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    square_size * row as f32 - (square_size * 8.0),
                    square_size * square as f32 - (0.5 * square_size * 8.0),
                    0.0,
                ),
                ..default()
            });

            // Render each chess piece
            match board.layout[square][row] {
                Square::Empty => (),
                a => {
                    // Display each piece being rendered (For debugging purposes)
                    println!("{:?}", a);

                    // Render based on piece colour
                    // TODO: Create macro to automate the match
                    let piece_colour = match a {
                        Square::Bishop(Colour::Black)
                        | Square::King(Colour::Black)
                        | Square::Knight(Colour::Black)
                        | Square::Pawn(Colour::Black)
                        | Square::Queen(Colour::Black)
                        | Square::Rook(Colour::Black) => Color::rgb(0.0, 0.0, 0.0),
                        Square::Bishop(Colour::White)
                        | Square::King(Colour::White)
                        | Square::Knight(Colour::White)
                        | Square::Pawn(Colour::White)
                        | Square::Queen(Colour::White)
                        | Square::Rook(Colour::White) => Color::rgb(1.0, 1.0, 1.0),
                        _ => Color::rgb(0.0, 0.0, 0.0),
                    };

                    commands.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: piece_colour,
                            custom_size: Some(Vec2::new(temp_piece_size, temp_piece_size)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            square_size * row as f32 - (square_size * 8.0),
                            square_size * square as f32 - (0.5 * square_size * 8.0),
                            1.0,
                        ),
                        ..default()
                    });
                }
            }
        }
    }
}
