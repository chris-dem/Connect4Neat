use colored::Colorize;
use rand_derive2::RandGen;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RandGen)]
pub enum Player {
    Red,
    Yellow,
}

impl FromStr for Player {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(Player::Red),
            "y" => Ok(Player::Yellow),
            _ => Err(()),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::Red => "*".red(),
                Player::Yellow => "*".cyan(),
            }
        )
    }
}

#[macro_export]
macro_rules! player {
    ($name : ident) => {{
        let st: Player = std::str::FromStr::from_str(stringify!($name)).unwrap();
        st
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macros() {
        assert_eq!(player!(r), Player::Red);
        assert_eq!(player!(y), Player::Yellow);
    }
}
