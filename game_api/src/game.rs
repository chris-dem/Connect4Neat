use bevy::prelude::*;
use round_api::board::Board;

pub struct GamePlugin;

#[derive(Resource, Default)]
pub struct BoardResource {
    board: Board,
}

pub fn setup_board(mut commands: Commands) {
    commands.insert_resource(BoardResource::default());
}
