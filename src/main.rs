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

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use chess::{
    board::{Board, Colour, Coord, Square},
    layouts::Layouts,
};

// Constants
const LIGHT_COLOUR: Color = Color::rgb(0.46, 0.59, 0.34);
const DARK_COLOUR: Color = Color::rgb(0.93, 0.93, 0.82);

// Bundle defining a square
#[derive(Bundle, Component)]
struct SquareBundle {
    square: Square,
    coord: Coord,
}

// Global variables
struct GameState {
    board: Board,
    player: Colour,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Chess".to_string(),
            ..Default::default()
        })
        .insert_resource(GameState {
            board: Board::new(Layouts::standard()),
            player: Colour::White,
        })
        .add_startup_system(setup)
        .add_startup_system(spawn_board.after(setup))
        .add_startup_system(spawn_pieces.after(setup).after(spawn_board))
        .add_system(resize_scene)
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<GameState>) {
    // Spawn a new 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // TESTING PURPOSES ONLY
    match game_state
        .board
        .move_piece(&Coord { x: 0, y: 1 }, &Coord { x: 0, y: 2 })
    {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
    windows: ResMut<Windows>,
) {
    for (y, rank) in game_state.board.get_layout().iter().enumerate() {
        for (x, square) in rank.iter().enumerate() {
            // TODO: REFACTOR THIS SIZE CODE
            let window = windows.get_primary().unwrap();
            let size = window.height() / (rank.len() as f32);

            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1.0),
                            flip: false,
                        }))
                        .into(),
                    transform: Transform::from_xyz(
                        (x as f32 * size) - window.width() / 2.0 + (size / 2.0),
                        (y as f32 * size) - window.height() / 2.0 + (size / 2.0),
                        0.0,
                    ),
                    material: materials.add(ColorMaterial::from(if (y + x) % 2 == 0 {
                        LIGHT_COLOUR
                    } else {
                        DARK_COLOUR
                    })),
                    ..default()
                })
                .insert_bundle(SquareBundle {
                    square: *square,
                    coord: Coord { x: x, y: y },
                });
        }
    }
}

fn spawn_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
) {
    for (y, rank) in game_state.board.get_layout().iter().enumerate() {
        for (x, square) in rank.iter().enumerate() {
            let piece_texture = match square {
                Square::Empty => continue,
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
            };

            // TODO: REFACTOR THIS SIZE CODE
            let window = windows.get_primary().unwrap();
            let size = window.height() / (rank.len() as f32);

            // INVERT PIECES
            // let transform_y = piece_size * i as f32 - (((window.height() - (PADDING_Y * 2.0)) / 2.0) - (SQUARE_SIZE / 2.0));

            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1.0),
                            flip: false,
                        }))
                        .into(),
                    transform: Transform::from_xyz(
                        (x as f32 * size) - window.width() / 2.0 + (size / 2.0),
                        (y as f32 * size) - window.height() / 2.0 + (size / 2.0),
                        1.0,
                    ),
                    material: materials.add(ColorMaterial::from(piece_texture)),
                    ..default()
                })
                .insert_bundle(SquareBundle {
                    square: *square,
                    coord: Coord { x: x, y: y },
                });
        }
    }
}

// Resizes the chessboard and pieces when the window is resized
fn resize_scene(
    mut resize_event: EventReader<WindowResized>,
    game_state: Res<GameState>,
    mut query: Query<(&mut Transform, &Coord, &Square)>,
) {
    // When the window is resized...
    for event in resize_event.iter() {
        // Calculate the new size
        let size = event.height / (game_state.board.get_layout().len() as f32);

        for (mut transform, coord, square) in query.iter_mut() {
            // Resize each sprite to the correct size
            transform.scale = Vec3::new(size, size, 0.0);

            // Determine which z-index to use
            let z_index = match square {
                Square::Empty => 0.0,
                _ => 1.0,
            };

            // Transform each sprite to correct position
            transform.translation = Vec3::new(
                (coord.x as f32 * size) - event.width / 2.0 + (size / 2.0),
                (coord.y as f32 * size) - event.height / 2.0 + (size / 2.0),
                z_index,
            );
        }

        println!("{:?}, {:?}", event.width, event.height);
    }
}

// https://github.com/bevyengine/bevy/blob/latest/examples/2d/move_sprite.rs
// fn flip_board(
//     mut commands: Commands,
//     mut query: Query<&mut Transform, With<Piec>>,
//     game_state: Res<GameState>,
//     windows: ResMut<Windows>,
// ) {
//     // for mut piece_transform in query.iter_mut() {
//     //     piece_transform.translation = match game_state.player {
//     //         Colour::White => Vec3::new((j as f32 * piece_size) - window.width() / 2.0 + (piece_size / 2.0), (i as f32 * piece_size) - window.height() / 2.0 + (piece_size / 2.0), 0.0),
//     //         Colour::Black => Vec3::new((j as f32 * piece_size) - window.width() / 2.0 + (piece_size / 2.0), (i as f32 * piece_size) - window.height() / 2.0 + (piece_size / 2.0), 0.0),
//     //     };
//     // }
// }

fn drag_pieces() {
    todo!()
}
