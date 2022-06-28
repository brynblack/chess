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
use chess::{Board, Coord};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initial_setup)
        .run();
}

fn initial_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Create a new board with default layout
    let mut board = Board::new(Board::default());

    // Move a piece
    board
        .move_piece(&Coord { x: 0, y: 6 }, &Coord { x: 0, y: 5 })
        .unwrap_or_else(|err| eprintln!("{}", err));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let square_size = 60.0;
    let temp_piece_size = 40.0;

    for i in 1..=8 {
        for j in 1..=8 {
            let square_colour = if (i + j) % 2 == 0 {
                Color::rgb(0.46, 0.59, 0.34)
            } else {
                Color::rgb(0.93, 0.93, 0.82)
            };
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: square_colour,
                    custom_size: Some(Vec2::new(square_size, square_size)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    square_size * i as f32 - (square_size * 8.0),
                    square_size * j as f32 - (0.5 * square_size * 8.0),
                    0.0,
                ),
                ..default()
            });

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(temp_piece_size, temp_piece_size)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    square_size * i as f32 - (square_size * 8.0),
                    square_size * j as f32 - (0.5 * square_size * 8.0),
                    1.0,
                ),
                ..default()
            });
        }
    }
}
