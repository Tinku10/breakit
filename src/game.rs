mod ball;
mod board;
mod wall;
mod brick;

use ball::Ball;
use board::Board;
use wall::Wall;
use brick::Brick;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    style::{self},
    terminal::{self, WindowSize},
    QueueableCommand,
};
use std::io::{self, stdout, Write};

use std::time::Duration;

#[derive(Clone, Copy)]
pub struct GameDimension(u16, u16);

pub struct ObjectCoordinates(u16, u16, u16, u16);

pub struct Vector(f64, f64);

pub struct Game {
    ball: Ball,
    board: Board,
    walls: [Wall; 3],
    bricks: Vec<Brick>,
    dim: GameDimension,
}

pub trait GameObject {
    fn fill_object(&self, symbol: style::StyledContent<&str>) -> io::Result<()>;
    fn draw_object(&self) -> io::Result<()>;
    fn clear_object(&self) -> io::Result<()>;
    fn move_object(&mut self) -> io::Result<()>;
}

pub trait MoveCommand {
    fn move_right(&mut self) -> io::Result<()>;
    fn move_left(&mut self) -> io::Result<()>;
    fn move_up(&mut self) -> io::Result<()>;
    fn move_down(&mut self) -> io::Result<()>;
}

pub trait Collidable {
    fn get_velocity(&self) -> Vector;
    fn get_normal(&self, other: &dyn Collidable) -> &Vector;
    fn get_coordinates(&self) -> ObjectCoordinates;
    fn has_collision(&self, other: &dyn Collidable) -> bool;
}

pub trait GamePhysics {
    fn update_object(&mut self) -> io::Result<()>;
    fn handle_collision(&mut self, other: &dyn Collidable) -> io::Result<()>;
}

impl Game {
    pub fn new() -> Self {
        let WindowSize { rows, columns, .. } = terminal::window_size().unwrap();
        let dim = GameDimension(rows, columns);

        Game {
            dim,
            ball: Ball::new(dim.clone()),
            board: Board::new(dim.clone()),
            walls: [
                Wall::new(dim.clone(), wall::Direction::Left),
                Wall::new(dim.clone(), wall::Direction::Right),
                Wall::new(dim.clone(), wall::Direction::Top),
                // Wall::new(dim.clone(), wall::Direction::Bottom),
            ],
            bricks: (0..dim.1).filter(|x| x % 4 != 0).map(|x| Brick::new(x, 0)).collect(),
        }
    }

    fn setup(&self) -> io::Result<()> {
        let mut stdout = stdout();

        stdout
            .queue(terminal::EnterAlternateScreen)?
            .queue(cursor::Hide)?;

        terminal::enable_raw_mode();
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.setup()?;

        loop {
            self.ball.draw_object();
            for w in &self.walls {
                w.draw_object();
            }
            for b in &self.bricks {
                b.draw_object();
            }
            self.board.draw_object();
            if poll(Duration::from_millis(40))? {
                match read()? {
                    Event::Key(event) if event.code == KeyCode::Left => self.board.move_left(),
                    Event::Key(event) if event.code == KeyCode::Right => self.board.move_right(),
                    Event::Key(event) if event.code == KeyCode::Esc => break,
                    _ => Ok(()),
                };
            }
            for w in &self.walls {
                self.ball.handle_collision(w);
            }
            for b in &mut self.bricks {
                b.handle_collision(&self.ball);
                self.ball.handle_collision(b);
            }
            self.ball.handle_collision(&self.board);
            self.ball.update_object();
            // sleep(Duration::from_millis(5));
        }

        self.clear()?;

        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        let mut stdout = stdout();

        stdout
            .queue(cursor::Show)?
            .queue(terminal::LeaveAlternateScreen)?;

        let _ = stdout.flush()?;

        Ok(())
    }
}
