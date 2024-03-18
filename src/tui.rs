use std::io::{self, Write};

use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

use crate::game::{ChineseChecker, Position};

#[derive(Debug)]
pub struct Screen<'a> {
    height: i16,
    origin: Position,
    pen: io::Stdout,
    cc: &'a ChineseChecker,
}

impl<'a> Screen<'a> {
    pub fn new(cc: &'a ChineseChecker) -> Screen<'a> {
        let (width, height) = terminal::size().expect("Cannot get the size of the terminal.");
        let width = width as i16;
        let height = height as i16;
        let origin = Position::from((width / 2 - 3 * cc.size, height / 2 - 2 * cc.size));
        Screen {
            height,
            origin,
            pen: io::stdout(),
            cc,
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.clear()?;
        // First plot the empty board
        for position in self.cc.nodes.keys() {
            self.reset_hole(position)?;
        }
        // Then plot the players
        for player in self.cc.players.iter() {
            for position in player.positions.iter() {
                self.fill_hole(position, player.color)?;
            }
        }
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.reset_cursor()?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.pen.flush()?;
        Ok(())
    }

    fn set_hole(
        &mut self,
        position: &Position,
        content: char,
        color: Option<Color>,
    ) -> io::Result<()> {
        let pixel = self.origin + *position;
        self.pen
            .queue(cursor::MoveTo(pixel.x as u16, pixel.y as u16))?
            .queue(style::PrintStyledContent(
                content.with(color.unwrap_or_else(|| Color::Grey)),
            ))?;
        Ok(())
    }

    pub fn fill_hole(&mut self, position: &Position, color: Color) -> io::Result<()> {
        self.set_hole(position, '\u{2B24}', Some(color))?;
        Ok(())
    }

    pub fn reset_hole(&mut self, position: &Position) -> io::Result<()> {
        self.set_hole(position, '\u{2B55}', None)?;
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.pen
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    fn reset_cursor(&mut self) -> io::Result<()> {
        self.pen.queue(cursor::MoveTo(0, self.height as u16))?;
        Ok(())
    }
}
