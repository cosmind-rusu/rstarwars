mod game;
mod input;
mod renderer;
mod screens;
mod term_guard;

use crossterm::{execute, terminal::ClearType};
use std::{io::stdout, thread, time::Duration};

use game::Game;
use screens::MenuChoice;

fn main() {
    let _term = term_guard::TerminalGuard::new();

    screens::star_wars_intro();

    let mut high_score = 0u32;

    loop {
        match screens::show_menu() {
            MenuChoice::Quit => break,
            MenuChoice::Start => {}
        }

        let mut game = Game::new();
        game.high_score = high_score;
        game.spawn_wave();

        execute!(stdout(), crossterm::terminal::Clear(ClearType::All)).ok();

        let mut running = true;
        while running {
            renderer::draw_game(&game);

            if !game.update() {
                if game.score > high_score {
                    high_score = game.score;
                    game.high_score = high_score;
                }
                break;
            }

            thread::sleep(Duration::from_millis(50));

            if let Some(action) = input::poll_action() {
                if !game.handle_action(action) {
                    if game.score > high_score {
                        high_score = game.score;
                    }
                    running = false;
                }
            }
        }

        screens::show_game_over(&game);
    }
}
