use monster_chess::games::chess::Chess;
use monster_ugi::{ugi::run_ugi, random::RandomEngine, engine::Engine};

pub fn main() {
    run_ugi(Engine {
        game: Chess::create(),
        behavior: Box::new(RandomEngine(rand::thread_rng()))
    });
}