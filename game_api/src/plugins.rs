use bevy::prelude::*;

use crate::{game::*, grid::*};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Grid>()
            .init_resource::<BoardResource>()
            .add_systems(Startup, (initialize_grid, setup_camera, setup_board))
            .add_systems(Startup, spawn_background.after(initialize_grid));
    }
}
