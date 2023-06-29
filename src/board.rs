use bitvec::prelude::*;
use itertools::{EitherOrBoth, Itertools};
use std::{fmt::Display, io::Read};

use rand::prelude::*;

use crate::{piece::Piece, player::Player, BOARD_SIZE, HEIGHT, WIDTH};

const COL_BASE: u64 = 255;
const ROW_BASE: u64 = 72_340_172_838_076_673;
const MAIN_DIAG_BASE: u64 = 9_241_421_688_590_303_745;
const SEC_DIAG_BASE: u64 = 72_624_976_668_147_840;

/// Illegal move possibilities
/// Either move is out of bounds or the current column is full
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalMove {
    OutOfBounds,
    StackIsFull,
}

/// Describes possible outcomes for each play
/// Either there is an illegal move, a valid play or the game has terminated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlay {
    ValidPlay,
    InvalidBoard(IllegalMove),
    GameTerminated(TerminatedStatus),
}

/// Game terminated status
/// Either a player had won or a draw occured
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminatedStatus {
    Win(Player),
    Draw,
}

#[inline]
fn to_position(i: u8, j: u8) -> usize {
    i as usize * WIDTH + j as usize
}

#[inline]
fn get_col(indx: u8) -> u64 {
    assert!(indx < 8);
    COL_BASE << indx as u64 * 8
}

#[inline]
fn get_row(indx: u8) -> u64 {
    assert!(indx < 8);
    ROW_BASE << indx as u64
}

// #[inline]
// fn extract_mask(indx : u64) ->

#[inline]
fn get_main_diag(indx: i8) -> u64 {
    assert!(indx.abs() < 8);
    if indx > 0 {
        let shifted_inverse = (ROW_BASE * ((1 << indx as u64) - 1)) << (8 - indx as u8);
        (MAIN_DIAG_BASE >> indx) & !shifted_inverse
    } else if indx < 0 {
        let indx = -indx as u64;
        let shifted = ROW_BASE * ((1 << indx as u64) - 1 as u64);
        MAIN_DIAG_BASE << indx as u64 & !shifted
    } else {
        MAIN_DIAG_BASE
    }
}

#[inline]
fn get_sec_diag(indx: i8) -> u64 {
    assert!(indx.abs() < 8);
    if indx > 0 {
        let shifted_inverse = ROW_BASE * ((1 << indx as u64) - 1);
        (SEC_DIAG_BASE << indx) & !shifted_inverse
    } else if indx < 0 {
        let indx = -indx as u64;
        let shifted = ROW_BASE * ((1 << indx as u64) - 1 as u64) << (8 - indx as u8);
        SEC_DIAG_BASE >> indx as u64 & !shifted
    } else {
        SEC_DIAG_BASE
    }
}

type BitBoard = BitArr!(for 64, in u8, Lsb0);

#[derive(Debug, Clone, Default)]
pub struct Board {
    red: BitBoard,
    yellow: BitBoard,
}

struct DiagStruct {
    main_diag: (u8, u8),
    sec_diag: (u8, u8),
}

fn print_array(arr: &[Piece]) -> String {
    let mut out_buffer = String::new();
    for j in (0..HEIGHT as u8).rev() {
        let mut buf = Vec::with_capacity(WIDTH);
        for i in 0..WIDTH as u8 {
            buf.push(format!("{}", arr[to_position(i, j) as usize]));
        }
        out_buffer.push_str(format!("|{}|\n", buf.join("|")).as_str());
    }
    out_buffer
}

impl Board {
    fn get_array(&self) -> [Piece; BOARD_SIZE] {
        let mut buffer = [Piece(None); BOARD_SIZE];
        for (ind, x) in self
            .red
            .iter()
            .zip(self.yellow.iter())
            .map(|(a, b)| (*a, *b))
            .enumerate()
        {
            buffer[ind] = match x {
                (false, false) => Piece(None),
                (true, false) => Piece(Some(Player::Red)),
                (false, true) => Piece(Some(Player::Yellow)),
                (true, true) => unreachable!("No valid state should result in this scenario"),
            }
        }
        buffer
    }

