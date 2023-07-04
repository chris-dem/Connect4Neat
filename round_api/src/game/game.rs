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

struct PlayerHandle<'a, 'b> {
    pub player_red: &'a mut dyn PlayerTrait,
    pub player_yellow: &'b mut dyn PlayerTrait,
}

struct Play<'a, 'b> {
    player_handler: PlayerHandle<'a, 'b>,
    currentPlayer: Player,
}
struct End {
    terminated: TerminatedStatus,
}

pub trait GameTrait {}
impl<'a, 'b> GameTrait for Play<'a, 'b> {}
impl GameTrait for End {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_state() {
        let game_start = RoundStart::new();

        assert_eq!(game_start.state, Start(None));
        assert_eq!(game_start.board, Board::default());
    }

    #[test]
    fn test_game_set_players() {}
}
