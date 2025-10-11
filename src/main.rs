use crossterm::{
    cursor, event::{self, Event, KeyCode}, execute, 
    style::{Color, Print, ResetColor, SetForegroundColor}, 
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
};
use rand::Rng;
use std::{io::{stdout, Write}, thread, time::Duration};

const WIDTH: usize = 80;
const HEIGHT: usize = 30;
const PLAYER_CHAR: char = '▲'; // X-Wing mejorado
const ENEMY_CHAR: char = '◆'; // TIE Fighter mejorado
const LASER_CHAR: char = '│';
const ENEMY_LASER_CHAR: char = '┊';

// Encapsula estado del terminal para asegurar limpieza en Drop
struct TerminalGuard;
impl TerminalGuard {
    fn new() -> Self {
        terminal::enable_raw_mode().ok();
        execute!(stdout(), EnterAlternateScreen, cursor::Hide).ok();
        Self
    }
}
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        execute!(stdout(), cursor::Show, LeaveAlternateScreen).ok();
        terminal::disable_raw_mode().ok();
    }
}

struct Explosion {
    x: usize,
    y: usize,
    lifetime: u8,
}

struct Game {
    player_x: usize,
    player_lives: u32,
    enemies: Vec<(usize, usize)>,
    lasers: Vec<(usize, usize)>,
    enemy_lasers: Vec<(usize, usize)>,
    explosions: Vec<Explosion>,
    score: u32,
    high_score: u32,
    enemy_dir: isize,
    tick: u64,
    step_interval: u32,
    wave: u32,
    enemies_destroyed: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            player_x: WIDTH / 2,
            player_lives: 3,
            enemies: Vec::new(),
            lasers: Vec::new(),
            enemy_lasers: Vec::new(),
            explosions: Vec::new(),
            score: 0,
            high_score: 0,
            enemy_dir: 1,
            tick: 0,
            step_interval: 10,
            wave: 0,
            enemies_destroyed: 0,
        }
    }

    fn spawn_wave(&mut self) {
        self.wave += 1;
        self.enemies.clear();
        let rows = ((5 + self.wave / 2).min(8)) as usize;
        let cols = 10usize;
        for i in 0..rows {
            for j in 0..cols {
                let x = j * 7 + 8;
                let y = i * 2 + 3;
                if x < WIDTH - 5 && y < HEIGHT - 10 {
                    self.enemies.push((x, y));
                }
            }
        }
        self.step_interval = (10u32.saturating_sub(self.wave / 2)).max(3);
        self.enemy_dir = 1;
    }

    fn draw(&self) {
        let mut screen = vec![vec![' '; WIDTH]; HEIGHT];
        
        // Dibujar borde superior
        for x in 0..WIDTH {
            screen[0][x] = '═';
            if HEIGHT >= 4 {
                screen[HEIGHT - 4][x] = '═';
            }
        }
        
        // Dibujar jugador (nave más elaborada)
        if self.player_x >= 1 && self.player_x < WIDTH - 1 && HEIGHT >= 7 {
            screen[HEIGHT - 6][self.player_x] = PLAYER_CHAR;
            if HEIGHT >= 8 {
                screen[HEIGHT - 7][self.player_x - 1] = '◀';
                screen[HEIGHT - 7][self.player_x + 1] = '▶';
            }
        }
        
        // Dibujar enemigos
        for &(x, y) in &self.enemies {
            if y < HEIGHT.saturating_sub(4) && x < WIDTH {
                screen[y][x] = ENEMY_CHAR;
            }
        }
        
        // Dibujar láseres del jugador
        for &(x, y) in &self.lasers {
            if y > 0 && y < HEIGHT.saturating_sub(4) && x < WIDTH {
                screen[y][x] = LASER_CHAR;
            }
        }
        
        // Dibujar láseres enemigos
        for &(x, y) in &self.enemy_lasers {
            if y > 0 && y < HEIGHT.saturating_sub(4) && x < WIDTH {
                screen[y][x] = ENEMY_LASER_CHAR;
            }
        }
        
        // Dibujar explosiones
        for exp in &self.explosions {
            if exp.y < HEIGHT.saturating_sub(4) && exp.x < WIDTH {
                let ch = match exp.lifetime {
                    6..=7 => '*',
                    4..=5 => '+',
                    2..=3 => '·',
                    _ => ' ',
                };
                screen[exp.y][exp.x] = ch;
            }
        }
        
        // Renderizar pantalla - Optimizado sin Clear
        let mut out = stdout();
        execute!(
            out,
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Cyan)
        )
        .unwrap();
        
        // Línea superior con título
        print!("╔{}╗", "═".repeat(WIDTH - 2));
        execute!(out, cursor::MoveToNextLine(1)).unwrap();
        print!("║");
        execute!(out, SetForegroundColor(Color::Yellow)).unwrap();
        print!("{:^width$}", "★ STAR WARS: TERMINAL ASSAULT ★", width = WIDTH - 2);
        execute!(out, SetForegroundColor(Color::Cyan)).unwrap();
        print!("║");
        execute!(out, cursor::MoveToNextLine(1)).unwrap();
        print!("╚{}╝", "═".repeat(WIDTH - 2));
        execute!(out, cursor::MoveToNextLine(1), ResetColor).unwrap();
        
        // Contenido del juego
        for (y, row) in screen.iter().enumerate() {
            if y == 0 || y == HEIGHT - 4 {
                execute!(out, SetForegroundColor(Color::DarkGrey)).unwrap();
            } else {
                execute!(out, ResetColor).unwrap();
            }
            
            for &ch in row {
                match ch {
                    PLAYER_CHAR => {
                        execute!(out, SetForegroundColor(Color::Green)).unwrap();
                        print!("{}", ch);
                        execute!(out, ResetColor).unwrap();
                    }
                    '◀' | '▶' => {
                        execute!(out, SetForegroundColor(Color::DarkGreen)).unwrap();
                        print!("{}", ch);
                        execute!(out, ResetColor).unwrap();
                    }
                    ENEMY_CHAR => {
                        execute!(out, SetForegroundColor(Color::Red)).unwrap();
                        print!("{}", ch);
                        execute!(out, ResetColor).unwrap();
                    }
                    LASER_CHAR => {
                        execute!(out, SetForegroundColor(Color::Cyan)).unwrap();
                        print!("{}", ch);
                        execute!(out, ResetColor).unwrap();
                    }
                    ENEMY_LASER_CHAR => {
                        execute!(out, SetForegroundColor(Color::Red)).unwrap();
                        print!("{}", ch);
                        execute!(out, ResetColor).unwrap();
                    }
                    '*' | '+' | '·' => {
                        execute!(out, SetForegroundColor(Color::Yellow)).unwrap();
                        print!("{}", ch);
                        execute!(out, ResetColor).unwrap();
                    }
                    _ => print!("{}", ch),
                }
            }
            execute!(out, cursor::MoveToNextLine(1)).unwrap();
        }
        
        // HUD mejorado
        execute!(out, SetForegroundColor(Color::DarkGrey)).unwrap();
        print!("{}", "═".repeat(WIDTH));
        execute!(out, cursor::MoveToNextLine(1), ResetColor).unwrap();
        
        // Barra de vidas
        execute!(out, SetForegroundColor(Color::Green)).unwrap();
        print!("♥ Lives: ");
        for _ in 0..self.player_lives {
            print!("█");
        }
        for _ in self.player_lives..3 {
            execute!(out, SetForegroundColor(Color::DarkGrey)).unwrap();
            print!("░");
        }
        execute!(out, ResetColor).unwrap();
        
        // Score y Wave
        execute!(out, SetForegroundColor(Color::Yellow)).unwrap();
        print!("  │  Score: {:06}", self.score);
        execute!(out, SetForegroundColor(Color::Magenta)).unwrap();
        print!("  │  Wave: {:02}", self.wave);
        execute!(out, SetForegroundColor(Color::Cyan)).unwrap();
        print!("  │  High: {:06}", self.high_score);
        execute!(out, cursor::MoveToNextLine(1), ResetColor).unwrap();
        
        // Barra de progreso de enemigos
        let rows = ((5 + self.wave / 2).min(8)) as usize;
        let total_enemies = rows * 10;
        let remaining = self.enemies.len();
        let bar_width = WIDTH.saturating_sub(20);
        let filled = if total_enemies > 0 {
            ((total_enemies.saturating_sub(remaining)) * bar_width) / total_enemies
        } else {
            0
        };
        
        execute!(out, SetForegroundColor(Color::DarkGrey)).unwrap();
        print!("Enemies: [");
        execute!(out, SetForegroundColor(Color::Red)).unwrap();
        print!("{}", "█".repeat(filled));
        execute!(out, SetForegroundColor(Color::DarkGrey)).unwrap();
        print!("{}", "░".repeat(bar_width.saturating_sub(filled)));
        print!("] {}/{}", total_enemies.saturating_sub(remaining), total_enemies);
        execute!(out, ResetColor).unwrap();
        
        out.flush().unwrap();
    }

    // Devuelve false si hay game over
    fn update(&mut self) -> bool {
        // Mover y actualizar explosiones
        self.explosions.retain_mut(|exp| {
            exp.lifetime = exp.lifetime.saturating_sub(1);
            exp.lifetime > 0
        });
        
        // Mover láseres del jugador
        for (_, y) in &mut self.lasers {
            *y = y.saturating_sub(1);
        }
        self.lasers.retain(|&(_, y)| y > 0);
        
        // Mover láseres enemigos
        for (_, y) in &mut self.enemy_lasers {
            if *y < HEIGHT.saturating_sub(5) {
                *y += 1;
            }
        }
        self.enemy_lasers.retain(|&(_, y)| y < HEIGHT.saturating_sub(5));
        
        // Enemigos disparan ocasionalmente
        if !self.enemies.is_empty() && rand::random::<u8>() % 40 == 0 {
            let idx = rand::thread_rng().gen_range(0..self.enemies.len());
            let (ex, ey) = self.enemies[idx];
            self.enemy_lasers.push((ex, ey + 1));
        }
        
        // Movimiento de enemigos
        self.tick += 1;
        if self.tick % self.step_interval as u64 == 0 {
            let hit_edge = self.enemies.iter().any(|&(x, _)| {
                (self.enemy_dir < 0 && x <= 2) || (self.enemy_dir > 0 && x >= WIDTH - 3)
            });
            if hit_edge {
                for (_, y) in &mut self.enemies {
                    *y += 1;
                }
                self.enemy_dir = -self.enemy_dir;
            } else {
                for (x, _) in &mut self.enemies {
                    let nx = (*x as isize + self.enemy_dir).clamp(1, (WIDTH - 2) as isize) as usize;
                    *x = nx;
                }
            }
        }
        
        // Colisiones de láseres del jugador con enemigos
        let mut kept_lasers = Vec::new();
        for (lx, ly) in self.lasers.drain(..) {
            if let Some(idx) = self.enemies.iter().position(|&(ex, ey)| ex == lx && ey == ly) {
                self.enemies.swap_remove(idx);
                self.score += 10;
                self.enemies_destroyed += 1;
                self.explosions.push(Explosion { x: lx, y: ly, lifetime: 8 });
            } else {
                kept_lasers.push((lx, ly));
            }
        }
        self.lasers = kept_lasers;
        
        // Colisiones de láseres enemigos con jugador
        if HEIGHT >= 7 {
            for &(lx, ly) in &self.enemy_lasers {
                if ly >= HEIGHT.saturating_sub(7) && ly <= HEIGHT.saturating_sub(6) && 
                   lx >= self.player_x.saturating_sub(1) && lx <= self.player_x.saturating_add(1) {
                    self.player_lives = self.player_lives.saturating_sub(1);
                    if HEIGHT >= 6 {
                        self.explosions.push(Explosion { x: self.player_x, y: HEIGHT - 6, lifetime: 10 });
                    }
                    self.enemy_lasers.clear();
                    thread::sleep(Duration::from_millis(500));
                    if self.player_lives == 0 {
                        return false;
                    }
                    break;
                }
            }
        }
        
        // Game over si enemigos llegan al fondo
        if HEIGHT >= 7 && self.enemies.iter().any(|&(_, y)| y >= HEIGHT.saturating_sub(7)) {
            self.player_lives = 0;
            return false;
        }
        
        // Nueva oleada
        if self.enemies.is_empty() {
            thread::sleep(Duration::from_millis(1000));
            self.spawn_wave();
        }
        
        true
    }
}

