use quoridor::{ai::moving::MoveOnly, quoridor::run::run};

fn main() {
    let player_one = MoveOnly::default();
    let player_two = MoveOnly::default();
    let result = run(player_one, player_two);

    println!("{} {}", result.winner, result.turns);
}
