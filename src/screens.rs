use crossterm::{
    cursor, event::{self, Event, KeyCode}, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::{io::{stdout, Write}, thread, time::Duration};

use crate::game::{Game, WIDTH, HEIGHT};

pub fn star_wars_intro() {
    let mut out = stdout();

    execute!(
        out,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Blue)
    )
    .ok();

    // "A long time ago..."
    let intro = "A long time ago in a galaxy far, far away...";
    for i in 0..=intro.len() {
        execute!(
            out,
            cursor::MoveTo(
                ((WIDTH.saturating_sub(intro.len())) / 2) as u16,
                (HEIGHT / 2) as u16
            ),
            Print(&intro[..i])
        )
        .ok();
        out.flush().ok();
        thread::sleep(Duration::from_millis(35));
    }
    thread::sleep(Duration::from_millis(1500));

    // Titulo ASCII art
    execute!(out, terminal::Clear(ClearType::All), SetForegroundColor(Color::Yellow)).ok();
    let title_art = vec![
        "   _____ _______    _____   ",
        "  / ____|__   __|  /  __ \\  ",
        " | (___    | |    | |__| |  ",
        "  \\___ \\   | |    |  __  |  ",
        "  ____) |  | |    | |  | |  ",
        " |_____/   |_|    |_|  |_|  ",
        "                            ",
        " ██╗    ██╗ █████╗ ██████╗ ███████╗",
        " ██║    ██║██╔══██╗██╔══██╗██╔════╝",
        " ██║ █╗ ██║███████║██████╔╝███████╗",
        " ██║███╗██║██╔══██║██╔══██╗╚════██║",
        " ╚███╔███╔╝██║  ██║██║  ██║███████║",
        "  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝",
    ];

    let start_y = (HEIGHT / 2) as u16 - (title_art.len() / 2) as u16;
    for (i, line) in title_art.iter().enumerate() {
        execute!(
            out,
            cursor::MoveTo(((WIDTH - line.len()) / 2) as u16, start_y + i as u16),
            Print(line)
        )
        .ok();
        out.flush().ok();
        thread::sleep(Duration::from_millis(50));
    }
    thread::sleep(Duration::from_millis(1200));

    // Crawl
    execute!(out, SetForegroundColor(Color::Cyan)).ok();
    let crawl = vec![
        "",
        "Episode Rust",
        "THE TERMINAL MENACE",
        "",
        "The Galactic Empire has deployed",
        "squadrons of TIE fighters to",
        "destroy the Rebel Alliance.",
        "",
        "As a skilled X-Wing pilot,",
        "you must defend the base",
        "and eliminate all threats...",
        "",
        "",
        "Press S to join the fight",
    ];

    let mut start_y = HEIGHT as i32;
    while start_y + crawl.len() as i32 >= -2 {
        execute!(out, terminal::Clear(ClearType::All)).ok();
        for (i, line) in crawl.iter().enumerate() {
            let y = start_y + i as i32;
            if y >= 0 && (y as usize) < HEIGHT {
                let x = (WIDTH.saturating_sub(line.len())) / 2;
                execute!(out, cursor::MoveTo(x as u16, y as u16), Print(line)).ok();
            }
        }
        out.flush().ok();

        if event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Event::Key(k) = event::read().unwrap() {
                if let KeyCode::Char('s') | KeyCode::Char('S') = k.code {
                    break;
                }
            }
        }
        start_y -= 1;
        thread::sleep(Duration::from_millis(100));
    }
    execute!(out, ResetColor).ok();
}

pub enum MenuChoice {
    Start,
    Quit,
}

pub fn show_menu() -> MenuChoice {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    let menu = vec![
        "╔══════════════════════════════════════════════════════════════════════════════╗",
        "║                                                                              ║",
        "║                        ★  MAIN COMMAND CENTER  ★                            ║",
        "║                                                                              ║",
        "║                           ▲                                                 ║",
        "║                          ◀▲▶     X-WING FIGHTER                             ║",
        "║                                                                              ║",
        "║                                                                              ║",
        "║                        [S] Start Mission                                    ║",
        "║                        [H] How to Play                                      ║",
        "║                        [Q] Quit                                             ║",
        "║                                                                              ║",
        "║                                                                              ║",
        "║                     May the Force be with you...                            ║",
        "║                                                                              ║",
        "╚══════════════════════════════════════════════════════════════════════════════╝",
    ];

    for (i, line) in menu.iter().enumerate() {
        execute!(
            stdout(),
            cursor::MoveTo(0, i as u16),
            SetForegroundColor(if i == 0 || i == menu.len() - 1 {
                Color::Cyan
            } else {
                Color::White
            }),
            Print(line)
        )
        .ok();
    }
    execute!(stdout(), ResetColor).ok();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('s') | KeyCode::Char('S') => return MenuChoice::Start,
                KeyCode::Char('h') | KeyCode::Char('H') => {
                    show_help();
                    return show_menu();
                }
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return MenuChoice::Quit,
                _ => {}
            }
        }
    }
}

