mod game;

use game::Game;
use std::io;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.run()?;
    Ok(())
}
