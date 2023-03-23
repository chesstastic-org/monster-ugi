<div align = "center">
<h1>monster-ugi</h1>
</div>

## Overview

`monster-ugi` is a fairy chess engine wrapper written in Rust for engines written using the `monster-chess` move generation library. It easily allows you to expose an API for your engine, and connect it to one of the **UCI**, **UAI**, or **UGI** protocols. It primarily aims for your engine to be compatible with [cutegames](https://github.com/kz04px/cutegames). Cutegames uses the [UGI](https://github.com/kz04px/cutegames/blob/master/ugi.md) protocol, which is a superset of the existing [UCI](https://backscattering.de/chess/uci/) and UAI protocols respectfully. To use it, implement the `EngineBehavior` trait:

```rust
impl<const T: usize> EngineBehavior<T> for RandomEngine<T> {
    fn select_move(&mut self, board: &mut Board<T>, time_control: TimeControl) -> MoveSelectionResults {
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
```

Then create an instance of engine:

```rust
let engine = Engine {
    behavior: RandomEngine::new(thread_rng()),
    game: Chess::create()
}
```

and run UGI.

```rust
run_ugi(engine);
```

## License

`monster-ugi` available under the
[MIT license](https://opensource.org/licenses/MIT). See
[LICENSE](https://github.com/chesstastic-org/monster-chess/blob/main/LICENSE) for the full
license text.