    pub(crate) fn from_rng(rng: &mut impl RngCore) -> Self {
        let mask: u64 = rng.gen();
        let mut numb: BitBoard = BitArray::ZERO;
        for i in 0..8 {
            numb[i * 8..(i * 8 + rng.gen_range(0..=8usize))].fill(true);
        }
        let a = (numb & mask.view_bits::<Lsb0>());
        let b = numb ^ a;
        Self { red: a, yellow: b }
    }

    pub fn play(&mut self, player: Player, col: u8) -> Result<u8, IllegalMove> {
        if col > 7 {
            return Err(IllegalMove::OutOfBounds);
        }

        let row_red = self.red[col as usize * 8..(col * 8 + 8) as usize].last_one();
        let row_yellow = self.yellow[col as usize * 8..(col * 8 + 8) as usize].last_one();

        let indx = match (row_red, row_yellow) {
            (None, None) => 0,
            (None, Some(e)) => e + 1,
            (Some(e), None) => e + 1,
            (Some(a), Some(b)) => a.max(b) + 1,
        };

        if indx > 7 {
            return Err(IllegalMove::StackIsFull);
        }

        let arr = match player {
            Player::Red => &mut self.red,
            Player::Yellow => &mut self.yellow,
        };

        arr.set(col as usize * 8 + indx, true);
        Ok(indx as u8)
    }

