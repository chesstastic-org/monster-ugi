use monster_chess::board::Board;
use rand::{seq::SliceRandom, rngs::ThreadRng};

use crate::engine::{EngineBehavior, Engine, TimeControl, MoveSelectionResults, EngineInfo};

pub struct RandomEngine<const T: usize>(pub ThreadRng);

impl<const T: usize> EngineBehavior<T> for RandomEngine<T> {
    fn select_move(&mut self, board: &mut Board<T>, time_control: TimeControl, hashes: &Vec<u64>) -> MoveSelectionResults {
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