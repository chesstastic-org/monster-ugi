use std::{thread, time::Duration};

use monster_chess::board::Board;
use rand::{seq::SliceRandom, rngs::ThreadRng};

use crate::engine::{EngineBehavior, TimeControl, MoveSelectionResults, EngineInfo};

pub struct RandomEngine<const T: usize>(pub ThreadRng);

impl<const T: usize> EngineBehavior<T> for RandomEngine<T> {
    fn select_move(&mut self, board: &mut Board<T>, _time_control: TimeControl, _hashes: &Vec<u64>) -> MoveSelectionResults {
        thread::sleep(Duration::from_millis(350));
        let best_move = *board.generate_legal_moves(0).choose(&mut self.0).expect("Could not find a move to pick between for random movegen.");
        MoveSelectionResults {
            best_move,
            evaluation: 0
        }
    }

    fn get_engine_info(&mut self) -> EngineInfo {
        EngineInfo {
            name: "Random",
            author: "Corman"
        }
    }

    fn is_ready(&mut self) -> bool {
        true
    }

    fn stop_search(&mut self) {}
}