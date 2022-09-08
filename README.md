# Quoridor in Rust
[Quoridor](https://en.wikipedia.org/wiki/Quoridor) is a zero-sum strategic board game with complete information. It was initially designed by Marko Marchesi based on [Blockade](https://en.wikipedia.org/wiki/Blockade_(board_game)) by Philip Slater.

## Gameplay
![Starting board for a 4-player game](https://en.wikipedia.org/wiki/File:Quoridor-game-board-initial-position.svg)

Quoridor is played on a 9x9 board, either as a 2-player game (red and blue) or as a 4-player game. The goal of each player is to get to the other side of the board. So the red player would have to reach the very bottom row of the board and the blue player would have to reach the very top row, respectively. During their turn players can either move or place a wall. They can move to any adjacent tile that is not blocked by wall but can't move diagonally. There is also rules that allow players to jump over another player (see the Wikipedia entry for the full set of rules). In a 2-player game both players get 10 walls each (in a 4-player game everyone gets 5). A wall is two tiles wide and can be placed between tiles - they prevent players from moving as walls can't be jumped. Walls can't be placed on top of each other (on an actual board this is physically impossible) and one may not place a wall such that a player can't reach their goal.

## Implementation and idea
The main idea of this project was to implement a simple board and allow users to not only play the game but implement and try different AIs.

The entry point of the game is the `run` function: it takes two "players" (structs that implement the `AI` trait). It then runs the game until it's over. When simulating the game, it does not check whether a move is valid or not - gentlemen's agreement - since it seemed unnecessary to check if a move is legal since the AI most likely already made sure of that (the provided helper functions like `get_valid_walls` obviously only return _valid_ wall placements).

The board is a plain `Vec<Node>`. A node represents the "state" of a tile:
```rs
struct Node {
    right: bool,
    down: bool,
}
```
`right` and `down` specify whether a player could move right/down from this tile. Having `left` and `up` is redundant, as two adjacent tiles would be holding "duplicate" information (`right` on tile A "==" `left` on tile B and same for `up`/`down`).

There is a few simple tests to make sure everything is implemented properly (and doesn't break when I fix the pathfinding). In the [`examples`](examples/) folder there is an example simulation to let two AIs play against each other, you can compile and run it as followed:

```sh
cargo run --example test
```

Additionally, since just knowing which AIs is a little boring, we can watch them play against each other by enabling the `print_game` feature:

```sh
cargo run --example test --features print_game
```

## Current AIs
Most of the currently implemented AIs are very primitive and straight forward.
- _MoveOnly_: takes the shortest path to its goal every turn
- _Random_: _random_
- _RandomMoving_: similar to _Random_ but doesn't place walls, it only moves randomly
They were mostly implemented to test how "random" "randomness" can be and to test the simulation and helper functions.
`WallFirstMax` and `WallFirstMinmax` are little more sophisticated, but not perfect. `WallFirstMax` iterates over all legal wall positions and picks the one that extends the enemy path the most (if there is multiple, it picks one at random). `WallFirstMinmax` turned out worse than I expected, the idea was to maximize the enemy path while keeping my own path as short as possible but this turned out to be a very weak algorithm.

## To-Do
Currently, the path finding is flawed: It doesn't really take jumping-over-a-player into consideration.

Based on a few other implementations of Quoridor and a few research papers, there is still a few more AIs I'd like to try and implement, as well as an actual "player" player (so you can play against the AIs yourself, instead of just watching them play). I'd also like to include better test cases and have more examples ready to run.

Check out other implementations of Quoridor:
- https://github.com/gorisanson/quoridor-ai
- https://github.com/danielborowski/quoridor-ai/
- https://github.com/boriel/quoridor
- https://github.com/MNLR/Quoridor

These papers helped me grasp the sheer complexity of Quoridor and helped me better understand the game as well as implement certain things.
- https://project.dke.maastrichtuniversity.nl/games/files/bsc/Mertens_BSc-paper.pdf
- https://www.researchgate.net/publication/327679826_Monte_Carlo_Tree_Search_for_Quoridor
- https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.100.5204&rep=rep1&type=pdf
