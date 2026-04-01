use crossterm::{cursor, execute, style::Color};
use std::io::{stdout, Write};

use crate::game::{
    Game, HEIGHT, WIDTH, ENEMY_CHAR, ENEMY_LASER_CHAR, LASER_CHAR, PLAYER_CHAR,
};

pub fn draw_game(game: &Game) {
    let mut screen = vec![vec![(' ', Color::Reset); WIDTH]; HEIGHT];

    // Bordes del area de juego
    for x in 0..WIDTH {
        screen[0][x] = ('═', Color::DarkGrey);
        if HEIGHT >= 4 {
            screen[HEIGHT - 4][x] = ('═', Color::DarkGrey);
        }
    }

    // Jugador
    if game.player_x >= 1 && game.player_x < WIDTH - 1 && HEIGHT >= 7 {
        screen[HEIGHT - 6][game.player_x] = (PLAYER_CHAR, Color::Green);
        if HEIGHT >= 8 {
            screen[HEIGHT - 7][game.player_x - 1] = ('◀', Color::DarkGreen);
            screen[HEIGHT - 7][game.player_x + 1] = ('▶', Color::DarkGreen);
        }
    }

    // Enemigos
    for &(x, y) in &game.enemies {
        if y < HEIGHT.saturating_sub(4) && x < WIDTH {
            screen[y][x] = (ENEMY_CHAR, Color::Red);
        }
    }

    // Laseres del jugador
    for &(x, y) in &game.lasers {
        if y > 0 && y < HEIGHT.saturating_sub(4) && x < WIDTH {
            screen[y][x] = (LASER_CHAR, Color::Cyan);
        }
    }

    // Laseres enemigos
    for &(x, y) in &game.enemy_lasers {
        if y > 0 && y < HEIGHT.saturating_sub(4) && x < WIDTH {
            screen[y][x] = (ENEMY_LASER_CHAR, Color::Red);
        }
    }

    // Explosiones
    for exp in &game.explosions {
        if exp.y < HEIGHT.saturating_sub(4) && exp.x < WIDTH {
            let ch = match exp.lifetime {
                6..=7 => '*',
                4..=5 => '+',
                2..=3 => '·',
                _ => ' ',
            };
            screen[exp.y][exp.x] = (ch, Color::Yellow);
        }
    }

    // Construir buffer
    let mut buf = String::with_capacity(WIDTH * HEIGHT * 4);
    let mut last_color = Color::Reset;

    // Cabecera
    push_color(&mut buf, Color::Cyan, &mut last_color);
    buf.push_str(&format!("╔{}╗\r\n", "═".repeat(WIDTH - 2)));
    buf.push('║');
    push_color(&mut buf, Color::Yellow, &mut last_color);
    buf.push_str(&format!("{:^width$}", "★ STAR WARS: TERMINAL ASSAULT ★", width = WIDTH - 2));
    push_color(&mut buf, Color::Cyan, &mut last_color);
    buf.push_str("║\r\n");
    buf.push_str(&format!("╚{}╝\r\n", "═".repeat(WIDTH - 2)));

    // Area de juego
    for row in &screen {
        for &(ch, color) in row {
            if color != Color::Reset {
                push_color(&mut buf, color, &mut last_color);
                buf.push(ch);
                push_color(&mut buf, Color::Reset, &mut last_color);
            } else {
                if last_color != Color::Reset {
                    push_color(&mut buf, Color::Reset, &mut last_color);
                }
                buf.push(ch);
            }
        }
        buf.push_str("\r\n");
    }

    // HUD
    draw_hud(&mut buf, game, &mut last_color);

    push_color(&mut buf, Color::Reset, &mut last_color);

    // Flush todo de una vez
    let mut out = stdout();
    execute!(out, cursor::MoveTo(0, 0)).unwrap();
    out.write_all(buf.as_bytes()).unwrap();
    out.flush().unwrap();
}

fn draw_hud(buf: &mut String, game: &Game, last_color: &mut Color) {
    push_color(buf, Color::DarkGrey, last_color);
    buf.push_str(&"═".repeat(WIDTH));
    buf.push_str("\r\n");

    // Vidas
    push_color(buf, Color::Green, last_color);
    buf.push_str("♥ Lives: ");
    for _ in 0..game.player_lives {
        buf.push('█');
    }
    push_color(buf, Color::DarkGrey, last_color);
    for _ in game.player_lives..3 {
        buf.push('░');
    }

    // Score, Wave, High Score
    push_color(buf, Color::Yellow, last_color);
    buf.push_str(&format!("  │  Score: {:06}", game.score));
    push_color(buf, Color::Magenta, last_color);
    buf.push_str(&format!("  │  Wave: {:02}", game.wave));
    push_color(buf, Color::Cyan, last_color);
    buf.push_str(&format!("  │  High: {:06}", game.high_score));
    buf.push_str("\r\n");

    // Barra de progreso
    let total_enemies = game.total_enemies_in_wave();
    let remaining = game.enemies.len();
    let bar_width = WIDTH.saturating_sub(20);
    let filled = if total_enemies > 0 {
        ((total_enemies.saturating_sub(remaining)) * bar_width) / total_enemies
    } else {
        0
    };

    push_color(buf, Color::DarkGrey, last_color);
    buf.push_str("Enemies: [");
    push_color(buf, Color::Red, last_color);
    buf.push_str(&"█".repeat(filled));
    push_color(buf, Color::DarkGrey, last_color);
    buf.push_str(&"░".repeat(bar_width.saturating_sub(filled)));
    buf.push_str(&format!("] {}/{}", total_enemies.saturating_sub(remaining), total_enemies));
}

fn push_color(buf: &mut String, color: Color, last: &mut Color) {
    if *last == color {
        return;
    }
    *last = color;
    if color == Color::Reset {
        buf.push_str("\x1b[0m");
        return;
    }
    let code = match color {
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::White => "37",
        Color::DarkGreen => "32;2",
        Color::DarkGrey => "90",
        _ => "0",
    };
    buf.push_str("\x1b[");
    buf.push_str(code);
    buf.push('m');
}
