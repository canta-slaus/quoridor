use super::super::quoridor::{
    board::{self, Board},
    player::Player,
    run::{Turn, AI},
};

use rand::{rngs::ThreadRng, Rng};

pub struct Random {
    rng: ThreadRng,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl AI for Random {
    fn play(&mut self, board: &Board, player: &Player, enemy: &Player) -> Turn {
        let mut moves = board::get_valid_moves(board, player, enemy);

        if player.walls > 0 {
            moves.append(
                &mut board::get_valid_walls(board, player, enemy)
                    .iter()
                    .map(|&wall| Turn::Wall(wall))
                    .collect(),
            );
        }

        let index = self.rng.gen_range(0..moves.len());
        moves[index]
    }
}

pub struct RandomMoving {
    rng: ThreadRng,
}

impl Default for RandomMoving {
    fn default() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl AI for RandomMoving {
    fn play(&mut self, board: &Board, player: &Player, enemy: &Player) -> Turn {
        let moves = board::get_valid_moves(board, player, enemy);

        let index = self.rng.gen_range(0..moves.len());
        moves[index]
    }
}
