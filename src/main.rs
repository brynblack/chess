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

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowMode, input::mouse::MouseButtonInput};
use chess::board::{Board, Colour, Position, Square};
use chess::layouts::Layouts;

const LIGHT_COLOUR: Color = Color::rgb(0.93, 0.93, 0.82);
const DARK_COLOUR: Color = Color::rgb(0.46, 0.59, 0.34);
const SQUARE_SIZE: f32 = 64.;

struct GameState {
    board: Board,
    player: Colour,
}

#[derive(Default, Component)]
struct CursorState {
    pos: Vec2,
    piece: Option<(Entity, Vec3)>,
}

#[derive(Bundle, Component)]
struct SquareComp {
    pos: Position,
}

#[derive(Bundle, Component)]
struct PieceComp {
    pos: Position,
    piece_type: Square,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            board: Board::new(Layouts::standard()),
            player: Colour::White,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chess".to_string(),
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(drag_and_drop)
        .run();
}

fn setup(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for (y, rank) in game_state.board.get_layout().iter().rev().enumerate() {
        for (x, square) in rank.iter().enumerate() {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1.),
                            flip: false,
                        }))
                        .into(),
                    transform: Transform::from_xyz(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        0.,
                    )
                    .with_scale(Vec3::splat(SQUARE_SIZE)),
                    material: materials.add(ColorMaterial::from(if (y + x) % 2 == 0 {
                        DARK_COLOUR
                    } else {
                        LIGHT_COLOUR
                    })),
                    ..default()
                })
                .insert_bundle(SquareComp {
                    pos: Position { x: x, y: y },
                });

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

            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1.),
                            flip: false,
                        }))
                        .into(),
                    transform: Transform::from_xyz(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        0.,
                    )
                    .with_scale(Vec3::splat(SQUARE_SIZE)),
                    material: materials.add(ColorMaterial::from(piece_texture)),
                    ..default()
                })
                .insert_bundle(PieceComp {
                    pos: Position { x: x, y: y },
                    piece_type: *square,
                });
        }
    }
}

fn drag_and_drop(
    mut cursor_state: Local<CursorState>,
    windows: Res<Windows>,
    mut mouse_input_event: EventReader<MouseButtonInput>,
    mut cursor_moved_event: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    query: Query<(&Position, &Square, Entity)>,
    mut transforms: Query<&mut Transform>,
) {
    // Calculate the cursor state relative to the center of the screen.
    let window = windows.get_primary().unwrap();
    let mouse_offset = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    if let Some(cursor_event) = cursor_moved_event.iter().last() {
            cursor_state.pos = cursor_event.position - mouse_offset;
        };

    // If the mouse was released, this means that the player is no longer dragging a piece,
    // therefore we set the piece to None.
    if mouse_button_input.just_released(MouseButton::Left) {
        let piece = match cursor_state.piece {
            Some(val) => val,
            None => {
                cursor_state.piece = None;
                return
            },
        };
        let mut piece_pos = transforms.get_mut(piece.0).unwrap();
        piece_pos.translation.z = 0.;
        cursor_state.piece = None;
        return;
    }

    // If the mouse is currently pressed and there currently exists a piece in the cursor state,
    // move the piece's position to the cursor.
    if mouse_button_input.pressed(MouseButton::Left) && cursor_state.piece.is_some() {
        let piece = cursor_state.piece.unwrap();
        let mut piece_pos = transforms.get_mut(piece.0).unwrap();

        piece_pos.translation.x = cursor_state.pos.x + piece.1.x;
        piece_pos.translation.y = cursor_state.pos.y + piece.1.y;
        piece_pos.translation.z = 1.;


        for (position, square, entity) in query.iter() {
            println!("{:?} {:?} {:?} {:?}", position, square, cursor_state.pos, entity);
        }
    }

    // If the mouse was just pressed, calculate which piece is on the mouse and set the cursor
    // state's piece to that piece.
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (position, square, entity) in query.iter() {
            let piece_pos = transforms.get_mut(entity).unwrap().translation;
            let piece_size = transforms.get_mut(entity).unwrap().scale;
            let diff = cursor_to_piece_diff(&cursor_state.pos, &piece_pos);
            if diff.length() < (piece_size.x / 2.0) {
                cursor_state.piece = Some((entity, diff));
            }
        }
    }
}

fn cursor_to_piece_diff(cursor_pos: &Vec2, piece_pos: &Vec3) -> Vec3 {
    Vec3::new(
        piece_pos.x - cursor_pos.x,
        piece_pos.y - cursor_pos.y,
        0.0,
    )
}
