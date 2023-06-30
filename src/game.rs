use std::default;

use crate::{board::*, player::Player};
pub struct GameState<T: GameTrait> {
    board: Board,
    current_player: Player,
    state: T,
    terminated: Option<TerminatedStatus>,
}

#[derive(Default, Debug)]
pub enum GameMode {
    #[default]
    NotSet,
    PVP,
    PVE,
}

struct Start(GameMode);
struct Play;
struct End;

pub trait GameTrait {}
impl GameTrait for Start {}
impl GameTrait for Play {}
impl GameTrait for End {}

impl GameState<Start> {
    fn new(board: Board) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    // Insert current chip at specific column and flip player
    // Player will only be flo
    pub fn play(&mut self, column: u8) -> GamePlay {
        todo!()
    }
}

impl Default for GameState<Start> {
    fn default() -> Self {
        Self {
            board: Board::default(),
            current_player: Player::Red,
            state: Start(GameMode::NotSet),
            terminated: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;
    use rand::prelude::*;

    mod condition_tests {
        use super::*;

        // proptest! {
        //     #[test]
        //     fn test_on_empty_board(a in any::<u8>()) {
        //         let mut game_state = GameState::default();
        //         if a < 8 {
        //             prop_assert_eq!(game_state.play(a), GamePlay::ValidPlay);
        //             prop_assert_eq!(game_state.current_player, Player::Yellow);
        //             prop_assert!(game_state.terminated.is_none());
        //         } else {
        //             prop_assert_eq!(game_state.play(a), GamePlay::InvalidBoard(IllegalMove::OutOfBounds));
        //             prop_assert_eq!(game_state.current_player, Player::Red);
        //             prop_assert!(game_state.terminated.is_none());
        //         }

        //     }
        // }

        // mod on_custom_boards_seed_42 {
        //     use crate::game;

        //     use super::*;
        //     #[test]
        //     fn test_on_col_1_player_red_should_be_valid() {
        //         let mut game_state =
        //             GameState::new(Board::from_rng(&mut StdRng::seed_from_u64(41)));

        //         let game_result = game_state.play(1);
        //         assert_eq!(game_result, GamePlay::ValidPlay);
        //         assert_eq!(game_state.current_player, Player::Yellow);
        //         assert!(game_state.terminated.is_none());
        //     }

        //     #[test]
        //     fn test_on_col_6_player_red_should_be_win() {
        //         let mut game_state =
        //             GameState::new(Board::from_rng(&mut StdRng::seed_from_u64(41)));

        //         let game_result = game_state.play(1);
        //         assert_eq!(
        //             game_result,
        //             GamePlay::GameTerminated(TerminatedStatus::Win(Player::Red))
        //         );
        //         assert_eq!(game_state.current_player, Player::Yellow);
        //         assert_eq!(game_state.terminated);
        //     }
        // }
    }
}
