use monster_chess::board::{game::{Game, GameResults}, Board, actions::Action};

pub struct Engine<const T: usize> {
    game: Game<T>,
    behavior: Box<dyn EngineBehavior<T>>
}

pub struct PlayerTime {
    time_ms: u128,
    inc_ms: u128
}

pub enum TimeControl {
    Timed(Vec<PlayerTime>)
}

pub struct MoveSelectionResults {
    best_move: Action,

    // Evaluation is in terms of centipieces (where one centi-piece is the lowest-value piece of the game)
    evaluation: u64
}

pub enum InitialPos<'a> {
    Startpos,
    Fen(&'a str)
}

pub trait EngineBehavior<const T: usize> {
    fn is_over(&self, engine: &Engine<T>, board: &mut Board<T>) -> bool {
        match self.get_result(engine, board) {
            GameResults::Ongoing => false,
            _ => true
        }
    }

    fn get_result(&self, engine: &Engine<T>, board: &mut Board<T>) -> GameResults {
        let legal_moves = board.generate_legal_moves(0);
        engine.game.resolution.resolution(board, &legal_moves)
    }

    fn select_move(&self, engine: &Engine<T>, board: &mut Board<T>, time_control: TimeControl) -> MoveSelectionResults;
    fn stop_search(&self, engine: &Engine<T>);

    fn position<'a>(&self, engine: &'a Engine<T>, initial_pos: InitialPos<'a>, actions: Vec<String>) -> Board<'a, T> {
        let mut board = match initial_pos {
            InitialPos::Fen(fen) => engine.game.from_fen(&fen),
            InitialPos::Startpos => engine.game.default()
        };

        for action in actions {
            let action = board.decode_action(&action, 0).expect(
                &format!("Could not find action {action} from FEN {}", board.to_fen())
            );
            board.make_move(&action);
        }

        board
    }
}