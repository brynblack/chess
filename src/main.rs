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
use chess::board::{Board, Colour, Coord, Square};
use chess::layouts::Layouts;

// Constants
const SQUARE_SIZE: f32 = 60.0;
const PIECE_SIZE: f32 = 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initial_setup)
        .add_system(print_mouse_events_system)
        .run();
}

fn initial_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a new board with default layout
    let mut board = Board::new(Layouts::standard());

    // Move a piece (For testing purposes only)
    match board.move_piece(&Coord { x: 0, y: 1 }, &Coord { x: 0, y: 2 }) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    // Spawn a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Render the board
    // TODO: Center the board on the screen
    for (index_r, row) in board.get_layout().iter().enumerate() {
        for (index_s, square) in row.iter().enumerate() {
            // Alternate the square colour
            let square_colour = if (index_r + index_s) % 2 == 0 {
                Color::rgb(0.46, 0.59, 0.34)
            } else {
                Color::rgb(0.93, 0.93, 0.82)
            };

            // Render the board
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: square_colour,
                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    SQUARE_SIZE * index_r as f32 - (SQUARE_SIZE * 8.0),
                    SQUARE_SIZE * index_s as f32 - (0.5 * SQUARE_SIZE * 8.0),
                    0.0,
                ),
                ..default()
            });

            // Render each chess piece
            match square {
                Square::Empty => (),
                square => {
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
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(PIECE_SIZE, PIECE_SIZE)),
                            ..Default::default()
                        },
                        texture: piece_texture,
                        transform: Transform::from_translation(
                            Vec3::new(
                                SQUARE_SIZE * index_r as f32 - (SQUARE_SIZE * 8.0),
                                SQUARE_SIZE * index_s as f32 - (0.5 * SQUARE_SIZE * 8.0),
                                1.0,
                            ),
                        ),
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
