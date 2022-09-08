use super::{
    player::Player,
    run::{MoveData, Turn, WallData},
};

use std::collections::{HashSet, VecDeque};

const BOARD_WIDTH: usize = 9;
const BOARD_HEIGHT: usize = 9;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub type Board = Vec<Node>;

#[derive(Clone)]
pub struct Node {
    pub right: bool,
    pub down: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn create_new_board() -> Board {
    let mut board = Vec::with_capacity(BOARD_SIZE);
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            board.push(Node {
                right: col != BOARD_WIDTH - 1,
                down: row != BOARD_HEIGHT - 1,
            });
        }
    }

    board
}

pub const fn get_board_width() -> usize {
    BOARD_WIDTH
}

pub const fn get_board_height() -> usize {
    BOARD_HEIGHT
}

pub const fn get_board_size() -> usize {
    BOARD_SIZE
}

pub fn point_to_index(x: usize, y: usize) -> usize {
    y * BOARD_WIDTH + x
}

pub fn index_to_point(index: usize) -> (usize, usize) {
    let y = index / BOARD_WIDTH;
    (index - y * BOARD_WIDTH, y)
}

pub fn can_place_wall(
    board: &Board,
    player_one: &Player,
    player_two: &Player,
    wall: WallData,
) -> bool {
    let (vert, x, y) = wall;

    if x > BOARD_WIDTH - 1 || y > BOARD_HEIGHT - 1 {
        return false;
    }

    let one = &board[point_to_index(x, y)];
    let two = &board[point_to_index(x + 1, y)];
    let three = &board[point_to_index(x, y + 1)];

    if (vert && (!one.right || !three.right))
        || (!vert && (!one.down || !two.down))
        || (!one.down && !two.down)
        || (!one.right && !three.right)
    {
        return false;
    }

    let mut board = board.clone();
    place_wall(&mut board, wall);

    !get_path_to_goal(&board, player_one, player_two).is_empty()
        && !get_path_to_goal(&board, player_two, player_one).is_empty()
}

fn wall_heuristic(
    board: &Board,
    player_one: &Player,
    player_two: &Player,
    wall: WallData,
) -> (usize, usize) {
    let mut board = board.clone();
    place_wall(&mut board, wall);

    (
        get_path_to_goal(&board, player_one, player_two).len(),
        get_path_to_goal(&board, player_two, player_one).len(),
    )
}

pub(crate) fn place_wall(board: &mut Board, wall: WallData) {
    if wall.0 {
        let one = point_to_index(wall.1, wall.2);
        let two = point_to_index(wall.1, wall.2 + 1);

        board[one].right = false;
        board[two].right = false;
    } else {
        let one = point_to_index(wall.1, wall.2);
        let two = point_to_index(wall.1 + 1, wall.2);

        board[one].down = false;
        board[two].down = false;
    }
}

fn can_move(board: &Board, x: usize, y: usize, direction: Direction) -> bool {
    match direction {
        Direction::Up => y > 0 && board[point_to_index(x, y - 1)].down,
        Direction::Down => y < BOARD_HEIGHT - 1 && board[point_to_index(x, y)].down,
        Direction::Left => x > 0 && board[point_to_index(x - 1, y)].right,
        Direction::Right => x < BOARD_WIDTH - 1 && board[point_to_index(x, y)].right,
    }
}

pub fn get_valid_moves(board: &Board, player_one: &Player, player_two: &Player) -> Vec<Turn> {
    let x = player_one.x;
    let y = player_one.y;
    let mut moves = Vec::new();

    if can_move(board, x, y, Direction::Up) {
        let y = y - 1;
        if x == player_two.x && y == player_two.y {
            if can_move(board, x, y, Direction::Up) {
                moves.push(Turn::Move((x, y - 1)));
            } else {
                if can_move(board, x, y, Direction::Left) {
                    moves.push(Turn::Move((x - 1, y)));
                }
                if can_move(board, x, y, Direction::Right) {
                    moves.push(Turn::Move((x + 1, y)));
                }
            }
        } else {
            moves.push(Turn::Move((x, y)));
        }
    }

    if can_move(board, x, y, Direction::Down) {
        let y = y + 1;
        if x == player_two.x && y == player_two.y {
            if can_move(board, x, y, Direction::Down) {
                moves.push(Turn::Move((x, y + 1)));
            } else {
                if can_move(board, x, y, Direction::Left) {
                    moves.push(Turn::Move((x - 1, y)));
                }
                if can_move(board, x, y, Direction::Right) {
                    moves.push(Turn::Move((x + 1, y)));
                }
            }
        } else {
            moves.push(Turn::Move((x, y)));
        }
    }

    if can_move(board, x, y, Direction::Left) {
        let x = x - 1;
        if x == player_two.x && y == player_two.y {
            if can_move(board, x, y, Direction::Left) {
                moves.push(Turn::Move((x - 1, y)));
            } else {
                if can_move(board, x, y, Direction::Up) {
                    moves.push(Turn::Move((x, y - 1)));
                }
                if can_move(board, x, y, Direction::Down) {
                    moves.push(Turn::Move((x, y + 1)));
                }
            }
        } else {
            moves.push(Turn::Move((x, y)));
        }
    }

    if can_move(board, x, y, Direction::Right) {
        let x = x + 1;
        if x == player_two.x && y == player_two.y {
            if can_move(board, x, y, Direction::Right) {
                moves.push(Turn::Move((x + 1, y)));
            } else {
                if can_move(board, x, y, Direction::Up) {
                    moves.push(Turn::Move((x, y - 1)));
                }
                if can_move(board, x, y, Direction::Down) {
                    moves.push(Turn::Move((x, y + 1)));
                }
            }
        } else {
            moves.push(Turn::Move((x, y)));
        }
    }

    moves
}

