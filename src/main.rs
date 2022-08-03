// chess - A Rust implementation of the famous game Chess.
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

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowMode, WindowResized},
};
use chess::board::{Board, Move, PieceColour, PieceKind, Position, Square};

const LIGHT_COLOUR: Color = Color::rgb(0.93, 0.93, 0.82);
const DARK_COLOUR: Color = Color::rgb(0.46, 0.59, 0.34);
const SQUARE_SIZE: f32 = 64.0;

#[derive(Default, Component)]
struct CursorState {
    position: Vec2,
    piece: Option<(Entity, Vec3)>,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chess".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .init_resource::<Board>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_dimensions)
        .add_system(drag_and_drop)
        .run();
}

fn setup(
    mut commands: Commands,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    board.layout().iter().enumerate().for_each(|(y, rank)| {
        rank.iter().enumerate().for_each(|(x, square)| {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1.0),
                            flip: false,
                        }))
                        .into(),
                    material: materials.add(ColorMaterial::from(if (y + x) % 2 == 0 {
                        DARK_COLOUR
                    } else {
                        LIGHT_COLOUR
                    })),
                    transform: Transform::from_xyz(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        0.0,
                    )
                    .with_scale(Vec3::splat(SQUARE_SIZE)),
                    ..default()
                })
                .insert(Position { x, y });

            let piece_texture = match square {
                Square::Empty => return,
                Square::Piece {
                    piece_kind: piece_type,
                    piece_colour,
                } => match piece_colour {
                    PieceColour::Black => match piece_type {
                        PieceKind::King => asset_server.load("../assets/bk.png"),
                        PieceKind::Pawn => asset_server.load("../assets/bp.png"),
                        PieceKind::Bishop => asset_server.load("../assets/bb.png"),
                        PieceKind::Knight => asset_server.load("../assets/bn.png"),
                        PieceKind::Rook => asset_server.load("../assets/br.png"),
                        PieceKind::Queen => asset_server.load("../assets/bq.png"),
                    },
                    PieceColour::White => match piece_type {
                        PieceKind::King => asset_server.load("../assets/wk.png"),
                        PieceKind::Pawn => asset_server.load("../assets/wp.png"),
                        PieceKind::Bishop => asset_server.load("../assets/wb.png"),
                        PieceKind::Knight => asset_server.load("../assets/wn.png"),
                        PieceKind::Rook => asset_server.load("../assets/wr.png"),
                        PieceKind::Queen => asset_server.load("../assets/wq.png"),
                    },
                },
            };

            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1.0),
                            flip: false,
                        }))
                        .into(),
                    material: materials.add(ColorMaterial::from(piece_texture)),
                    transform: Transform::from_xyz(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        1.0,
                    )
                    .with_scale(Vec3::splat(SQUARE_SIZE)),
                    ..default()
                })
                .insert_bundle((Position { x, y }, *square));
        })
    });

    commands.spawn_bundle(Camera2dBundle::default());
}

fn update_dimensions(
    mut window_resized_event: EventReader<WindowResized>,
    board: Res<Board>,
    entities: Query<Entity, With<Position>>,
    mut transforms: Query<&mut Transform>,
    piece_types: Query<&Square>,
    positions: Query<&Position>,
) {
    window_resized_event.iter().for_each(|event| {
        // Calculate new piece and square size, then calculate the centre of the window
        let w_size = event.width / board.layout().len() as f32;
        let h_size = event.height / board.layout().len() as f32;
        let size = w_size.total_cmp(&h_size);
        let size = match size {
            std::cmp::Ordering::Greater => h_size,
            _ => w_size,
        };

        // Update the size of the squares and pieces and translate them to correct position
        entities.iter().for_each(|entity| {
            // Update the size of the entity
            let mut transform = transforms.get_mut(entity).unwrap();
            transform.scale = Vec3::splat(size);

            // Evaluate the correct z-index for the type of entity
            let z_index = match piece_types.get(entity) {
                Ok(_) => 1.0,
                Err(_) => 0.0,
            };

            // Translate the entity to the correct position
            let position = positions.get(entity).unwrap();
            transform.translation = Vec3::new(
                position.x as f32 * size - event.width / 2.0 + (size / 2.0),
                position.y as f32 * size - event.height / 2.0 + (size / 2.0),
                z_index,
            )
        });
    });
}