    pub fn check_win(&self, row: u8, col: u8, player: Player) -> bool {
        let arr = match player {
            Player::Red => &self.red,
            Player::Yellow => &self.yellow,
        };
        let arr_mask = [
            get_row(row),
            get_col(col),
            get_sec_diag((row + col) as i8 - 7),
            get_main_diag(row as i8 - col as i8),
        ];
        arr_mask.into_iter().any(|mask| {
            let mask = mask.view_bits::<Lsb0>();
            let arr_masked = *arr & mask;
            mask.iter_ones()
                .enumerate()
                .merge_join_by(arr_masked.iter_ones(), |(_indx, el1), el2| el1.cmp(el2))
                .filter_map(|res| {
                    println!("{res:?}");
                    if let EitherOrBoth::Both((indx, _), _) = res {
                        Some(indx)
                    } else {
                        None
                    }
                })
                .collect_vec()
                .windows(4)
                .any(|el| {
                    el.iter()
                        .copied()
                        .zip(el[1..].iter().copied())
                        .all(|(a, b)| a + 1 == b)
                })
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buff = self.get_array();
        write!(f, "{}", print_array(&buff))
    }
}

#[cfg(test)]
mod board_tests {
    use crate::{
        board::{print_array, to_position, Board},
        piece::Piece,
        player::Player,
    };
    use rand::prelude::*;

    #[test]
    fn create_empty_board_array() {
        let board = Board::default();
        assert!(board.get_array().iter().all(|el| el.0.is_none()))
    }

    mod display_board {
        use super::*;
        use rand::{prelude::*, SeedableRng};

        #[test]
        fn test_display_random() {
            let mut rng = StdRng::seed_from_u64(53);
            let board = Board::from_rng(&mut rng);
            println!("{}", board);
        }

        #[test]
        fn check_from_string() {
            let board = Board::default();
            println!("{}", board)
        }
    }

    mod indexing_functions {
        use crate::board::{
            get_col, get_main_diag, get_row, get_sec_diag, COL_BASE, MAIN_DIAG_BASE, ROW_BASE,
            SEC_DIAG_BASE,
        };

        #[test]
        fn column_testing() {
            // First column
            assert_eq!(COL_BASE, get_col(0));
            // Second column
            assert_eq!(65_280, get_col(1));
            // Third column
            assert_eq!(16_711_680, get_col(2));
            // Fourth column
            assert_eq!(4_278_190_080, get_col(3));
            // Fifth column
            assert_eq!(1_095_216_660_480, get_col(4));
            // Sixth column
            assert_eq!(280_375_465_082_880, get_col(5));
            // Seventh column
            assert_eq!(71_776_119_061_217_280, get_col(6));
            // 8th column
            assert_eq!(18_374_686_479_671_623_680, get_col(7));
        }

        #[test]
        fn row_testing() {
            // First row
            assert_eq!(ROW_BASE, get_row(0));
            // Second row
            assert_eq!(144_680_345_676_153_346, get_row(1));
            // Third row
            assert_eq!(289_360_691_352_306_692, get_row(2));
            // Fourth row
            assert_eq!(578_721_382_704_613_384, get_row(3));
            // Fifth row
            assert_eq!(1_157_442_765_409_226_768, get_row(4));
            // Sixth row
            assert_eq!(2_314_885_530_818_453_536, get_row(5));
            // Seventh row
            assert_eq!(4_629_771_061_636_907_072, get_row(6));
            // Eighth row
            assert_eq!(9_259_542_123_273_814_144, get_row(7));
        }

        #[test]
        fn main_diag_testing() {
            // Firso row
            assert_eq!(MAIN_DIAG_BASE, get_main_diag(0));
            // Second row
            assert_eq!(36_099_303_471_055_874, get_main_diag(-1));
            // Third row
            assert_eq!(141_012_904_183_812, get_main_diag(-2));
            // Fourth row
            assert_eq!(550_831_656_968, get_main_diag(-3));
            // Fifth row
            assert_eq!(2_151_686_160, get_main_diag(-4));
            // Sixth row
            assert_eq!(8_405_024, get_main_diag(-5));
            // Seventh row
            assert_eq!(32_832, get_main_diag(-6));
            // Eighth row
            assert_eq!(128, get_main_diag(-7));

            // Other halve of diag

            // First row
            assert_eq!(4_620_710_844_295_151_872, get_main_diag(1));
            // Third row
            assert_eq!(2_310_355_422_147_575_808, get_main_diag(2));
            // Fourth row
            assert_eq!(1_155_177_711_073_755_136, get_main_diag(3));
            // Fifth row
            assert_eq!(577_588_855_528_488_960, get_main_diag(4));
            // Sixth row
            assert_eq!(288_794_425_616_760_832, get_main_diag(5));
            // Seventh row
            assert_eq!(144_396_663_052_566_528, get_main_diag(6));
            // Eighth row
            assert_eq!(72_057_594_037_927_936, get_main_diag(7));
        }

        #[test]
        fn sec_diag_testing() {
            // Firso row
            assert_eq!(SEC_DIAG_BASE, get_sec_diag(0));
            // Second row
            assert_eq!(283_691_315_109_952, get_sec_diag(-1));
            // Third row
            assert_eq!(1_108_169_199_648, get_sec_diag(-2));
            // Fourth row
            assert_eq!(4_328_785_936, get_sec_diag(-3));
            // Fifth row
            assert_eq!(16_909_320, get_sec_diag(-4));
            // Sixth row
            assert_eq!(66_052, get_sec_diag(-5));
            // Seventh row
            assert_eq!(258, get_sec_diag(-6));
            // Eighth row
            assert_eq!(1, get_sec_diag(-7));

            // Other halve of diag

            // Second row
            assert_eq!(145_249_953_336_295_424, get_sec_diag(1));
            // Third row
            assert_eq!(290_499_906_672_525_312, get_sec_diag(2));
            // Fourth row
            assert_eq!(580_999_813_328_273_408, get_sec_diag(3));
            // Fifth row
            assert_eq!(1_161_999_622_361_579_520, get_sec_diag(4));
            // Sixth row
            assert_eq!(2_323_998_145_211_531_264, get_sec_diag(5));
            // Seventh row
            assert_eq!(4_647_714_815_446_351_872, get_sec_diag(6));
            // Eighth row
            assert_eq!(9_223_372_036_854_775_808, get_sec_diag(7));
        }
    }

    mod play_testing {
        use crate::{board::IllegalMove, player::Player};

        use super::*;
        use proptest::prelude::*;

        #[test]
        fn play_empty_board() {
            let mut board = Board::default();
            let mut rng = thread_rng();

            assert_eq!(
                board
                    .play(Player::Red, rng.gen_range(0..=7))
                    .expect("Should work for empty boards"),
                0
            );
        }

        proptest! {
            #[test]
            fn play_on_empty_board(a in any::<u8>()) {
                let mut board = Board::default();
                let res = board.play(Player::Red, a);
                if a < 8 {
                    prop_assert_eq!(res.unwrap(), 0)
                } else {
                    prop_assert_eq!(res.unwrap_err(),IllegalMove::OutOfBounds)
                }
            }
        }

        mod against_custom_boards {
            use super::*;

            mod seed_31 {
                use super::*;
                #[test]
                #[ignore = "Just for printing"]
                fn print_board() {
                    let mut rng = StdRng::seed_from_u64(31);
                    let board = Board::from_rng(&mut rng);

                    println!("31");
                    println!("{board}")
                }

                #[test]
                fn test_col_0() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(31));
                    let mut original_array = board.get_array();
                    original_array[to_position(0, 5)] = Piece(Some(Player::Red));
                    let row = board.play(Player::Red, 0).unwrap();
                    assert_eq!(5, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_1() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(31));
                    let mut original_array = board.get_array();
                    original_array[to_position(1, 1)] = Piece(Some(Player::Red));
                    let row = board.play(Player::Red, 1).unwrap();
                    assert_eq!(1, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_4() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(31));
                    let mut original_array = board.get_array();
                    original_array[to_position(4, 0)] = Piece(Some(Player::Red));
                    let row = board.play(Player::Red, 4).unwrap();
                    assert_eq!(0, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_5() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(31));
                    let mut original_array = board.get_array();
                    original_array[to_position(5, 7)] = Piece(Some(Player::Red));
                    let row = board.play(Player::Red, 5).unwrap();
                    assert_eq!(7, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }
            }

            mod seed_32 {
                use super::*;
                #[test]
                #[ignore = "Just for printing"]
                fn print_board() {
                    let mut rng = StdRng::seed_from_u64(32);
                    let board = Board::from_rng(&mut rng);

                    println!("32");
                    println!("{board}");
                }

                #[test]
                fn test_col_0() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(32));
                    let mut original_array = board.get_array();
                    original_array[to_position(0, 0)] = Piece(Some(Player::Yellow));
                    let row = board.play(Player::Yellow, 0).unwrap();
                    assert_eq!(0, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_2() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(32));
                    let mut original_array = board.get_array();
                    original_array[to_position(2, 5)] = Piece(Some(Player::Yellow));
                    let row = board.play(Player::Yellow, 2).unwrap();
                    assert_eq!(5, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_3() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(32));
                    let mut original_array = board.get_array();
                    original_array[to_position(3, 0)] = Piece(Some(Player::Yellow));
                    let row = board.play(Player::Yellow, 3).unwrap();
                    assert_eq!(0, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_5() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(32));
                    let mut original_array = board.get_array();
                    original_array[to_position(5, 6)] = Piece(Some(Player::Yellow));
                    let row = board.play(Player::Yellow, 5).unwrap();
                    assert_eq!(6, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }
            }

            mod seed_231 {

                use crate::board::IllegalMove;

                use super::*;
                #[test]
                #[ignore = "Just for printing"]
                fn print_board() {
                    let mut rng = StdRng::seed_from_u64(231);
                    let board = Board::from_rng(&mut rng);

                    println!("231");
                    println!("{board}")
                }

                #[test]
                fn test_col_0() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(231));
                    let mut original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    original_array[to_position(0, 1)] = Piece(Some(player));
                    let row = board.play(player, 0).unwrap();
                    assert_eq!(1, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_2() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(231));
                    let mut original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    original_array[to_position(2, 1)] = Piece(Some(player));
                    let row = board.play(player, 2).unwrap();
                    assert_eq!(1, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_3() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(231));
                    let arr = board.get_array();
                    let player: Player = thread_rng().gen();
                    let err = board.play(player, 3).unwrap_err();
                    assert_eq!(IllegalMove::StackIsFull, err);
                    // Should not affect stack
                    assert!(arr
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(a, b)| a == b));
                }

