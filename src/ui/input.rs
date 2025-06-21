//! Input handling logic for the Jira TUI.
//!
//! This module provides functions to handle key events in both normal and editing modes.
//! It is designed to be testable and independent of the UI framework.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Represents the current input mode of the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Handles key events in normal mode.
/// Returns an enum describing the action to take.
/// Handles key events in normal mode, supporting numeric prefixes for j/k.
/// Returns an enum describing the action to take.
pub fn handle_normal_mode_key(
    key: &KeyEvent,
    pending_count: &mut Option<usize>,
) -> NormalModeAction {
    use KeyCode::*;

    // Accumulate digits and return early
    if let Char(c) = key.code {
        if c.is_ascii_digit() && !(c == '0' && pending_count.is_none()) {
            let digit = c.to_digit(10).unwrap() as usize;
            *pending_count = Some(pending_count.unwrap_or(0) * 10 + digit);
            return NormalModeAction::None;
        }
    }

    match (key.code, pending_count.take().unwrap_or(1)) {
        (Down | Char('j'), count) => NormalModeAction::Jump(count as isize),
        (Up | Char('k'), count) => NormalModeAction::Jump(-(count as isize)),
        (Char('d'), _) => NormalModeAction::Jump(20),
        (Char('u'), _) => NormalModeAction::Jump(-20),
        (Char('i'), _) => NormalModeAction::EnterInput,
        (Char('g'), _) => NormalModeAction::GotoTop,
        (Char('G'), _) => NormalModeAction::GotoBottom,
        (Char('q'), _) => NormalModeAction::Quit,
        _ => NormalModeAction::None,
    }
}

/// Actions that can be taken in normal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalModeAction {
    Quit,
    Jump(isize),
    EnterInput,
    GotoTop,
    GotoBottom,
    None,
}

/// Handles key events in editing mode, mutating the input string as needed.
/// Returns an enum describing the action to take.
pub fn handle_editing_mode_key(key: &KeyEvent, input: &mut String) -> EditingModeAction {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Enter => EditingModeAction::Submit,
        KeyCode::Esc => EditingModeAction::Cancel,
        KeyCode::Char('w') if ctrl => {
            delete_prev_word(input);
            EditingModeAction::Edited
        }
        KeyCode::Char('u') if ctrl => {
            input.clear();
            EditingModeAction::Edited
        }
        KeyCode::Char(c) => {
            input.push(c);
            EditingModeAction::Edited
        }
        KeyCode::Backspace => {
            input.pop();
            EditingModeAction::Edited
        }
        _ => EditingModeAction::None,
    }
}

/// Actions that can be taken in editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditingModeAction {
    Submit,
    Cancel,
    Edited,
    None,
}

/// Deletes the previous word from the input string.
fn delete_prev_word(input: &mut String) {
    let trimmed = input.trim_end_matches(|c: char| c.is_whitespace());
    if let Some(pos) = trimmed.rfind(|c: char| c.is_whitespace()) {
        input.truncate(pos);
        input.truncate(input.trim_end_matches(|c: char| c.is_whitespace()).len());
    } else {
        input.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_prev_word() {
        let mut s = String::from("hello world");
        delete_prev_word(&mut s);
        assert_eq!(s, "hello");

        let mut s = String::from("hello ");
        delete_prev_word(&mut s);
        assert_eq!(s, "");

        let mut s = String::from("one two three");
        delete_prev_word(&mut s);
        assert_eq!(s, "one two");

        let mut s = String::from("singleword");
        delete_prev_word(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_handle_editing_mode_key_ctrl_u() {
        let mut s = String::from("something here");
        let key = KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL);
        let action = handle_editing_mode_key(&key, &mut s);
        assert_eq!(s, "");
        assert_eq!(action, EditingModeAction::Edited);
    }

    #[test]
    fn test_handle_editing_mode_key_ctrl_w() {
        let mut s = String::from("foo bar baz");
        let key = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL);
        let action = handle_editing_mode_key(&key, &mut s);
        assert_eq!(s, "foo bar");
        assert_eq!(action, EditingModeAction::Edited);
    }
}