pub fn get_valid_walls(board: &Board, player_one: &Player, player_two: &Player) -> Vec<WallData> {
    let mut moves = Vec::new();

    for row in 0..get_board_height() - 1 {
        for col in 0..get_board_width() - 1 {
            let vert = (true, col, row);
            let hor = (false, col, row);

            if can_place_wall(board, player_one, player_two, vert) {
                moves.push(vert);
            }
            if can_place_wall(board, player_one, player_two, hor) {
                moves.push(hor);
            }
        }
    }

    moves
}

pub fn get_best_max_walls(board: &Board, player_one: &Player, player_two: &Player) -> Vec<Turn> {
    let mut turns = Vec::new();
    let mut max = 0;

    for wall in get_valid_walls(board, player_one, player_two) {
        let (own_path, new_max) = wall_heuristic(board, player_one, player_two, wall);

        if own_path != 0 && new_max != 0 && new_max >= max {
            if new_max > max {
                max = new_max;
                turns.clear();
            }

            turns.push(Turn::Wall(wall));
        }
    }

    turns
}

pub fn get_best_minmax_walls(board: &Board, player_one: &Player, player_two: &Player) -> Vec<Turn> {
    let mut turns = Vec::new();
    let mut max = isize::MIN;

    for wall in get_valid_walls(board, player_one, player_two) {
        let (own_path, enemy_path) = wall_heuristic(board, player_one, player_two, wall);

        if own_path != 0 && enemy_path != 0 {
            let new_max = (enemy_path as isize) - (own_path as isize);

            if new_max > max {
                max = new_max;
                turns.clear();
            }

            turns.push(Turn::Wall(wall));
        }
    }

    turns
}

pub fn get_path_to_goal(
    board: &Board,
    player_one: &Player,
    player_two: &Player,
) -> VecDeque<MoveData> {
    let root = point_to_index(player_one.x, player_one.y);
    let mut open = HashSet::new();

    let mut came_from = vec![usize::MAX; get_board_size()];
    let mut g_score = vec![usize::MAX; get_board_size()];
    let mut f_score = vec![usize::MAX; get_board_size()];

    open.insert(root);
    g_score[root] = 0;

    while !open.is_empty() {
        let current = *open
            .iter()
            .min_by(|&&x, &&y| f_score[x].cmp(&f_score[y]))
            .unwrap();
        open.remove(&current);

        let (x, y) = index_to_point(current);

        if y == player_one.end_y {
            let mut path = VecDeque::from([(x, y)]);
            let mut current = current;
            let player_two_index = point_to_index(player_two.x, player_two.y);

            while current != root {
                current = came_from[current];
                if player_two_index != current {
                    let (x, y) = index_to_point(current);
                    path.push_front((x, y));
                }
            }

            return path;
        }

        // Check up
        if y != 0 {
            let n_x = x;
            let n_y = y - 1;
            let n_index = point_to_index(n_x, n_y);
            if board[n_index].down {
                expand_node(
                    &mut open,
                    &mut came_from,
                    &mut g_score,
                    &mut f_score,
                    current,
                    n_index,
                    n_x,
                    n_y,
                    player_one.end_y,
                    player_two,
                );
            }
        }

        // Check down
        if y != get_board_height() - 1 {
            let n_x = x;
            let n_y = y + 1;
            let n_index = point_to_index(n_x, n_y);
            if board[current].down {
                expand_node(
                    &mut open,
                    &mut came_from,
                    &mut g_score,
                    &mut f_score,
                    current,
                    n_index,
                    n_x,
                    n_y,
                    player_one.end_y,
                    player_two,
                );
            }
        }

        // Check left
        if x != 0 {
            let n_x = x - 1;
            let n_y = y;
            let n_index = point_to_index(n_x, n_y);
            if board[n_index].right {
                expand_node(
                    &mut open,
                    &mut came_from,
                    &mut g_score,
                    &mut f_score,
                    current,
                    n_index,
                    n_x,
                    n_y,
                    player_one.end_y,
                    player_two,
                );
            }
        }

        // Check right
        if x != get_board_width() - 1 {
            let n_x = x + 1;
            let n_y = y;
            let n_index = point_to_index(n_x, n_y);
            if board[current].right {
                expand_node(
                    &mut open,
                    &mut came_from,
                    &mut g_score,
                    &mut f_score,
                    current,
                    n_index,
                    n_x,
                    n_y,
                    player_one.end_y,
                    player_two,
                );
            }
        }
    }

    VecDeque::new()
}

fn expand_node(
    open: &mut HashSet<usize>,
    came_from: &mut Vec<usize>,
    g_score: &mut Vec<usize>,
    f_score: &mut Vec<usize>,
    current: usize,
    n_index: usize,
    n_x: usize,
    n_y: usize,
    end_y: usize,
    player_two: &Player,
) {
    let cost = if n_x == player_two.x && n_y == player_two.y {
        0
    } else {
        1
    };
    let score = g_score[current] + cost;
    if score < g_score[n_index] {
        came_from[n_index] = current;
        g_score[n_index] = score;
        f_score[n_index] = score + n_y.abs_diff(end_y);
        if !open.contains(&n_index) {
            open.insert(n_index);
        }
    }
}
