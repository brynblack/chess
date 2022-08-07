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
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
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

#[derive(Component)]
struct PlayerText;

#[derive(Component)]
struct FpsText;

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
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(update_dimensions)
        .add_system(drag_and_drop)
        .add_system(update_player_text)
        .add_system(update_fps_counter)
        .run();
}

fn setup(
    mut commands: Commands,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Current player: ",
                    TextStyle {
                        font: asset_server.load("fonts/text.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/text.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(PlayerText);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/text.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/text.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(20.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(FpsText);

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
    board: Res<Board>,
    mut query: Query<(&mut Transform, &Position)>,
    mut resize_events: EventReader<WindowResized>,
) {
    resize_events.iter().for_each(|event| {
        let new_size = (event.width / board.layout().len() as f32)
            .min(event.height / board.layout().len() as f32);

        query.iter_mut().for_each(|(mut transform, position)| {
            transform.scale = Vec3::splat(new_size);

            transform.translation = Vec3::new(
                position.x as f32 * new_size - event.width / 2.0 + (new_size / 2.0),
                position.y as f32 * new_size - event.height / 2.0 + (new_size / 2.0),
                transform.translation.z,
            )
        });
    });
}

fn update_player_text(board: Res<Board>, mut query: Query<&mut Text, With<PlayerText>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{:?}", board.player());
    }
}

fn update_fps_counter(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}

fn drag_and_drop(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut cursor_state: Local<CursorState>,
    mouse_inputs: Res<Input<MouseButton>>,
    mut moved_events: EventReader<CursorMoved>,
    mut positions: Query<&mut Position>,
    mut query: Query<(Entity, &mut Transform, Option<&Square>)>,
    windows: Res<Windows>,
) {
    if let Some(cursor_event) = moved_events.iter().last() {
        let window = windows.get_primary().unwrap();
        let window_centre = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        cursor_state.position = cursor_event.position - window_centre;
    };

    if cursor_state.piece.is_some() {
        if mouse_inputs.just_released(MouseButton::Left) {
            let mut closest_square: Option<Entity> = None;
            query.iter().for_each(|(entity, transform, square)| {
                if square.is_none() {
                    let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
                    if diff.length() < (transform.scale.x / 2.0) {
                        closest_square = Some(entity);
                    }
                }
            });

            if closest_square.is_none() {
                return;
            }

            let mut closest_piece: Option<Entity> = None;
            query.iter().for_each(|(entity, transform, square)| {
                if square.is_some() {
                    let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
                    if diff.length() < (transform.scale.x / 2.0) {
                        if entity != cursor_state.piece.unwrap().0 {
                            closest_piece = Some(entity);
                        }
                    }
                }
            });

            if closest_square.is_none() {
                return;
            }

            let piece = cursor_state.piece.unwrap();

            let piece_coord = positions.get(piece.0).unwrap();

            let closest_square_coord = positions.get(closest_square.unwrap()).unwrap();
            let boilerplate = closest_square_coord.clone();

            let piece_size = query.get(piece.0).unwrap().1.scale;
            let mut piece_pos = query.get_mut(piece.0).unwrap().1;

            println!("{:?}", piece_coord);
            println!("{:?}", closest_square_coord);

            match board.move_piece(Move {
                old_pos: *piece_coord,
                new_pos: *closest_square_coord,
            }) {
                Ok(_) => {
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
                    if closest_piece.is_some() {
                        commands.entity(closest_piece.unwrap()).despawn();
                    }
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

        if mouse_inputs.pressed(MouseButton::Left) {
            let piece = cursor_state.piece.unwrap();
            let mut transform = query.get_mut(piece.0).unwrap().1;

            transform.translation.x = cursor_state.position.x + piece.1.x;
            transform.translation.y = cursor_state.position.y + piece.1.y;
            return;
        }
    }

    if mouse_inputs.just_pressed(MouseButton::Left) {
        query.iter().for_each(|(entity, transform, _)| {
            let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
            if diff.length() < (transform.scale.x / 2.0) {
                cursor_state.piece = Some((entity, diff));
            }
        });
    }
}

fn cursor_to_piece_diff(cursor_pos: &Vec2, piece_pos: &Vec3) -> Vec3 {
    Vec3::new(piece_pos.x - cursor_pos.x, piece_pos.y - cursor_pos.y, 0.0)
}
