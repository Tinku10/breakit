use crossterm::{
    cursor,
    event::{self, poll, read, Event, KeyCode, KeyEvent},
    style::{self, style, Stylize},
    terminal::{self, WindowSize},
    ExecutableCommand, QueueableCommand,
};

use super::{
    Ball, Collidable, GameDimension, GameObject, GamePhysics, MoveCommand, ObjectCoordinates,
    ObjectVelocity, SurfaceNormal,
};
use std::any::Any;
use std::cmp::*;
use std::io::{self, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const MOVE_INCR: u16 = 2;

pub struct Board {
    pub pos: u16,
    pub width: u16,
    pub velocity: f64,
    dim: GameDimension,
    normals: [SurfaceNormal; 1],
}

impl Board {
    pub fn new(dim: GameDimension) -> Board {
        const BOARD_WIDTH: u16 = 10;
        let pos = dim.1 / 2 - BOARD_WIDTH / 2 + 20;

        Board {
            pos,
            width: BOARD_WIDTH,
            velocity: 0.0,
            dim,
            normals: [SurfaceNormal(0, -1)],
        }
    }
}

impl Collidable for Board {
    fn get_coordinates(&self) -> ObjectCoordinates {
        ObjectCoordinates(self.pos, self.dim.0, self.pos + self.width, self.dim.0)
    }
    fn has_collision(&self, other: &dyn Collidable) -> bool {
        let ObjectCoordinates(ax1, ay1, ax2, ay2) = self.get_coordinates();
        let ObjectCoordinates(bx1, by1, bx2, by2) = other.get_coordinates();

        let x1 = max(ax1, bx1);
        let x2 = max(ax2, bx2);
        let y1 = min(ay1, by1);
        let y2 = min(ay2, by2);

        if x1 <= x2 && y1 <= y2 {
            return true;
        }
        return false;
    }
    fn get_normal(&self, other: &dyn Collidable) -> &super::SurfaceNormal {
        let ObjectCoordinates(ox1, oy1, ox2, oy2) = other.get_coordinates();
        let ObjectCoordinates(sx1, sy1, sx2, sy2) = self.get_coordinates();

        if oy1 <= sy1 {
            // Collision on top
            &self.normals[0]
        } else {
            println!("{} {}", ox1, sx1);
            todo!("Cannot hit other surfaces yet!")
        }
    }
    fn get_velocity(&self) -> super::ObjectVelocity {
        ObjectVelocity(self.velocity, 0.0)
    }
}

impl GameObject for Board {
    fn fill_object(&self, symbol: style::StyledContent<&str>) -> io::Result<()> {
        let mut stdout = stdout();

        let board_l = max(0, self.pos);
        let board_r = min(self.dim.1, self.pos + self.width);

        // draw the new board
        for i in board_l..=board_r {
            stdout
                .queue(cursor::MoveTo(i, self.dim.0 - 1))?
                // seems to write the whole line from y removing previously
                // drawn characters to the right
                .queue(style::PrintStyledContent(symbol))?;
            // .queue(cursor::hide)?;
        }
        stdout.flush();
        Ok(())
    }

    fn draw_object(&self) -> io::Result<()> {
        self.fill_object("▇".with(style::Color::Green))
    }

    fn clear_object(&self) -> io::Result<()> {
        self.fill_object(style(" "))
    }

    fn move_object(&mut self) -> io::Result<()> {
        self.clear_object()?;
        let new_pos =
            (self.pos as f64 + self.velocity).clamp(0.0, (self.dim.1 - self.width) as f64);
        self.pos = new_pos as u16;
        Ok(())
    }
}

impl MoveCommand for Board {
    fn move_right(&mut self) -> io::Result<()> {
        self.velocity = 4.0;
        self.move_object()
    }

    fn move_left(&mut self) -> io::Result<()> {
        self.velocity = -4.0;
        self.move_object()
    }

    fn move_up(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn move_down(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl GamePhysics for Board {
    fn update_object(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn handle_collision(&mut self, other: &dyn Collidable) -> io::Result<()> {
        Ok(())
    }
}
