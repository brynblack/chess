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

use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};
use chess::{Board, Colour, Coord, Square};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initial_setup)
        .add_system(print_mouse_events_system)
        .run();
    // Everything after this function call is unreachable!
}

// This function runs only once on startup
fn initial_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a new board with default layout
    let mut board = Board::new(Board::default());

    // Move a piece (For testing purposes only)
    board
        .move_piece(&Coord { x: 0, y: 1 }, &Coord { x: 0, y: 2 })
        .unwrap_or_else(|err| eprintln!("{}", err));

    let square_size = 60.0;
    let temp_piece_size = 0.4;

    // Render the board
    // TODO: Center the board on the screen
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for row in 0..board.layout.len() {
        for column in 0..board.layout.len() {
            // Alternate the square colour
            let square_colour = if (row + column) % 2 == 0 {
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
                    square_size * column as f32 - (0.5 * square_size * 8.0),
                    0.0,
                ),
                ..default()
            });

            // Render each chess piece
            match board.get_square(&Coord { x: row, y: column }) {
                Square::Empty => (),
                square => {
                    // Render based on piece colour
                    // TODO: Create macro to automate the match
                    let piece_texture = match square {
                        Square::King(Colour::Black) => asset_server.load("../assets/bk.png"),
                        Square::Pawn(Colour::Black) => asset_server.load("../assets/bp.png"),
                        Square::Bishop(Colour::Black) => asset_server.load("../assets/bb.png"),
                        Square::Knight(Colour::Black) => asset_server.load("../assets/bn.png"),
                        Square::Rook(Colour::Black) => asset_server.load("../assets/br.png"),
                        Square::Queen(Colour::Black) => asset_server.load("../assets/bq.png"),

                        Square::King(Colour::White) => asset_server.load("../assets/wk.png"),
                        Square::Pawn(Colour::White) => asset_server.load("../assets/wp.png"),
                        Square::Bishop(Colour::White) => asset_server.load("../assets/wb.png"),
                        Square::Knight(Colour::White) => asset_server.load("../assets/wn.png"),
                        Square::Rook(Colour::White) => asset_server.load("../assets/wr.png"),
                        Square::Queen(Colour::White) => asset_server.load("../assets/wq.png"),

                        _ => continue,
                    };

                    commands.spawn_bundle(SpriteBundle {
                        texture: piece_texture,
                        transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                            Vec3::new(temp_piece_size, temp_piece_size, 1.0),
                            Quat::IDENTITY,
                            Vec3::new(
                                square_size * row as f32 - (square_size * 8.0),
                                square_size * column as f32 - (0.5 * square_size * 8.0),
                                1.0,
                            ),
                        )),
                        ..default()
                    });
                }
            }
        }
    }
}

fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!(
            "Mouse location x:{} y:{}",
            event.position.x, event.position.y
        );
    }
}
