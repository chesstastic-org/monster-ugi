use crate::engine::{Engine, TimeControl, MoveSelectionResults, PlayerTime};

use monster_chess::board::Board;
use monster_chess::board::game::{NORMAL_MODE, GameResults};

use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Chunks<'a, 'b>(Vec<&'a[&'b str]>);

impl Chunks<'_, '_> {
    pub fn has_named<const T: usize>(&self, names: [ &str; T ]) -> bool {
        self.0.iter().any(|chunk| names.contains(&chunk[0]))
    }
    
    pub fn get_raw<const T: usize>(&self, names: [ &str; T ]) -> Option<String> {
        self.0.iter().find(|chunk| names.contains(&chunk[0]))
            .map(|el| el[1].to_string())
    }
    
    pub fn get_int<const T: usize>(&self, names: [ &str; T ]) -> Option<u128> {
        self.get_raw(names).map(|raw| {
            raw
                .parse::<u128>()
                .expect("Expected integer argument")
        })
    }
}

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

                let newline = line.clone();
                let components = newline.split(" moves").collect::<Vec<_>>();

                if components.len() > 1 {
                    line = format!("moves {}", components[1].trim().to_string())
                };
                engine.game.default()
            } else {
                assert!(line.starts_with("fen "), "'position' has two subcommands: 'position startpos' and 'position fen'. 'position {}' is invalid.", line);
                line = line.strip_prefix("fen ").expect("String dynamics changed throughout spacetime.").to_string();

                let newline = line.clone();
                let components = newline.split(" moves").collect::<Vec<_>>();

                let position = components[0];
                if components.len() > 1 {
                    line = format!("moves {}", components[1].trim().to_string())
                };

                engine.game.from_fen(&position)
            };

            hashes.clear();
            hashes.push(new_board.game.zobrist.compute(&new_board));

            if line.starts_with("moves ") {
                line = line.strip_prefix("moves ")
                    .expect("String dynamics changed throughout spacetime.").to_string();
                for action in line.split(" ") {
                    let action = new_board.decode_action(action, NORMAL_MODE)
                        .expect(&format!("{} is not a possible move.", action));
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

            let info = line.split(" ").collect::<Vec<_>>();
            let chunks = Chunks(info.chunks(2).collect::<Vec<_>>());

            if chunks.has_named([ "p1time", "wtime" ]) {
                let p1time = chunks.get_int([ "p1time", "wtime" ]).expect("Must have p1time");
                let p2time = chunks.get_int([ "p2time", "btime" ]).expect("Must have p2time");
                let p1inc = chunks.get_int([ "p1time", "binc" ]).unwrap_or(0);
                let p2inc = chunks.get_int([ "p2inc", "binc" ]).unwrap_or(0);

                    time_control = TimeControl::Timed(vec![ 
                    PlayerTime { time_ms: p1time, inc_ms: p1inc }, 
                    PlayerTime { time_ms: p2time, inc_ms: p2inc }
                ]);
            } else if chunks.has_named([ "movetime" ]) {
                let movetime_ms = chunks.get_int([ "movetime" ]).expect("Must have movetime");
                time_control = TimeControl::MoveTime(movetime_ms);
            } else if chunks.has_named([ "depth" ]) {
                let depth = chunks.get_int([ "depth" ]).expect("Must have depth") as u32;
                time_control = TimeControl::Depth(depth);
            } else if line.starts_with("nodes ") {
                let nodes = chunks.get_int([ "nodes" ]).expect("Must have nodes") as u64;
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
