use monster_chess::board::{game::{Game, GameResults}, Board, actions::Move};

pub struct Engine<const T: usize> {
    pub game: Game<T>,
    pub behavior: Box<dyn EngineBehavior<T>>
}

pub struct EngineInfo<'a> {
    pub name: &'a str,
    pub author: &'a str
}

#[derive(Debug)]
pub struct PlayerTime {
    pub time_ms: u128,
    pub inc_ms: u128
}

#[derive(Debug)]
pub enum TimeControl {
    Timed(Vec<PlayerTime>),
    MoveTime(u128),
    Depth(u32),
    Nodes(u64),
    Infinite
}

pub struct MoveSelectionResults {
    pub best_move: Move,

    /// Evaluation is in terms of centipieces (where one centi-piece is the lowest-value piece of the game)
    pub evaluation: u64
}

pub enum InitialPos<'a> {
    Startpos,
    Fen(&'a str)
}

pub struct Info<'a> {
    pub depth: Option<u32>,
    /// Score is in terms of centipieces (where one centi-piece is the lowest-value piece of the game)
    pub score: Option<u32>,
    pub pv: Option<&'a str>
}

pub trait EngineBehavior<const T: usize> {
    // UGI -> Engine

    fn get_engine_info(&mut self) -> EngineInfo;
    fn is_ready(&mut self) -> bool;

    fn is_over(&mut self, game: &Game<T>, board: &mut Board<T>) -> bool {
        match self.get_result(game, board) {
            GameResults::Ongoing => false,
            _ => true
        }
    }

    fn get_result(&mut self, game: &Game<T>, board: &mut Board<T>) -> GameResults {
        let legal_moves = board.generate_legal_moves(0);
        game.resolution.resolve(board, &legal_moves)
    }

    fn get_turn(&mut self, board: &Board<T>) -> u16 {
        board.state.moving_team
    }

    fn select_move(&mut self, board: &mut Board<T>, time_control: TimeControl, hashes: &Vec<u64>) -> MoveSelectionResults;
    fn stop_search(&mut self);

    fn position<'a>(&mut self, engine: &'a Engine<T>, initial_pos: InitialPos<'a>, actions: Vec<String>) -> Board<'a, T> {
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

    // Engine -> UGI

    fn init(&mut self, protocol: &str) {
        let engine_info = self.get_engine_info();
        println!("id name {}", engine_info.name);
        println!("id author {}", engine_info.author);
        self.ugiok(protocol);
    }

    fn info(&mut self, info: Info) {
        print!("info");
        
        if let Some(depth) = info.depth {
            print!(" depth {depth}");
        }

        if let Some(eval) = info.score {
            print!(" eval cp {eval}");
        }

        if let Some(pv) = info.pv {
            print!(" pv {pv}");
        }

        println!();
    }

    fn bestmove(&mut self, board: &Board<T>, action: Move) {
        println!("bestmove {}", board.encode_action(&action));
    }

    fn ugiok(&mut self, protocol: &str) {
        println!("{}ok", protocol);
    }

    fn readyok(&mut self) {
        println!("readyok");
    }
    
    fn response(&mut self, display: &str) {
        println!("response {display}");
    }

    fn response_bool(&mut self, display: bool) {
        self.response(&display.to_string())
    }
}