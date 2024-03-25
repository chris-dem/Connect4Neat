use std::fmt::Display;

use crate::player::Player;

#[macro_export]
macro_rules! piece {
    () => {
        Piece(None)
    };
    
    ($r:ident) => {
        Piece(Some($crate::player!($r)))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece(pub Option<Player>);

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

#[cfg(test)]
mod test_macros {
    use super::*;

    #[test]
    fn check_piece() {
        assert_eq!(piece!(), Piece(None));
        assert_eq!(piece!(r), Piece(Some(Player::Red)));
        assert_eq!(piece!(y), Piece(Some(Player::Yellow)));
    }
}
