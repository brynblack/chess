//! A module for the update dimensions plugin.

use crate::board::*;
use bevy::{prelude::*, window::WindowResized};

/// A plugin implementing rescaling of pieces on the chessboard.
pub struct UpdateDimensionsPlugin;

impl Plugin for UpdateDimensionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_dimensions);
    }
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
