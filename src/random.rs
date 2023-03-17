use monster_chess::board::Board;
use rand::{seq::SliceRandom, rngs::ThreadRng};

use crate::engine::{EngineBehavior, Engine, TimeControl, MoveSelectionResults};

pub struct RandomEngine<const T: usize>(pub ThreadRng);

impl<const T: usize> EngineBehavior<T> for RandomEngine<T> {
    fn select_move(&mut self, board: &mut Board<T>, time_control: TimeControl) -> MoveSelectionResults {
        let best_move = *board.generate_legal_moves(0).choose(&mut self.0).expect("Must. Have. Random!!!");
        MoveSelectionResults {
            best_move,
            evaluation: 0
        }
    }

    fn init(&mut self) {
        println!("id name RANDOM MONSTER");
        println!("id author Corman");
    }

    fn is_ready(&mut self) -> bool {
        true
    }

    fn stop_search(&mut self) {}
}