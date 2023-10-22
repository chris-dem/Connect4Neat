use crate::game::start::*;
use crate::player_agent::agent::PlayerTrait;
use crate::{board::*, player::Player};

pub struct RoundAPI<T: GameTrait> {
    pub(crate) state: T,
    pub(crate) board: Board,
}

impl<T: GameTrait> RoundAPI<T> {
    pub fn get_board(&self) -> &Board {
        &self.board
    }
}

// struct End {
//     terminated: TerminatedStatus,
// }

pub trait GameTrait {}
// impl<'a, 'b> GameTrait for Play<'a, 'b> {}
// impl GameTrait for End {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_set_players() {}
}
