mod chip8;
mod game;
mod interface;

fn main() {
    if let Err(e) = game::run() {
        eprintln!("{}", e);
    }
}