fn drag_and_drop(
    mut cursor_moved_event: EventReader<CursorMoved>,
    windows: Res<Windows>,
    mut cursor_state: Local<CursorState>,
    mouse_button_input: Res<Input<MouseButton>>,
    pieces: Query<Entity, (With<Position>, With<Square>)>,
    squares: Query<Entity, (With<Position>, Without<Square>)>,
    mut transforms: Query<&mut Transform>,
    mut positions: Query<&mut Position>,
    mut board: ResMut<Board>,
) {
    // If the cursor moves, calculate the position of the cursor relative to the origin of the chessboard
    if let Some(cursor_event) = cursor_moved_event.iter().last() {
        let window = windows.get_primary().unwrap();
        let window_centre = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        cursor_state.position = cursor_event.position - window_centre;
    };

    if cursor_state.piece.is_some() {
        if mouse_button_input.just_released(MouseButton::Left) {
            let mut closest_square: Option<Entity> = None;
            squares.iter().for_each(|square| {
                let transform = transforms.get(square).unwrap();
                let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
                if diff.length() < (transform.scale.x / 2.0) {
                    closest_square = Some(square);
                }
            });

            if closest_square.is_none() {
                return;
            }

            let piece = cursor_state.piece.unwrap();

            let piece_coord = positions.get(piece.0).unwrap();
            let closest_square_coord = positions.get(closest_square.unwrap()).unwrap();
            let boilerplate = closest_square_coord.clone();

            let piece_size = transforms.get(piece.0).unwrap().scale;
            let mut piece_pos = transforms.get_mut(piece.0).unwrap();

            match board.move_piece(Move {
                old_pos: *piece_coord,
                new_pos: *closest_square_coord,
            }) {
                Ok(_) => {
                    // Update the translation of the piece
                    let window = windows.get_primary().unwrap();
                    let mut piece_coord = positions.get_mut(piece.0).unwrap();
                    piece_coord.x = boilerplate.x;
                    piece_coord.y = boilerplate.y;
                    piece_pos.translation.x = boilerplate.x as f32 * piece_size.x
                        - window.width() / 2.0
                        + (piece_size.x / 2.0);
                    piece_pos.translation.y = boilerplate.y as f32 * piece_size.y
                        - window.height() / 2.0
                        + (piece_size.y / 2.0);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    let window = windows.get_primary().unwrap();
                    piece_pos.translation.x = piece_coord.x as f32 * piece_size.x
                        - window.width() / 2.0
                        + (piece_size.x / 2.0);
                    piece_pos.translation.y = piece_coord.y as f32 * piece_size.y
                        - window.height() / 2.0
                        + (piece_size.y / 2.0);
                }
            }

            cursor_state.piece = None;
            return;
        }

        if mouse_button_input.pressed(MouseButton::Left) {
            let piece = cursor_state.piece.unwrap();
            let mut piece_pos = transforms.get_mut(piece.0).unwrap();

            piece_pos.translation.x = cursor_state.position.x + piece.1.x;
            piece_pos.translation.y = cursor_state.position.y + piece.1.y;
            return;
        }
    }

    // If the left mouse button is pressed, update the cursor state to contain the closest piece to the cursor
    if mouse_button_input.just_pressed(MouseButton::Left) {
        pieces.iter().for_each(|piece| {
            let transform = transforms.get(piece).unwrap();
            let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
            if diff.length() < (transform.scale.x / 2.0) {
                cursor_state.piece = Some((piece, diff));
            }
        });
    }
}

fn cursor_to_piece_diff(cursor_pos: &Vec2, piece_pos: &Vec3) -> Vec3 {
    Vec3::new(piece_pos.x - cursor_pos.x, piece_pos.y - cursor_pos.y, 0.0)
}
