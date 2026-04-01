use crossterm::{cursor, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use std::io::stdout;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> Self {
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
