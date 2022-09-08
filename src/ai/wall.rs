use super::super::quoridor::{
    board::{self, Board},
    player::Player,
    run::{Turn, AI},
};

use rand::{rngs::ThreadRng, Rng};

pub struct WallFirstMax {
    rng: ThreadRng,
}

impl Default for WallFirstMax {
    fn default() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl AI for WallFirstMax {
    fn play(&mut self, board: &Board, player: &Player, enemy: &Player) -> Turn {
        if player.walls > 0 {
            let walls = board::get_best_max_walls(board, player, enemy);
            if !walls.is_empty() {
                let index = self.rng.gen_range(0..walls.len());
                return walls[index];
            }
        }

        let mut path = board::get_path_to_goal(board, player, enemy);
        let _current = path.pop_front().unwrap();
        let next = path.pop_front().unwrap();

        Turn::Move(next)
    }
}

pub struct WallFirstMinmax {
    rng: ThreadRng,
}

impl Default for WallFirstMinmax {
    fn default() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl AI for WallFirstMinmax {
    fn play(&mut self, board: &Board, player: &Player, enemy: &Player) -> Turn {
        if player.walls > 0 {
            let walls = board::get_best_minmax_walls(board, player, enemy);
            if !walls.is_empty() {
                let index = self.rng.gen_range(0..walls.len());
                return walls[index];
            }
        }

        let mut path = board::get_path_to_goal(board, player, enemy);
        let _current = path.pop_front().unwrap();
        let next = path.pop_front().unwrap();

        Turn::Move(next)
    }
}
