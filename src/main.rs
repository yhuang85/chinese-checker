use crossterm::style::Color;
use player::Player;
use tui::Screen;

#[allow(dead_code)]
pub mod game;
pub mod player;
pub mod tui;

fn main() {
    let mut cc = game::ChineseChecker::new(4);
    let players = vec![
        Player::new(String::from("Green"), Color::Green, 0),
        Player::new(String::from("Red"), Color::Red, 1),
        Player::new(String::from("Blue"), Color::Blue, 2),
        Player::new(String::from("Yellow"), Color::Yellow, 3),
        Player::new(String::from("White"), Color::White, 4),
        Player::new(String::from("Magenta"), Color::Magenta, 5),
    ];

    for player in players.iter() {
        cc.add_player(player);
    }
    let mut screen = Screen::new(&cc, 1e-2, 1e-2);
    screen.draw(&cc, &players).unwrap();
    screen.pause(1.0);

    // The game loop
    let mut done_player_colors: Vec<Color> = vec![];
    for round in 1..=100 {
        screen.display_round(round).unwrap();
        for player in &players {
            if !player.is_done(&cc) {
                let selected_move = player.select_move(&cc);
                if let Some(m) = selected_move {
                    cc.make_move(player, &m);
                    screen.update(&player, &m).unwrap();
                }
            } else if !done_player_colors.contains(&player.color) {
                done_player_colors.push(player.color);
            }
        }
        if !done_player_colors.is_empty() {
            screen.display_result(&done_player_colors).unwrap();
        }
        if done_player_colors.len() == players.len() {
            break;
        }
    }
    screen.close().unwrap();
}
