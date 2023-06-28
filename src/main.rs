use bitvec::prelude::*;
use itertools::Itertools;

pub const BOARD_SIZE: usize = 64;
pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;

mod board;
mod game;
mod piece;
mod player;

fn main() {
    let arr = bitarr!(u32, Lsb0; 1;33);
    let x = arr.iter().map(|x| *x).collect_vec();
    println!("{x:?}");
}
