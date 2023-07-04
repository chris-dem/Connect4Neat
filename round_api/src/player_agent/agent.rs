use mockall::*;

use crate::board::{Board, Col};

#[automock]
pub trait PlayerTrait {
    fn play(&mut self, board: &Board) -> Col;
}
