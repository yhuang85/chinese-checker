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

use crate::game::{ChineseChecker, Position};
use crate::player::{Move, Player};

#[derive(Debug)]
pub struct Screen {
    height: i16,
    origin: Position,
    pen: io::Stdout,
    speed: f32,
    pause: f32,
}

impl Screen {
    pub fn new(cc: &ChineseChecker, move_speed: f32, pause_between_players: f32) -> Screen {
        let (width, height) = terminal::size().expect("Cannot get the size of the terminal.");
        let width = width as i16;
        let height = height as i16;
        let origin = Position::from((width / 2 - 3 * cc.size, height / 2 - 2 * cc.size));
        Screen {
            height,
            origin,
            pen: io::stdout(),
            speed: move_speed,
            pause: pause_between_players,
        }
    }

    pub fn draw(&mut self, cc: &ChineseChecker, players: &Vec<Player>) -> io::Result<()> {
        self.clear()?;
        // First plot the empty board
        for position in cc.nodes.keys() {
            self.empty(position, None, false)?;
        }
        // Then plot the players
        for player in players {
            for position in cc
                .state
                .get(&player.name)
                .expect(&format!("Player {} is not found in the game.", player.name))
                .positions
                .iter()
            {
                self.fill(position, player.color, false)?;
            }
        }
        self.pen.flush()?;
        Ok(())
    }

    pub fn update(&mut self, player: &Player, mv: &Move) -> io::Result<()> {
        self.empty(&mv.from, Some(player.color), true)?;
        self.pause(self.speed / 2.0);
        self.empty(&mv.to, Some(player.color), true)?;
        self.pause(self.speed / 2.0);
        self.empty(&mv.from, None, true)?;
        self.fill(&mv.to, player.color, true)?;
        thread::sleep(Duration::from_secs_f32(self.pause));
        Ok(())
    }

    pub fn display_round(&mut self, round: u32) -> io::Result<()> {
        self.pen
            .queue(cursor::Hide)?
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::Print(format!("Round {}", round)))?
            .flush()?;
        Ok(())
    }

    pub fn display_result(&mut self, done_player_colors: &Vec<Color>) -> io::Result<()> {
        self.pen
            .queue(cursor::Hide)?
            .queue(cursor::MoveTo(0, 1))?
            .queue(style::Print("Finished in order:".to_string()))?;
        for color in done_player_colors {
            self.pen
                .queue(cursor::MoveRight(1))?
                .queue(style::PrintStyledContent("\u{2B24}".with(*color)))?;
        }
        self.pen.flush()?;
        Ok(())
    }

    pub fn pause(&self, secs: f32) {
        thread::sleep(Duration::from_secs_f32(secs));
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.reset_cursor()?;
        Ok(())
    }

    fn handle_position(
        &mut self,
        position: &Position,
        content: char,
        color: Option<Color>,
        flush: bool,
    ) -> io::Result<()> {
        let pixel = self.origin + *position;
        self.pen
            .queue(cursor::Hide)?
            .queue(cursor::MoveTo(pixel.x as u16, pixel.y as u16))?
            .queue(style::PrintStyledContent(
                content.with(color.unwrap_or_else(|| Color::Grey)),
            ))?;
        if flush {
            self.pen.flush()?;
        }
        Ok(())
    }

    pub fn fill(&mut self, position: &Position, color: Color, flush: bool) -> io::Result<()> {
        self.handle_position(position, '\u{2B24}', Some(color), flush)?;
        Ok(())
    }

    pub fn empty(
        &mut self,
        position: &Position,
        color: Option<Color>,
        flush: bool,
    ) -> io::Result<()> {
        self.handle_position(position, '\u{2B55}', color, flush)?;
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.pen
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    fn reset_cursor(&mut self) -> io::Result<()> {
        self.pen
            .queue(cursor::MoveTo(0, self.height as u16))?
            .queue(cursor::Show)?;
        Ok(())
    }
}
