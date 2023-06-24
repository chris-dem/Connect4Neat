use std::fmt::Display;

use itertools::Itertools;

use crate::{player::Player, BOARD_SIZE, HEIGHT, WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece(pub Option<Player>);

#[inline]
pub fn get_position(indx: u8) -> (u8, u8) {
    (indx / HEIGHT as u8, indx % HEIGHT as u8)
}

#[inline]
pub fn to_position(i: u8, j: u8) -> u8 {
    i + j * HEIGHT as u8
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                None => " ".to_owned(),
                Some(e) => format!("{}", e),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Board(pub [Piece; BOARD_SIZE]);

impl Board {
    pub fn new() -> Self {
        Self([Piece(None); BOARD_SIZE])
    }

    // pub fn ref_index(&self, indx : usize) ->
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_buffer = String::new();
        for j in 0..HEIGHT as u8 {
            let mut buf = Vec::with_capacity(WIDTH);
            for i in (0..WIDTH as u8).rev() {
                buf.push(format!("{}", self.0[to_position(i, j) as usize]));
            }
            out_buffer.push_str(format!("|{}|\n", buf.join("|")).as_str());
        }
        write!(f, "{out_buffer}")
    }
}

pub enum BoardError {
    IndexError,
    FillError,
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, player::Player, *};

    use super::{to_position, Piece};
    #[test]
    fn create_empty_board() {
        let get_board = Board::new();
        assert!(get_board.0.iter().all(|e| e.0.is_none()))
    }

    mod display_board {
        use super::*;

        #[test]
        fn test_display() {
            let board = Board::new();
            println!("{}", board);
        }
    }

    #[test]
    fn check_from_string() {
        let mut board = [Piece(None); BOARD_SIZE];
        board[to_position(0, 3) as usize] = Piece(Some(Player::Red));
        board[to_position(1, 3) as usize] = Piece(Some(Player::Yellow));
        board[to_position(2, 3) as usize] = Piece(Some(Player::Yellow));
        board[to_position(0, 4) as usize] = Piece(Some(Player::Red));
        let board = Board(board);
        println!("{}", board)
    }
}
