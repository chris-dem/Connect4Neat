use bevy::prelude::*;

mod grid;
mod mouse;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(grid::GridPlugin)
        .add_plugins(mouse::MousePlugin)
        .run();
}

