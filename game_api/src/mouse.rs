use bevy::prelude::*;

use crate::grid::Grid;

pub struct MousePlugin;

#[derive(Component)]
pub struct MouseEllipse;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_cursor);
    }
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    board : Res<Grid>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let (i,j) = board.get_indexes((point.x, point.y));
    let point = board.get_translation(i, j);

    let (i,j) = board.gap_size();

    gizmos.circle_2d(point.into(),i.min(j) / 2., Color::WHITE);
}
