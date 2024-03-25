use bevy::prelude::*;

mod game;
mod mouse;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::BoardPlugin)
        .add_plugins(mouse::MousePlugin)
        .run();
}

