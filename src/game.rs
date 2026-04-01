use rand::Rng;
use std::time::{Duration, Instant};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 30;
pub const PLAYER_CHAR: char = '▲';
pub const ENEMY_CHAR: char = '◆';
pub const LASER_CHAR: char = '│';
pub const ENEMY_LASER_CHAR: char = '┊';
pub const SHOT_COOLDOWN_MS: u128 = 200;

pub struct Explosion {
    pub x: usize,
    pub y: usize,
    pub lifetime: u8,
}

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    HitStun { until: Instant },
    WaveTransition { until: Instant },
    GameOver,
}

pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Shoot,
    Quit,
}

pub struct Game {
    pub player_x: usize,
    pub player_lives: u32,
    pub enemies: Vec<(usize, usize)>,
    pub lasers: Vec<(usize, usize)>,
    pub enemy_lasers: Vec<(usize, usize)>,
    pub explosions: Vec<Explosion>,
    pub score: u32,
    pub high_score: u32,
    pub enemy_dir: isize,
    pub tick: u64,
    pub step_interval: u32,
    pub wave: u32,
    pub enemies_destroyed: u32,
    pub last_shot: Instant,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
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
            last_shot: Instant::now() - Duration::from_secs(1),
            state: GameState::Playing,
        }
    }

    pub fn spawn_wave(&mut self) {
        self.wave += 1;
        self.enemies.clear();
        let rows = self.wave_rows();
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

    pub fn wave_rows(&self) -> usize {
        ((5 + self.wave / 2).min(8)) as usize
    }

    pub fn total_enemies_in_wave(&self) -> usize {
        self.wave_rows() * 10
    }

    pub fn handle_action(&mut self, action: PlayerAction) -> bool {
        match action {
            PlayerAction::MoveLeft => {
                if self.player_x > 2 {
                    self.player_x -= 1;
                }
            }
            PlayerAction::MoveRight => {
                if self.player_x < WIDTH.saturating_sub(3) {
                    self.player_x += 1;
                }
            }
            PlayerAction::Shoot => self.shoot(),
            PlayerAction::Quit => return false,
        }
        true
    }

    fn shoot(&mut self) {
        if self.last_shot.elapsed().as_millis() >= SHOT_COOLDOWN_MS && HEIGHT >= 8 {
            self.lasers.push((self.player_x, HEIGHT - 8));
            self.last_shot = Instant::now();
        }
    }

    /// Devuelve false si hay game over.
    pub fn update(&mut self) -> bool {
        let now = Instant::now();

        match &self.state {
            GameState::HitStun { until } => {
                if now >= *until {
                    if self.player_lives == 0 {
                        self.state = GameState::GameOver;
                        return false;
                    }
                    self.state = GameState::Playing;
                }
                return true;
            }
            GameState::WaveTransition { until } => {
                if now >= *until {
                    self.spawn_wave();
                    self.state = GameState::Playing;
                }
                return true;
            }
            GameState::GameOver => return false,
            GameState::Playing => {}
        }

        self.update_explosions();
        self.update_player_lasers();
        self.update_enemy_lasers();
        self.enemy_fire();
        self.move_enemies();

        if !self.check_player_laser_hits() {
            return false;
        }
        if !self.check_enemy_laser_hits(now) {
            return true; // still alive but in hit stun
        }
        if !self.check_enemies_reached_bottom() {
            return false;
        }

        if self.enemies.is_empty() {
            self.state = GameState::WaveTransition {
                until: now + Duration::from_millis(1000),
            };
        }

        true
    }

    fn update_explosions(&mut self) {
        self.explosions.retain_mut(|exp| {
            exp.lifetime = exp.lifetime.saturating_sub(1);
            exp.lifetime > 0
        });
    }

    fn update_player_lasers(&mut self) {
        for (_, y) in &mut self.lasers {
            *y = y.saturating_sub(1);
        }
        self.lasers.retain(|&(_, y)| y > 0);
    }

    fn update_enemy_lasers(&mut self) {
        for (_, y) in &mut self.enemy_lasers {
            if *y < HEIGHT.saturating_sub(5) {
                *y += 1;
            }
        }
        self.enemy_lasers.retain(|&(_, y)| y < HEIGHT.saturating_sub(5));
    }

    fn enemy_fire(&mut self) {
        if !self.enemies.is_empty() && rand::random::<u8>() % 40 == 0 {
            let idx = rand::thread_rng().gen_range(0..self.enemies.len());
            let (ex, ey) = self.enemies[idx];
            self.enemy_lasers.push((ex, ey + 1));
        }
    }

    fn move_enemies(&mut self) {
        self.tick += 1;
        if self.tick % self.step_interval as u64 != 0 {
            return;
        }

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
                *x = (*x as isize + self.enemy_dir).clamp(1, (WIDTH - 2) as isize) as usize;
            }
        }
    }

    fn check_player_laser_hits(&mut self) -> bool {
        let mut kept_lasers = Vec::new();
        for (lx, ly) in self.lasers.drain(..) {
            let hit = self.enemies.iter().position(|&(ex, ey)| {
                ex == lx && (ey == ly || ey == ly + 1)
            });
            if let Some(idx) = hit {
                let (ex, ey) = self.enemies[idx];
                self.enemies.swap_remove(idx);
                self.score += 10;
                self.enemies_destroyed += 1;
                self.explosions.push(Explosion { x: ex, y: ey, lifetime: 8 });
            } else {
                kept_lasers.push((lx, ly));
            }
        }
        self.lasers = kept_lasers;
        true
    }

    /// Devuelve false si el jugador fue golpeado (entra en HitStun).
    fn check_enemy_laser_hits(&mut self, now: Instant) -> bool {
        if HEIGHT < 7 {
            return true;
        }

        let hit = self.enemy_lasers.iter().any(|&(lx, ly)| {
            ly >= HEIGHT.saturating_sub(7) && ly <= HEIGHT.saturating_sub(6)
                && lx >= self.player_x.saturating_sub(1)
                && lx <= self.player_x.saturating_add(1)
        });

        if hit {
            self.player_lives = self.player_lives.saturating_sub(1);
            if HEIGHT >= 6 {
                self.explosions.push(Explosion { x: self.player_x, y: HEIGHT - 6, lifetime: 10 });
            }
            self.enemy_lasers.clear();
            self.state = GameState::HitStun {
                until: now + Duration::from_millis(500),
            };
            return false;
        }

        true
    }

    fn check_enemies_reached_bottom(&self) -> bool {
        if HEIGHT >= 7 && self.enemies.iter().any(|&(_, y)| y >= HEIGHT.saturating_sub(7)) {
            return false;
        }
        true
    }
}
