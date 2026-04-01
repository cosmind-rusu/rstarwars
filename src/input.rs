use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::game::PlayerAction;

/// Lee input del teclado sin bloquear. Devuelve None si no hay tecla presionada.
pub fn poll_action() -> Option<PlayerAction> {
    if !event::poll(Duration::from_millis(0)).unwrap_or(false) {
        return None;
    }

    if let Ok(Event::Key(key)) = event::read() {
        match key.code {
            KeyCode::Left => Some(PlayerAction::MoveLeft),
            KeyCode::Right => Some(PlayerAction::MoveRight),
            KeyCode::Char(' ') => Some(PlayerAction::Shoot),
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(PlayerAction::Quit),
            _ => None,
        }
    } else {
        None
    }
}
