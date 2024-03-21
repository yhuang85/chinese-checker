use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

use crate::game::{ChineseChecker, Move, Player, Position};

#[derive(Debug)]
pub struct Screen {
    height: i16,
    origin: Position,
    pen: io::Stdout,
}

impl Screen {
    pub fn new(cc: &ChineseChecker) -> Screen {
        let (width, height) = terminal::size().expect("Cannot get the size of the terminal.");
        let width = width as i16;
        let height = height as i16;
        let origin = Position::from((width / 2 - 3 * cc.size, height / 2 - 2 * cc.size));
        Screen {
            height,
            origin,
            pen: io::stdout(),
        }
    }

    pub fn start(&mut self, cc: &ChineseChecker, players: &Vec<Player>) -> io::Result<()> {
        self.clear()?;
        // First plot the empty board
        for position in cc.nodes.keys() {
            self.reset_hole(position)?;
        }
        // Then plot the players
        for player in players {
            for position in cc
                .state
                .get(&player.name)
                .expect("Player is not found in the game.")
                .positions
                .iter()
            {
                self.fill_hole(position, player.color)?;
            }
        }
        Ok(())
    }

    pub fn update(&mut self, player: &Player, next_move: &Move) -> io::Result<()> {
        self.reset_hole(&next_move.from)?;
        self.flush()?;
        thread::sleep(Duration::from_secs(1));
        self.fill_hole(&next_move.to, player.color)?;
        self.flush()?;
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
