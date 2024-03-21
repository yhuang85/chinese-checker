use std::process;

use crossterm::style::Color;
use game::Player;
use tui::Screen;

#[allow(dead_code)]
pub mod game;
pub mod tui;

fn main() {
    let mut cc = game::ChineseChecker::new(5);
    let players = vec![
        Player::new(String::from("Green"), Color::Green),
        Player::new(String::from("Red"), Color::Red),
        // Player::new(String::from("Blue"), Color::Blue),
        // Player::new(String::from("Yellow"), Color::Yellow),
        // Player::new(String::from("White"), Color::White),
        // Player::new(String::from("Magenta"), Color::Magenta),
    ];

    for player in players.iter() {
        cc.add_player(player).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
    }
    let mut screen = Screen::new(&cc);
    screen
        .start(&cc, &players)
        .unwrap_or_else(|err| eprintln!("{err}"));

    // The game loop
    for _ in 0..10 {
        for player in &players {
            let selected_move = player.select_move(&cc);
            if let Some(m) = selected_move {
                cc.play(player, &m);
                screen
                    .update(&player, &m)
                    .unwrap_or_else(|err| eprintln!("{err}"));
            }
        }
    }
    screen.close().unwrap();
}
