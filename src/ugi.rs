use crate::engine::{Engine, TimeControl, MoveSelectionResults, PlayerTime};

use monster_chess::board::Board;
use monster_chess::board::game::{NORMAL_MODE, GameResults};

use std::io;
use std::io::prelude::*;

pub fn run_ugi<const T: usize>(mut engine: Engine<T>) {
    let mut board: Option<Board<T>> = None;
    let mut hashes: Vec<u64> = vec![];

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let protocol = lines.next().expect("No user input.").expect("User input error.");
    if protocol != "ugi" && protocol != "uci" && protocol != "uai" {
        println!("Unknown protocol.");
        return;
    }

    engine.behavior.init(&protocol);

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
                assert!(line.starts_with("fen "), "'position' has two subcommands: 'position startpos' and 'position fen'. 'position {}' is invalid.", line);
                line = line.strip_prefix("fen ").expect("String dynamics changed throughout spacetime.").to_string();

                let newline = line.clone();
                let moves = newline.split(" moves").collect::<Vec<_>>();
                line = line.strip_prefix(moves[0]).expect("String dynamics changed throughout spacetime.").to_string();

                engine.game.from_fen(moves[0])
            };

            hashes.clear();
            hashes.push(new_board.game.zobrist.compute(&new_board));

            if line.starts_with(" moves ") {
                line = line.strip_prefix(" moves ").expect("String dynamics changed throughout spacetime.").to_string();
                for action in line.split(" ") {
                    let action = new_board.decode_action(action, NORMAL_MODE).expect(&format!("{} is not a possible move.", action));
                    new_board.make_move(&action);
                    hashes.push(new_board.game.zobrist.compute(&new_board));
                }
            }

            board = Some(new_board);

            continue;
        }

        if line.starts_with("go ") {
            line = line.strip_prefix("go ").expect("String dynamics changed throughout spacetime.").to_string();

            let mut time_control = TimeControl::Infinite;
            if line.starts_with("p1time") || line.starts_with("wtime") {
                let info: [ &str; 8 ] = line.split(" ").collect::<Vec<_>>().try_into().expect("Could not convert '{info}' into a time+inc time control.");
                let [ _, p1time, _, p2time, _, p1inc, _, p2inc ] = info;
                let [ p1time, p2time, p1inc, p2inc ]: [ u128; 4 ] = [ p1time, p2time, p1inc, p2inc ]
                    .iter().map(|el| el.parse::<u128>().expect("Could not convert '{info}' into MS for time control"))
                    .collect::<Vec<_>>().try_into().unwrap();
                time_control = TimeControl::Timed(vec![ 
                    PlayerTime { time_ms: p1time, inc_ms: p1inc }, 
                    PlayerTime { time_ms: p2time, inc_ms: p2inc }
                ])
            } else if line.starts_with("movetime ") {
                line = line.strip_prefix("movetime ").expect("String dynamics changed throughout spacetime.").to_string();
                let movetime_ms = line.parse::<u128>().expect("Could not convert '{info}' into MS for time control");
                time_control = TimeControl::MoveTime(movetime_ms);
            } else if line.starts_with("depth ") {
                line = line.strip_prefix("depth ").expect("String dynamics changed throughout spacetime.").to_string();
                let depth = line.parse::<u32>().expect("Could not convert '{info}' into depth for time control");
                time_control = TimeControl::Depth(depth);
            } else if line.starts_with("nodes ") {
                line = line.strip_prefix("nodes ").expect("String dynamics changed throughout spacetime.").to_string();
                let nodes = line.parse::<u64>().expect("Could not convert '{nodes}' into depth for time control");
                time_control = TimeControl::Nodes(nodes);
            }

            match &mut board {
                Some(board) => {
                    let MoveSelectionResults { best_move, .. } = engine.behavior.select_move(board, time_control, &hashes);
                    engine.behavior.bestmove(board, best_move)
                }
                None => {
                    println!("Cannot run 'go' if no board has been initialized with the 'position' command.");
                    return;
                }
            }
        }

        if line.starts_with("query ") {
            match &mut board {
                Some(board) => {
                    line = line.strip_prefix("query ").expect("String dynamics changed throughout spacetime.").to_string();
                    if line == "gameover" {
                        let is_over = engine.behavior.is_over(&engine.game, board);
                        engine.behavior.response_bool(is_over);
                    } else if line == "p1turn" {
                        let first_turn = engine.behavior.get_turn(board) == 0;
                        engine.behavior.response_bool(first_turn);
                    } else if line == "result" {
                        let result = match engine.behavior.get_result(&engine.game, board) {
                            GameResults::Win(team) => format!("p{}win", team + 1),
                            GameResults::Draw => "draw".to_string(),
                            GameResults::Ongoing => "none".to_string()
                        };
                        engine.behavior.response(&result);
                    }
                }
                None => {
                    println!("Cannot run 'query' if no board has been initialized with the 'position' command.");
                    return;
                }
            }
        }
    }
}
