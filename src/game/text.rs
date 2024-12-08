use super::{GameDimension, GameObject};
use crossterm::{
    cursor::{self, MoveTo},
    style::{self, style, Stylize},
    terminal, QueueableCommand,
};
use std::io::{self, stdout, Write};

pub struct Text {
    content: String,
    outlined: bool,
    pos: Position,
    dim: GameDimension,
}

pub enum HorizontalAlign {
    Left,
    Centre,
    Right,
}

pub enum VerticalAlign {
    Top,
    Centre,
    Bottom,
}

pub struct Position(pub HorizontalAlign, pub VerticalAlign);

impl Text {
    pub fn new(content: &str, pos: Position, dim: GameDimension) -> Self {
        Self {
            content: content.to_string(),
            pos,
            dim,
            outlined: false,
        }
    }
}

impl GameObject for Text {
    fn fill_object(&self, symbol: style::StyledContent<&str>) -> io::Result<()> {
        let mut stdout = stdout();

        let length = symbol.content().len() as u16;

        // No content
        if length == 0 {
            return Ok(());
        }

        let Position(horizontal, vertical) = &self.pos;

        let x_offset = match horizontal {
            HorizontalAlign::Left => 0,
            HorizontalAlign::Centre => self.dim.1 / 2 - (length + 2) / 2,
            HorizontalAlign::Right => self.dim.1 - length - 2,
        };

        let y_offset = match vertical {
            VerticalAlign::Top => 0,
            VerticalAlign::Centre => self.dim.0 / 2 - 1,
            VerticalAlign::Bottom => self.dim.0 - 3,
        };

        // Top border
        stdout
            .queue(MoveTo(x_offset, y_offset))?
            .queue(style::PrintStyledContent(style("┌")))?;
        for i in 1..=length {
            stdout
                .queue(MoveTo(x_offset + i as u16, y_offset))?
                .queue(style::PrintStyledContent(style("─")))?;
        }
        stdout
            .queue(MoveTo(x_offset + (length + 1) as u16, y_offset))?
            .queue(style::PrintStyledContent(style("┐")))?;

        // Middle
        stdout
            .queue(MoveTo(x_offset as u16, y_offset + 1))?
            .queue(style::PrintStyledContent(style("│")))?
            .queue(MoveTo(x_offset + 1, y_offset + 1))?
            .queue(style::PrintStyledContent(symbol))?
            .queue(MoveTo(x_offset + (length + 1) as u16, y_offset + 1))?
            .queue(style::PrintStyledContent(style("│")))?;

        // Bottom border
        stdout
            .queue(MoveTo(x_offset, y_offset + 2))?
            .queue(style::PrintStyledContent(style("└")))?;
        for i in 1..=length {
            stdout
                .queue(MoveTo(x_offset + i as u16, y_offset + 2))?
                .queue(style::PrintStyledContent(style("─")))?;
        }
        stdout
            .queue(MoveTo(x_offset + (length + 1) as u16, y_offset + 2))?
            .queue(style::PrintStyledContent(style("┘")))?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_object(&self) -> io::Result<()> {
        self.fill_object(style(&self.content));
        Ok(())
    }

    fn clear_object(&self) -> io::Result<()> {
        self.fill_object(style(""));
        Ok(())
    }

    fn move_object(&mut self) -> io::Result<()> {
        Ok(())
    }
}