                #[test]
                fn test_col_6() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(231));
                    let mut original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    original_array[to_position(6, 0)] = Piece(Some(player));
                    let row = board.play(player, 6).unwrap();
                    assert_eq!(0, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }
            }

            mod seed_122 {

                use crate::{board::IllegalMove, player};

                use super::*;
                #[test]
                #[ignore = "Just for printing"]
                fn print_board() {
                    let mut rng = StdRng::seed_from_u64(122);
                    let board = Board::from_rng(&mut rng);

                    println!("122");
                    println!("{board}")
                }

                #[test]
                fn test_col_0() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(122));
                    let original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    let row = board.play(player, 0).unwrap_err();
                    assert_eq!(IllegalMove::StackIsFull, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_1() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(122));
                    let original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    let row = board.play(player, 1).unwrap_err();
                    assert_eq!(IllegalMove::StackIsFull, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_5() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(122));
                    let original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    let row = board.play(player, 5).unwrap_err();
                    assert_eq!(IllegalMove::StackIsFull, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }

                #[test]
                fn test_col_7() {
                    let mut board = Board::from_rng(&mut StdRng::seed_from_u64(122));
                    let mut original_array = board.get_array();
                    let player: Player = thread_rng().gen();
                    original_array[to_position(7, 0)] = Piece(Some(player));
                    let row = board.play(player, 7).unwrap();
                    assert_eq!(0, row);
                    assert!(original_array
                        .iter()
                        .zip(board.get_array().iter())
                        .all(|(el_org, el_after)| el_org == el_after));
                }
            }
        }
    }

    mod validate_board {
        use super::*;

        mod test_vertical {
            use super::*;

            #[test]
            fn test_vertical_seed_122() {
                let board = Board::from_rng(&mut StdRng::seed_from_u64(122));

                for i in 3..=6 {
                    assert!(board.check_win(i, 5, Player::Yellow));
                    assert!(!board.check_win(i, 5, Player::Red));
                }
            }
        }

        mod test_diag {
            use super::*;

            #[test]
            fn test_diag_seed_124() {
                let board = Board::from_rng(&mut StdRng::seed_from_u64(124));
                for (j, i) in [(6, 0), (4, 2), (3, 3)] {
                    assert!(board.check_win(i, j, Player::Yellow), "for row {i} col {j}");
                    assert!(!board.check_win(i, j, Player::Red));
                }
            }
        }

        mod test_horizontal {
            use super::*;

            #[test]
            fn test_horizontal_seed_123() {
                let board = Board::from_rng(&mut StdRng::seed_from_u64(123));
                for i in 4..=7 {
                    assert!(board.check_win(0, i, Player::Yellow));
                    assert!(!board.check_win(0, i, Player::Red));
                }
            }
        }
    }

    #[test]
    #[ignore = "Testing the interaction between bit representation and array representation"]
    fn test_array() {
        for i in 20..50 {
            let mut rng = StdRng::seed_from_u64(i);
            let board = Board::from_rng(&mut rng);
            println!("State:{i}\n{board}");
        }

        // let mut arr = board.get_array();

        // arr[to_position(4, 2)] = Piece(Some(Player::Red));
        // println!("{}", print_array(&arr));
    }
}
