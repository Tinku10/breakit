use crossterm::{
    cursor,
    style::{self, style, Stylize},
    QueueableCommand,
};

use super::{
    Collidable, GameDimension, GameObject, GamePhysics, MoveCommand, ObjectCoordinates, Vector,
};

use std::cmp::*;
use std::io::{self, stdout, Write};

pub struct Brick {
    xpos: u16,
    ypos: u16,
    normals: [Vector; 1],
    destroyed: bool,
}

impl Brick {
    pub fn new(x: u16, y: u16) -> Self {
        Brick {
            xpos: x,
            ypos: y,
            normals: [Vector(0.0, 1.0)],
            destroyed: false,
        }
    }
}

impl GameObject for Brick {
    fn fill_object(&self, symbol: style::StyledContent<&str>) -> io::Result<()> {
        let mut stdout = stdout();

        stdout
            .queue(cursor::MoveTo(self.xpos, self.ypos))?
            .queue(style::PrintStyledContent(symbol))?;

        stdout.flush()?;
        Ok(())
    }

    fn draw_object(&self) -> io::Result<()> {
        if self.destroyed {
            return Ok(());
        }
        self.fill_object("▆".magenta())
    }
    fn clear_object(&self) -> io::Result<()> {
        if self.destroyed {
            return Ok(());
        }
        self.fill_object(style(" "))
    }
    fn move_object(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Collidable for Brick {
    fn get_velocity(&self) -> Vector {
        Vector(0.0, 0.0)
    }
    fn get_normal(&self, other: &dyn Collidable) -> &Vector {
        // Only one normal for now
        &self.normals[0]
    }
    fn get_coordinates(&self) -> ObjectCoordinates {
        ObjectCoordinates(self.xpos, self.ypos, self.xpos, self.ypos)
    }
    fn has_collision(&self, other: &dyn Collidable) -> bool {
        if self.destroyed {
            return false
        }
        let ObjectCoordinates(ax1, ay1, ax2, ay2) = self.get_coordinates();
        let ObjectCoordinates(bx1, by1, bx2, by2) = other.get_coordinates();

        let x1 = max(ax1, bx1);
        let x2 = min(ax2, bx2);
        let y1 = max(ay1, by1);
        let y2 = min(ay2, by2);

        if x1 <= x2 && y1 <= y2 {
            return true;
        }
        return false;
    }
}

impl GamePhysics for Brick {
    fn update_object(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn handle_collision(&mut self, other: &dyn Collidable) -> io::Result<()> {
        if self.has_collision(other) {
            self.clear_object()?;
            self.destroyed = true;
        }
        Ok(())
    }
}

