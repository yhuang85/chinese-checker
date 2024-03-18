use std::process;

use crossterm::style::Color;
use tui::Screen;

#[allow(dead_code)]
pub mod game;
pub mod tui;

fn main() {
    let mut cc = game::ChineseChecker::new(5);
    for (name, color) in [
        ("Green", Color::Green),
        ("Red", Color::Red),
        ("Blue", Color::Blue),
        ("White", Color::White),
        ("Yellow", Color::Yellow),
        ("Magenta", Color::Magenta),
    ] {
        cc.add_player(String::from(name), color)
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
    }
    let mut screen = Screen::new(&cc);
    screen.start().unwrap();
    screen.close().unwrap();
}