fn show_help() {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    let help = vec![
        "╔══════════════════════════════════════════════════════════════════════════════╗",
        "║                            HOW TO PLAY                                       ║",
        "╠══════════════════════════════════════════════════════════════════════════════╣",
        "║                                                                              ║",
        "║  OBJECTIVE: Destroy all TIE Fighters before they reach your base!           ║",
        "║                                                                              ║",
        "║  CONTROLS:                                                                   ║",
        "║    ◄ ► Arrow Keys  - Move your X-Wing left/right                           ║",
        "║    SPACE           - Fire laser cannons                                     ║",
        "║    Q / ESC         - Pause and quit                                         ║",
        "║                                                                              ║",
        "║  SCORING:                                                                    ║",
        "║    • Each TIE Fighter destroyed: 10 points                                  ║",
        "║    • Complete waves for progressive difficulty                              ║",
        "║    • You have 3 lives - avoid enemy fire!                                   ║",
        "║                                                                              ║",
        "║  TIPS:                                                                       ║",
        "║    • Enemies shoot back - stay mobile!                                      ║",
        "║    • Higher waves = faster enemies + more ships                             ║",
        "║    • Don't let them reach the bottom!                                       ║",
        "║                                                                              ║",
        "║                                                                              ║",
        "║                    Press any key to return...                               ║",
        "║                                                                              ║",
        "╚══════════════════════════════════════════════════════════════════════════════╝",
    ];

    for (i, line) in help.iter().enumerate() {
        execute!(
            stdout(),
            cursor::MoveTo(0, i as u16),
            SetForegroundColor(Color::Green),
            Print(line)
        )
        .ok();
    }
    execute!(stdout(), ResetColor).ok();

    let _ = event::read();
}

pub fn show_game_over(game: &Game) {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    let final_score = format!("║    Final Score:         {:>6}                                          ║", game.score);
    let waves_completed = format!("║    Waves Completed:     {:>6}                                          ║", game.wave.saturating_sub(1));
    let enemies_destroyed = format!("║    Enemies Destroyed:   {:>6}                                          ║", game.enemies_destroyed);
    let high_score_line = format!("║    High Score:          {:>6}                                          ║", game.high_score);

    let game_over = vec![
        "╔══════════════════════════════════════════════════════════════════════════════╗",
        "║                                                                              ║",
        "║                          ▓▓▓  MISSION FAILED  ▓▓▓                           ║",
        "║                                                                              ║",
        "║                          The Empire has won...                              ║",
        "║                                                                              ║",
        "╠══════════════════════════════════════════════════════════════════════════════╣",
        "║                                                                              ║",
        "║                          MISSION STATISTICS                                 ║",
        "║                                                                              ║",
        &final_score,
        &waves_completed,
        &enemies_destroyed,
        &high_score_line,
        "║                                                                              ║",
        "║                                                                              ║",
        "║                    Press any key to continue...                             ║",
        "║                                                                              ║",
        "╚══════════════════════════════════════════════════════════════════════════════╝",
    ];

    let start_y = if HEIGHT > game_over.len() {
        (HEIGHT / 2).saturating_sub(game_over.len() / 2)
    } else {
        0
    };

    for (i, line) in game_over.iter().enumerate() {
        let y = start_y + i;
        if y < HEIGHT {
            execute!(
                stdout(),
                cursor::MoveTo(0, y as u16),
                SetForegroundColor(if i < 6 { Color::Red } else { Color::Yellow }),
                Print(line)
            )
            .ok();
        }
    }
    execute!(stdout(), ResetColor).ok();

    let _ = event::read();
}
