use crate::board::Board;

use super::{
    game::{GameTrait, RoundAPI},
    play::Play,
    player_interaction::PlayerHandle,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Start;

impl GameTrait for Start {}

impl Default for RoundAPI<Start> {
    fn default() -> Self {
        Self {
            board: Board::default(),
            state: Start::default(),
        }
    }
}

pub type RoundStart = RoundAPI<Start>;

impl RoundStart {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_game(self) -> RoundAPI<Play> {
        RoundAPI {
            board: self.board,
            state: Play::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_state() {
        let game_start = RoundStart::new();
        assert_eq!(game_start.state, Start::default());
        assert_eq!(game_start.board, Board::default());
    }
}
