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
pub fn handle_normal_mode_key(key: &KeyEvent) -> NormalModeAction {
    match key.code {
        KeyCode::Char('q') => NormalModeAction::Quit,
        KeyCode::Down | KeyCode::Char('j') => NormalModeAction::SelectNext,
        KeyCode::Up | KeyCode::Char('k') => NormalModeAction::SelectPrev,
        KeyCode::Char('i') => NormalModeAction::EnterInput,
        _ => NormalModeAction::None,
    }
}

/// Actions that can be taken in normal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalModeAction {
    Quit,
    SelectNext,
    SelectPrev,
    EnterInput,
    None,
}

/// Handles key events in editing mode, mutating the input string as needed.
/// Returns an enum describing the action to take.
pub fn handle_editing_mode_key(key: &KeyEvent, input: &mut String) -> EditingModeAction {
    match key.code {
        KeyCode::Enter => EditingModeAction::Submit,
        KeyCode::Esc => EditingModeAction::Cancel,
        KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            delete_prev_word(input);
            EditingModeAction::Edited
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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
