use engine::{EngineBehavior, Engine, TimeControl, MoveSelectionResults};
use monster_chess::board::Board;
use monster_chess::board::game::NORMAL_MODE;
use monster_chess::games::chess::Chess;
use rand::thread_rng;
use random::RandomEngine;

use std::io;
use std::io::prelude::*;

mod engine;
mod random;

fn main() {
    let mut engine = Engine {
        game: Chess::create(),
        behavior: Box::new(RandomEngine::<1>(thread_rng())),

    };
    let mut board: Option<Board<1>> = None;

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let protocol = lines.next().expect("No user input.").expect("User input error.");
    if protocol != "ugi" {
        println!("Unknown protocol.");
        return;
    }

    engine.behavior.init();
    engine.behavior.ugiok();

    for line in lines {
        let mut line = line.expect("User input error.");
        if line == "isready" {
            if engine.behavior.is_ready() {
                engine.behavior.readyok();
            }

            continue;
        }

        if line.starts_with("position ") {
            line = line.strip_prefix("position ").expect("String dynamics changed throughout spacetime.").to_string();

            let mut new_board = if line.starts_with("startpos") {
                line = line.strip_prefix("startpos").expect("String dynamics changed throughout spacetime.").to_string();
                engine.game.default()
            } else {
                assert!(line.starts_with("fen"), "'position' has two subcommands: 'position startpos' and 'position fen'. 'position {}' is invalid.", line);
                line = line.strip_prefix("fen").expect("String dynamics changed throughout spacetime.").to_string();
                engine.game.default()
            };

            if line.starts_with(" moves ") {
                line = line.strip_prefix(" moves ").expect("String dynamics changed throughout spacetime.").to_string();
                for action in line.split(" ") {
                    let action = new_board.decode_action(action, NORMAL_MODE).expect(&format!("{} is not a possible move.", action));
                    new_board.make_move(&action);
                }
            }

            board = Some(new_board);

            continue;
        }

        if line.starts_with("go") {
            // TODO: Actually implement TCs

            match &mut board {
                Some(board) => {
                    let MoveSelectionResults { best_move, .. } = engine.behavior.select_move(board, TimeControl::Timed(vec![]));
                    engine.behavior.bestmove(board, best_move)
                }
                None => {
                    println!("Cannot run 'go' if no board has been initialized with the 'position' command.");
                    return;
                }
            }
        }

        // TODO: Implement query
    }
}
