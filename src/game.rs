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

    // pub fn play(&mut self, column: u8) -> Result<()> {}
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Red,
            terminated: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn play() {}
}
