use super::{
    Collidable, GameDimension, GameObject, GamePhysics, ObjectCoordinates, ObjectVelocity,
    SurfaceNormal,
};
use crossterm::{
    cursor,
    event::{self, poll, read, Event, KeyCode, KeyEvent},
    style::{self, style, Stylize},
    terminal::{self, WindowSize},
    ExecutableCommand, QueueableCommand,
};
use std::cmp::*;
use std::io::{self, stdout, Write};

pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Wall {
    normals: [SurfaceNormal; 1],
    dir: Direction,
    dim: GameDimension,
}

impl Wall {
    pub fn new(dim: GameDimension, dir: Direction) -> Self {
        match dir {
            Direction::Left => Wall {
                normals: [SurfaceNormal(1, 0)],
                dir,
                dim,
            },
            Direction::Right => Wall {
                normals: [SurfaceNormal(-1, 0)],
                dir,
                dim,
            },
            Direction::Top => Wall {
                normals: [SurfaceNormal(0, -1)],
                dir,
                dim,
            },
            Direction::Bottom => Wall {
                normals: [SurfaceNormal(0, 1)],
                dir,
                dim,
            },
        }
    }
}

impl Collidable for Wall {
    fn get_coordinates(&self) -> ObjectCoordinates {
        match self.dir {
            Direction::Left => ObjectCoordinates(0, 0, 0, self.dim.0),
            Direction::Right => ObjectCoordinates(self.dim.1, 0, self.dim.1, self.dim.0),
            Direction::Top => ObjectCoordinates(0, 0, self.dim.1, 0),
            Direction::Bottom => ObjectCoordinates(0, self.dim.0, self.dim.1, self.dim.0),
        }
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
        // For now, a wall can have only one surface
        &self.normals[0]
    }
    fn get_velocity(&self) -> super::ObjectVelocity {
        ObjectVelocity(0.0, 0.0)
    }
}

impl GameObject for Wall {
    fn fill_object(&self, symbol: style::StyledContent<&str>) -> io::Result<()> {
        let mut stdout = stdout();

        let ObjectCoordinates (x1, y1, x2, y2) = self.get_coordinates();

        for j in y1..=y2 {
            for i in x1..=x2 {
                stdout
                    .queue(cursor::MoveTo(i, j))?
                    .queue(style::PrintStyledContent(symbol))?;
            }
        }
        stdout.flush();
        Ok(())
    }

    fn draw_object(&self) -> io::Result<()> {
        match self.dir {
            Direction::Left => self.fill_object("┃".with(style::Color::Green)),
            Direction::Right => self.fill_object("┃".with(style::Color::Green)),
            Direction::Top => self.fill_object("─".with(style::Color::Green)),
            Direction::Bottom => self.fill_object("─".with(style::Color::Green)),
        }
    }

    fn clear_object(&self) -> io::Result<()> {
        self.fill_object(style(" "))
    }

    fn move_object(&mut self) -> io::Result<()> {
        Ok(())
    }
}
