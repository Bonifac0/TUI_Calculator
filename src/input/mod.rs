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
        KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => AppAction::MoveCursorLeft,
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => AppAction::MoveCursorRight,
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => AppAction::ScrollUp,
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => AppAction::ScrollDown,
        KeyCode::Home => AppAction::MoveCursorHome,
        KeyCode::End => AppAction::MoveCursorEnd,
        KeyCode::Backspace => AppAction::Backspace,
        KeyCode::Delete if key.modifiers.contains(KeyModifiers::SHIFT) => AppAction::AllClear,
        KeyCode::Delete => AppAction::ClearInput,
        KeyCode::Enter | KeyCode::Char('=') => AppAction::Evaluate,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => AppAction::AllClear,
        KeyCode::Char('u') | KeyCode::Char('U') => AppAction::ToggleAngleUnit,
        KeyCode::Char(c) if c.is_ascii_alphabetic() => {
            let upper = c.to_ascii_uppercase();
            if ('A'..='F').contains(&upper) {
                AppAction::InsertVariable(upper)
            } else {
                AppAction::Warn(format!("Ignored key '{}': only A-F can be inserted", c))
            }
        }
        KeyCode::Char(c) if is_allowed_symbol(c) => AppAction::InsertChar(c),
        KeyCode::Char(c) => AppAction::Warn(format!("Ignored key '{}': not a valid input symbol", c)),
        _ => AppAction::Warn(format!("Ignored key: {:?}", key.code)),
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

fn is_allowed_symbol(c: char) -> bool {
    matches!(c,
        // digits and decimal
        '0'..='9' | '.' |
        // arithmetic operators
        '+' | '-' | '*' | '/' | '%' | '^' | '!' |
        // brackets and delimiters
        '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' |
        // LaTeX entry and display aliases
        '\\' | '×' | '÷' | '√'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_vim_navigation_keys() {
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)),
            AppAction::MoveCursorLeft
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)),
            AppAction::MoveCursorRight
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)),
            AppAction::ScrollUp
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)),
            AppAction::ScrollDown
        );
    }

    #[test]
    fn maps_delete_shortcuts() {
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)),
            AppAction::ClearInput
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Delete, KeyModifiers::SHIFT)),
            AppAction::AllClear
        );
    }

    #[test]
    fn maps_variable_letters_to_uppercase() {
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
            AppAction::InsertVariable('A')
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE)),
            AppAction::InsertVariable('F')
        );
    }

    #[test]
    fn warns_on_non_variable_letters() {
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)),
            AppAction::Warn("Ignored key 'g': only A-F can be inserted".to_string())
        );
    }

    #[test]
    fn allows_math_symbols() {
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE)),
            AppAction::InsertChar('+')
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE)),
            AppAction::InsertChar('!')
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::NONE)),
            AppAction::InsertChar('^')
        );
    }

    #[test]
    fn warns_on_disallowed_symbols() {
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE)),
            AppAction::Warn("Ignored key '@': not a valid input symbol".to_string())
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('~'), KeyModifiers::NONE)),
            AppAction::Warn("Ignored key '~': not a valid input symbol".to_string())
        );
        assert_eq!(
            handle_key_event(KeyEvent::new(KeyCode::Char('#'), KeyModifiers::NONE)),
            AppAction::Warn("Ignored key '#': not a valid input symbol".to_string())
        );
    }
}
