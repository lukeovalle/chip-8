mod chip8;
mod game;
mod interface;

use std::env;


fn main() {
    let file = env::args().nth(1).unwrap_or("game.ch8".to_string());

    if let Err(e) = game::run(&file) {
        eprintln!("{}", e);
    }
}
