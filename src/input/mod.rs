pub mod action;

pub use action::AppAction;

use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::time::Duration;

pub fn poll_action() -> Result<Option<AppAction>, std::io::Error> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => Ok(Some(handle_key_event(key))),
            Event::Mouse(mouse) => Ok(handle_mouse_event(mouse)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn handle_key_event(key: KeyEvent) -> AppAction {
    // Check Shift + Letter (Store Variable)
    if key.modifiers.contains(KeyModifiers::SHIFT) {
        if let KeyCode::Char(c) = key.code {
            let u = c.to_ascii_uppercase();
            if ('A'..='F').contains(&u) {
                return AppAction::StoreVariable(u);
            }
        }
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => AppAction::Quit,
        KeyCode::Esc => AppAction::Quit,
        KeyCode::Char('?') | KeyCode::F(1) => AppAction::OpenHelp,
        KeyCode::Left => AppAction::MoveCursorLeft,
        KeyCode::Right => AppAction::MoveCursorRight,
        KeyCode::Up => AppAction::ScrollUp,
        KeyCode::Down => AppAction::ScrollDown,
        KeyCode::Home => AppAction::MoveCursorHome,
        KeyCode::End => AppAction::MoveCursorEnd,
        KeyCode::Backspace => AppAction::Backspace,
        KeyCode::Enter | KeyCode::Char('=') => AppAction::Evaluate,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => AppAction::AllClear,
        KeyCode::Char('c') | KeyCode::Char('C') => AppAction::ClearInput,
        KeyCode::Char('u') | KeyCode::Char('U') => AppAction::ToggleAngleUnit,
        KeyCode::Char(c) => AppAction::InsertChar(c),
        _ => AppAction::Quit,
    }
}

fn handle_mouse_event(mouse: MouseEvent) -> Option<AppAction> {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            Some(AppAction::ClickAt(mouse.column, mouse.row))
        }
        MouseEventKind::ScrollUp => Some(AppAction::ScrollUp),
        MouseEventKind::ScrollDown => Some(AppAction::ScrollDown),
        _ => None,
    }
}