fn star_wars_intro() {
    let mut out = stdout();
    
    // Mensaje inicial
    execute!(
        out,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Blue)
    )
    .ok();
    
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
        thread::sleep(Duration::from_millis(35));
    }
    thread::sleep(Duration::from_millis(1500));
    
    // Título ASCII art
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

fn show_menu() -> bool {
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
                KeyCode::Char('s') | KeyCode::Char('S') => return true,
                KeyCode::Char('h') | KeyCode::Char('H') => {
                    show_help();
                    return show_menu();
                }
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return false,
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

fn show_game_over(game: &Game) {
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

fn main() {
    let _term = TerminalGuard::new();
    
    star_wars_intro();
    
    let mut high_score = 0u32;
    
    loop {
        if !show_menu() {
            break;
        }
        
        let mut game = Game::new();
        game.high_score = high_score;
        game.spawn_wave();
        
        // Limpiar una sola vez antes del juego
        execute!(stdout(), terminal::Clear(ClearType::All)).ok();
        
        loop {
            game.draw();
            if !game.update() {
                if game.score > high_score {
                    high_score = game.score;
                    game.high_score = high_score;
                }
                break;
            }
            thread::sleep(Duration::from_millis(50));
            
            if event::poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Left => {
                            if game.player_x > 2 {
                                game.player_x -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if game.player_x < WIDTH.saturating_sub(3) {
                                game.player_x += 1;
                            }
                        }
                        KeyCode::Char(' ') => {
                            if HEIGHT >= 8 {
                                game.lasers.push((game.player_x, HEIGHT - 8));
                            }
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                            if game.score > high_score {
                                high_score = game.score;
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        
        show_game_over(&game);
    }
}
