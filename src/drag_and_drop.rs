//! A module for the drag and drop plugin.

use crate::board::*;
use bevy::prelude::*;

/// A plugin implementing a drag and drop system.
pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(drag_and_drop);
    }
}

#[derive(Default, Component)]
struct CursorState {
    position: Vec2,
    piece: Option<(Entity, Vec3)>,
}

fn drag_and_drop(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut cursor_state: Local<CursorState>,
    mouse_inputs: Res<Input<MouseButton>>,
    mut moved_events: EventReader<CursorMoved>,
    mut query: Query<(Entity, &mut Transform, &mut Position, Option<&Square>)>,
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
            let mut closest_piece: Option<Entity> = None;

            query.iter().for_each(|(entity, transform, _, piece)| {
                if piece.is_none() {
                    let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
                    if diff.length() < (transform.scale.x / 2.0) {
                        closest_square = Some(entity);
                    }
                }
                if piece.is_some() {
                    let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
                    if diff.length() < (transform.scale.x / 2.0)
                        && entity != cursor_state.piece.unwrap().0
                    {
                        closest_piece = Some(entity);
                    }
                }
            });
            let piece = cursor_state.piece.unwrap();
            let piece_size = query.get(piece.0).unwrap().1.scale;

            let closest_square = match closest_square {
                Some(square) => square,
                None => {
                    let (_, mut piece_pos, piece_coord, _) = query.get_mut(piece.0).unwrap();
                    let window = windows.get_primary().unwrap();
                    piece_pos.translation.x = piece_coord.x as f32 * piece_size.x
                        - window.width() / 2.0
                        + (piece_size.x / 2.0);
                    piece_pos.translation.y = piece_coord.y as f32 * piece_size.y
                        - window.height() / 2.0
                        + (piece_size.y / 2.0);
                    cursor_state.piece = None;
                    return;
                }
            };

            let piece_coord = query.get(piece.0).unwrap().2;
            let closest_square_coord = query.get(closest_square).unwrap().2;
            let boilerplate = *closest_square_coord;

            match board.move_piece(Move {
                old_pos: *piece_coord,
                new_pos: *closest_square_coord,
            }) {
                Ok(_) => {
                    let window = windows.get_primary().unwrap();
                    let (_, mut piece_pos, mut piece_coord, _) = query.get_mut(piece.0).unwrap();
                    piece_coord.x = boilerplate.x;
                    piece_coord.y = boilerplate.y;
                    piece_pos.translation.x = boilerplate.x as f32 * piece_size.x
                        - window.width() / 2.0
                        + (piece_size.x / 2.0);
                    piece_pos.translation.y = boilerplate.y as f32 * piece_size.y
                        - window.height() / 2.0
                        + (piece_size.y / 2.0);
                    if let Some(closest_piece) = closest_piece {
                        commands.entity(closest_piece).despawn();
                    }
                }
                Err(err) => {
                    let (_, mut piece_pos, piece_coord, _) = query.get_mut(piece.0).unwrap();
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
        query.iter().for_each(|(entity, transform, _, piece)| {
            if piece.is_some() {
                let diff = cursor_to_piece_diff(&cursor_state.position, &transform.translation);
                if diff.length() < (transform.scale.x / 2.0) {
                    cursor_state.piece = Some((entity, diff));
                }
            }
        });
    }
}

fn cursor_to_piece_diff(cursor_pos: &Vec2, piece_pos: &Vec3) -> Vec3 {
    Vec3::new(piece_pos.x - cursor_pos.x, piece_pos.y - cursor_pos.y, 0.0)
}
