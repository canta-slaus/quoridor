pub mod ai;
pub mod quoridor;

#[cfg(test)]
mod tests {
    use super::{
        ai::moving::MoveOnly,
        quoridor::{
            board,
            run::{create_two_players, run},
        },
    };

    #[test]
    fn valid_moves() {
        let board = board::create_new_board();
        let (player_one, player_two) = create_two_players();

        assert_eq!(
            board::get_valid_moves(&board, &player_one, &player_two).len(),
            3
        );
        assert_eq!(
            board::get_valid_moves(&board, &player_two, &player_one).len(),
            3
        );
    }

    #[test]
    fn valid_walls() {
        let mut board = board::create_new_board();
        let (player_one, player_two) = create_two_players();

        assert_eq!(
            board::get_valid_walls(&board, &player_one, &player_two).len(),
            128
        );

        board::place_wall(&mut board, (false, 1, 1));
        assert_eq!(
            board::get_valid_walls(&board, &player_one, &player_two).len(),
            124
        );

        let illegal_walls = vec![
            (false, 1, 1),
            (false, 0, 1),
            (false, 2, 1),
            (true, 1, 1),
            (false, board::get_board_width(), board::get_board_height()),
        ];

        let legal_walls = vec![(false, 3, 1), (false, 0, 0), (true, 0, 0)];

        assert!(illegal_walls.iter().all(|&wall| !board::can_place_wall(
            &board,
            &player_one,
            &player_two,
            wall
        )));
        assert!(legal_walls.iter().all(|&wall| board::can_place_wall(
            &board,
            &player_one,
            &player_two,
            wall
        )));
    }

    #[test]
    fn shortest_paths() {
        let board = board::create_new_board();
        let (player_one, player_two) = create_two_players();

        assert_eq!(
            board::get_path_to_goal(&board, &player_one, &player_two).len(),
            9
        );
        assert_eq!(
            board::get_path_to_goal(&board, &player_two, &player_one).len(),
            9
        );
    }

    #[test]
    fn jumping() {
        let board = board::create_new_board();
        let (mut player_one, mut player_two) = create_two_players();

        player_one.y = 1;

        assert_eq!(
            board::get_path_to_goal(&board, &player_one, &player_two).len(),
            8
        );
        assert_eq!(
            board::get_path_to_goal(&board, &player_two, &player_one).len(),
            8
        );

        player_two.y = 7;

        assert_eq!(
            board::get_path_to_goal(&board, &player_one, &player_two).len(),
            7
        );
        assert_eq!(
            board::get_path_to_goal(&board, &player_two, &player_one).len(),
            7
        );
    }

    #[test]
    fn test_game() {
        for _ in 0..10 {
            let player_one = MoveOnly::default();
            let player_two = MoveOnly::default();
            let result = run(player_one, player_two);

            assert_eq!(result.winner, false);
            assert_eq!(result.turns, 14);
        }
    }
}
