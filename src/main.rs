// chess - A Rust implementation of the famous game Chess.
// Copyright (C) 2023  Brynley Llewellyn-Roux and Aryan Jassal
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
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowMode},
};
use chess::{
    board::{Board, PieceColour, PieceKind, Position, Square},
    drag_and_drop::DragAndDropPlugin,
    update_dimensions::UpdateDimensionsPlugin,
};

const LIGHT_COLOUR: Color = Color::rgb(0.93, 0.93, 0.82);
const DARK_COLOUR: Color = Color::rgb(0.46, 0.59, 0.34);
const SQUARE_SIZE: f32 = 64.0;

#[derive(Component)]
struct PlayerText;

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..Default::default()
        }))
        .init_resource::<Board>()
        .add_plugins(DragAndDropPlugin)
        .add_plugins(UpdateDimensionsPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_player_text)
        .add_systems(Update, update_fps_counter)
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
        .spawn(
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
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            }),
        )
        .insert(PlayerText);

    commands
        .spawn(
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
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(15.0),
                ..default()
            }),
        )
        .insert(FpsText);

    board.layout().iter().enumerate().for_each(|(y, rank)| {
        rank.iter().enumerate().for_each(|(x, square)| {
            commands
                .spawn(MaterialMesh2dBundle {
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
                .spawn(MaterialMesh2dBundle {
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
                        0.5,
                    )
                    .with_scale(Vec3::splat(SQUARE_SIZE)),
                    ..default()
                })
                .insert((Position { x, y }, *square));
        })
    });

    commands.spawn(Camera2dBundle::default());
}

fn update_player_text(board: Res<Board>, mut query: Query<&mut Text, With<PlayerText>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{:?}", board.player());
    }
}

fn update_fps_counter(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}
