use super::{
    board::{self, Board},
    player::Player,
};

#[cfg(feature = "print_game")]
use std::{thread, time::Duration};

pub type WallData = (bool, usize, usize);
pub type MoveData = (usize, usize);

#[derive(Clone, Copy)]
pub enum Turn {
    Move(MoveData),
    Wall(WallData),
}

pub trait AI {
    fn play(&mut self, board: &Board, player: &Player, enemy: &Player) -> Turn;
}

pub struct GameResult {
    pub turns: usize,
    pub winner: bool,
}

pub fn run<F, V>(mut player_one_ai: F, mut player_two_ai: V) -> GameResult
where
    F: AI,
    V: AI,
{
    let mut board = board::create_new_board();
    let mut turns = 0;

    let (mut player_one, mut player_two) = create_two_players();

    loop {
        let turn = player_one_ai.play(&board, &player_one, &player_two);

        match turn {
            Turn::Move((x, y)) => {
                player_one.x = x;
                player_one.y = y;
            }
            Turn::Wall(wall) => {
                player_one.walls -= 1;
                board::place_wall(&mut board, wall);
            }
        }

        turns += 1;

        #[cfg(feature = "print_game")]
        print(&board, &player_one, &player_two);

        if player_one.y == board::get_board_height() - 1 {
            return GameResult {
                turns,
                winner: true,
            };
        }

        let turn = player_two_ai.play(&board, &player_two, &player_one);

        match turn {
            Turn::Move((x, y)) => {
                player_two.x = x;
                player_two.y = y;
            }
            Turn::Wall(wall) => {
                player_two.walls -= 1;
                board::place_wall(&mut board, wall);
            }
        }

        turns += 1;

        #[cfg(feature = "print_game")]
        print(&board, &player_one, &player_two);

        if player_two.y == 0 {
            return GameResult {
                turns,
                winner: false,
            };
        }
    }
}

pub(crate) fn create_two_players() -> (Player, Player) {
    (
        Player::new(4, 0, board::get_board_height() - 1),
        Player::new(4, 8, 0),
    )
}

#[cfg(feature = "print_game")]
fn print(board: &Board, player_one: &Player, player_two: &Player) {
    print!("┌");
    for _ in 0..board::get_board_width() - 1 {
        print!("───");
        print!("┬");
    }
    print!("───");
    println!("┐");

    for row in 0..board::get_board_height() {
        print!("│");
        for col in 0..board::get_board_width() {
            let user = if player_one.y == row && player_one.x == col {
                "x"
            } else if player_two.y == row && player_two.x == col {
                "o"
            } else {
                " "
            };
            print!(" {} ", user);
            if col != board::get_board_width() - 1 {
                print!(
                    "{}",
                    if !board[board::point_to_index(col, row)].right {
                        "│"
                    } else {
                        " "
                    }
                );
            }
        }
        print!("│");
        println!();

        if row != board::get_board_height() - 1 {
            print!("├");
            for col in 0..board::get_board_width() {
                print!(
                    "{}",
                    if !board[board::point_to_index(col, row)].down {
                        "───"
                    } else {
                        "   "
                    }
                );
                if col != board::get_board_width() - 1 {
                    print!("┼");
                }
            }
            print!("┤");
            println!();
        }
    }

    print!("└");
    for _ in 0..board::get_board_width() - 1 {
        print!("───");
        print!("┴");
    }
    print!("───");
    println!("┘");

    println!();

    thread::sleep(Duration::from_secs(1));
}
