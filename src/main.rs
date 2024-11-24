mod game;

use game::Game;
use std::io;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    let _ = game.run();
    let _ = game.clear();
    Ok(())
}
