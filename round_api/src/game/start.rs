use crate::board::Board;

use super::game::{GameTrait, RoundAPI};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Start(pub(crate) Option<GameMode>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    PVP,
    PVE,
    Training,
}

impl GameTrait for Start {}

impl Default for RoundAPI<Start> {
    fn default() -> Self {
        Self {
            board: Board::default(),
            state: Start(None),
        }
    }
}

pub type RoundStart = RoundAPI<Start>;

impl RoundStart {
    pub fn new() -> Self {
        Self::default()
    }
}
