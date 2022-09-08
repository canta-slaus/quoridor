use super::super::quoridor::{
    board::{self, Board},
    player::Player,
    run::{Turn, AI},
};

#[derive(Default)]
pub struct MoveOnly {}

impl AI for MoveOnly {
    fn play(&mut self, board: &Board, player: &Player, enemy: &Player) -> Turn {
        let mut path = board::get_path_to_goal(board, player, enemy);
        let _current = path.pop_front().unwrap();
        let next = path.pop_front().unwrap();

        Turn::Move(next)
    }
}
