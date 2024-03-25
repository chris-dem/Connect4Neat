use bevy::prelude::*;

mod grid;
mod plugins;
mod mouse;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(grid::GridPlugin)
        .add_plugins(mouse::MousePlugin)
        .run();
}

