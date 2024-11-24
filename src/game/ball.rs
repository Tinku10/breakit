use super::{
    Board, Collidable, GameDimension, GameObject, GamePhysics, MoveCommand, ObjectCoordinates,
    ObjectVelocity, SurfaceNormal,
};
use crossterm::{
    cursor,
    style::{self, style, StyledContent, Stylize},
    terminal, QueueableCommand,
};
use std::any::Any;
use std::cmp::*;
use std::io::{self, stdout, Write};

const MOVE_INCR: u16 = 2;

pub struct Ball {
    pub xpos: u16,
    pub ypos: u16,
    pub radius: u16,
    pub xvelocity: f64,
    pub yvelocity: f64,
    dim: GameDimension,
    normals: [SurfaceNormal; 4],
}

impl Ball {
    pub fn new(dim: GameDimension) -> Ball {
        const BALL_RADIUS: u16 = 0;
        Ball {
            xpos: dim.1 / 2 - BALL_RADIUS / 2,
            ypos: dim.0 - 20,
            xvelocity: 1.0,
            yvelocity: 1.0,
            radius: BALL_RADIUS,
            dim,
            normals: [
                SurfaceNormal(0, -1),
                SurfaceNormal(0, 1),
                SurfaceNormal(-1, 0),
                SurfaceNormal(1, 0),
            ],
        }
    }
}

impl Collidable for Ball {
    fn get_coordinates(&self) -> ObjectCoordinates {
        ObjectCoordinates(
            self.xpos,
            self.ypos,
            self.xpos + 2 * self.radius,
            self.ypos + 2 * self.radius,
        )
    }

    fn has_collision(&self, other: &dyn Collidable) -> bool {
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

    fn get_normal(&self, other: &dyn Collidable) -> &super::SurfaceNormal {
        todo!("do not require ball's normal yet!")
    }

    fn get_velocity(&self) -> super::ObjectVelocity {
        ObjectVelocity(self.xvelocity, self.yvelocity)
    }
}

impl GameObject for Ball {
    fn fill_object(&self, symbol: StyledContent<&str>) -> io::Result<()> {
        let mut stdout = stdout();

        const ASPECT_RATIO: f64 = 2.0;

        let xball_from = self.xpos;
        let xball_to = self.xpos + 2 * self.radius;

        let yball_from = self.ypos;
        let yball_to = self.ypos + 2 * self.radius;

        // Rows
        for i in yball_from..=yball_to {
            // Columns
            for j in xball_from..=xball_to {
                let x: f64 = (j as f64) - (self.xpos + self.radius) as f64;
                let y: f64 = ((i as f64) - (self.ypos + self.radius) as f64) * ASPECT_RATIO;

                if x * x + y * y <= (self.radius as f64 * self.radius as f64) {
                    stdout
                        .queue(cursor::MoveTo(j, i))?
                        .queue(style::PrintStyledContent(symbol))?;
                }
            }
        }

        stdout.flush()?;
        Ok(())
    }

    fn draw_object(&self) -> io::Result<()> {
        self.fill_object("●".green())
    }

    fn clear_object(&self) -> io::Result<()> {
        self.fill_object(style(" "))
    }

    fn move_object(&mut self) -> io::Result<()> {
        self.clear_object()?;
        let new_xpos =
            (self.xpos as f64 + self.xvelocity).clamp(0.0, (self.dim.1 - 2 * self.radius) as f64);
        let new_ypos =
            (self.ypos as f64 + self.yvelocity).clamp(0.0, (self.dim.0 - 2 * self.radius) as f64);
        self.xpos = new_xpos as u16;
        self.ypos = new_ypos as u16;
        Ok(())
    }
}

// impl MoveCommand for Ball {
//     fn move_left(&mut self) -> io::Result<()> {
//         self.xvelocity = -1;
//         self.move_object()
//     }

//     fn move_right(&mut self) -> io::Result<()> {
//         self.xvelocity = 1;
//         self.move_object()
//     }

//     fn move_up(&mut self) -> io::Result<()> {
//         self.yvelocity = -1;
//         self.move_object()
//     }

//     fn move_down(&mut self) -> io::Result<()> {
//         self.yvelocity = 1;
//         self.move_object()
//     }
// }

impl GamePhysics for Ball {
    fn update_object(&mut self) -> io::Result<()> {
        self.move_object();
        Ok(())
    }

    fn handle_collision(&mut self, other: &dyn Collidable) -> io::Result<()> {
        if self.has_collision(other) {
            // Retrieve the surface normal from the other object
            let SurfaceNormal(nx, ny) = other.get_normal(self);

            // They are already normal, so not needed
            let normal_magnitude = 1.0; //((nx * nx + ny * ny) as f64).sqrt();
            let nx = *nx as f64 / normal_magnitude;
            let ny = *ny as f64 / normal_magnitude;

            // Current velocity vector
            let vx = self.xvelocity as f64;
            let vy = self.yvelocity as f64;

            // Calculate the dot product of the velocity and the normal
            let dot_product = vx * nx + vy * ny;

            // Calculate the reflected velocity
            let reflected_vx = vx - 2.0 * dot_product * nx;
            let reflected_vy = vy - 2.0 * dot_product * ny;

            // Update the ball's velocity
            self.xvelocity = reflected_vx;
            self.yvelocity = reflected_vy;
        }
        Ok(())
    }
}
