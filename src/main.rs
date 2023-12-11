use monster_chess::games::chess::Chess;
use monster_ugi::{ugi::run_ugi, engine::Engine, random::RandomEngine};
use rand::thread_rng;

fn main() {
    run_ugi(Engine {
        game: Chess::create(),
        behavior: Box::new(RandomEngine(thread_rng()))
    })
}