use crate::board::*;

pub(crate) struct RoundAPI<T: GameTrait> {
    pub(crate) state: T,
    pub(crate) board: Board,
}

impl<T: GameTrait> RoundAPI<T> {
    pub fn get_board(&self) -> &Board {
        &self.board
    }
}

pub trait GameTrait {}

