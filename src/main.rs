use bitvec::prelude::*;
use itertools::{EitherOrBoth, Itertools};

pub const BOARD_SIZE: usize = 64;
pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;

mod board;
mod game;
mod piece;
mod player;

fn main() {
    let arr = (0..7).collect_vec();
    let arr2 = [1, 2, 3, 4, 5];
    let res = arr
        .iter()
        .copied()
        .merge_join_by(arr2.iter().copied(), usize::cmp)
        .filter_map(|sort_res| match sort_res {
            EitherOrBoth::Both(a, _) => Some(a),
            _ => None,
        })
        .collect_vec();
    println!("{res:?}");
}
