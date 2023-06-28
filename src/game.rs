use crate::{board::*, player::Player};
pub struct GameState {
    board: Board,
    current_player: Player,
    terminated: bool,
}

impl GameState {
    pub fn new(board: Board) -> Self {
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
    // pub fn play(&mut self, column: u8) -> GamePlay {
    //     todo!()
    // }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: Board::default(),
            current_player: Player::Red,
            terminated: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::RngCore;

    use super::*;
    use crate::*;
    use itertools::*;
    use rand::prelude::*;

    mod condition_tests {
        use super::*;

        mod valid_moves {
            use super::*;

            // #[test]
            // fn test_empty_board_on_all_columns() {
            //     for col in 0..7 {
            //         let mut game_state = GameState::default();
            //         assert_eq!(game_state.play(col), GamePlay::ValidPlay);
            //     }
            // }

            #[test]
            fn test_on_random_positions() {}
        }
    }
}
