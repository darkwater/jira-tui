//! Input handling logic for the Jira TUI.
//!
//! This module provides functions to handle key events in both normal and editing modes.
//! It is designed to be testable and independent of the UI framework.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// --- ratatui widget imports for custom input widget ---
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, StatefulWidget, Widget};

/// Represents the current input mode of the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
}

// --- TextInput stateful widget and state ---

/// State for the text input widget (cursor position, selection, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TextInputState {
    /// The cursor position (byte index in value).
    pub cursor: usize,
}

impl TextInputState {
    pub fn new(cursor: usize) -> Self {
        Self { cursor }
    }
}

/// A simple single-line text input widget.
pub struct TextInputWidget<'a> {
    pub value: &'a str,
    pub placeholder: &'a str,
    pub is_editing: bool,
    pub style: Style,
    pub placeholder_style: Style,
    pub block: Option<Block<'a>>,
}

impl<'a> TextInputWidget<'a> {
    pub fn new(
        value: &'a str,
        placeholder: &'a str,
        is_editing: bool,
        style: Style,
        placeholder_style: Style,
    ) -> Self {
        Self {
            value,
            placeholder,
            is_editing,
            style,
            placeholder_style,
            block: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for TextInputWidget<'a> {
    type State = TextInputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (display, style) = if self.value.is_empty() {
            (self.placeholder, self.placeholder_style)
        } else {
            (self.value, self.style)
        };

        let mut text = Text::from(Line::from(Span::styled(display, style)));

        let mut inner_area = area;
        if let Some(block) = self.block.as_ref() {
            block.render(area, buf);
            inner_area = block.inner(area);
            if inner_area.width < 1 || inner_area.height < 1 {
                return;
            }
        }

        // Render the text
        Widget::render(ratatui::widgets::Paragraph::new(text.clone()), inner_area, buf);

        // Cursor is set by the Frame, not the Buffer.
        // See render_issue_input in mod.rs for cursor logic.
    }
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
    use KeyModifiers as M;

    // Accumulate digits and return early
    if let Char(c) = key.code {
        if c.is_ascii_digit() && !(c == '0' && pending_count.is_none()) {
            let digit = c.to_digit(10).unwrap() as usize;
            *pending_count = Some(pending_count.unwrap_or(0) * 10 + digit);
            return NormalModeAction::None;
        }
    }

    match (pending_count.take().unwrap_or(1), key.modifiers, key.code) {
        (count, M::NONE, Char('j') | Down) => NormalModeAction::Jump(count as isize),
        (count, M::NONE, Char('k') | Up) => NormalModeAction::Jump(-(count as isize)),
        (_, M::NONE, Char('d')) => NormalModeAction::Jump(20),
        (_, M::NONE, Char('u')) => NormalModeAction::Jump(-20),
        (_, M::NONE, Char('i')) => NormalModeAction::EnterInput,
        (_, M::NONE, Char('g')) => NormalModeAction::GotoTop,
        (_, M::NONE, Char('G')) => NormalModeAction::GotoBottom,
        (_, M::NONE, Char('s')) => NormalModeAction::ToggleSidebar,
        (_, M::NONE, Char('q')) => NormalModeAction::Quit,
        (count, M::CONTROL, Char('e')) => NormalModeAction::Scroll(count as isize),
        (count, M::CONTROL, Char('y')) => NormalModeAction::Scroll(-(count as isize)),
        _ => NormalModeAction::None,
    }
}

/// Actions that can be taken in normal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalModeAction {
    Quit,
    Jump(isize),
    Scroll(isize),
    EnterInput,
    GotoTop,
    GotoBottom,
    ToggleSidebar,
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
    // Remove trailing whitespace
    let trimmed = input.trim_end_matches(|c: char| c.is_whitespace());

    // Find the last whitespace *before* the word
    if let Some(pos) = trimmed.rfind(|c: char| c.is_whitespace()) {
        // Truncate after the whitespace (keep the whitespace itself)
        input.truncate(pos + 1);
    } else {
        // No whitespace found, clear the whole string
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
        assert_eq!(s, "hello ");

        let mut s = String::from("hello  world");
        delete_prev_word(&mut s);
        assert_eq!(s, "hello  ");

        let mut s = String::from("hello ");
        delete_prev_word(&mut s);
        assert_eq!(s, "");

        let mut s = String::from("one two three");
        delete_prev_word(&mut s);
        assert_eq!(s, "one two ");

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
        assert_eq!(s, "foo bar ");
        assert_eq!(action, EditingModeAction::Edited);
    }
}
