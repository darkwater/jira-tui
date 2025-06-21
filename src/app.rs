use crate::ui::{
    input::{InputMode, TextInputState},
    issue::Issue,
};
use crossterm::event::{self};
use ratatui::{Terminal, backend::Backend};
use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::widgets::ListState;

pub struct App {
    pub issues: Vec<Issue>,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input: String,
    pub input_state: TextInputState,
    pub sidebar_visible: bool,
}

impl App {
    pub fn new(issues: Vec<Issue>) -> Self {
        let mut list_state = ListState::default();
        if !issues.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            issues,
            list_state,
            input_mode: InputMode::Normal,
            input: String::new(),
            input_state: TextInputState::default(),
            sidebar_visible: false,
        }
    }
}

use crate::ui::input::{EditingModeAction, NormalModeAction};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();
    let mut pending_count: Option<usize> = None;

    loop {
        terminal.draw(|f| crate::ui::render_ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => {
                        match crate::ui::input::handle_normal_mode_key(&key, &mut pending_count) {
                            NormalModeAction::Quit => return Ok(()),
                            NormalModeAction::Jump(offset) => {
                                let len = app.issues.len();
                                if len == 0 {
                                    app.list_state.select(None);
                                } else {
                                    let current = app.list_state.selected().unwrap_or(0);
                                    let new_idx = (current as isize + offset)
                                        .clamp(0, len as isize - 1)
                                        as usize;
                                    app.list_state.select(Some(new_idx));
                                }
                            }
                            NormalModeAction::Scroll(scroll) => {
                                let len = app.issues.len();
                                if len == 0 {
                                    // nothing to scroll
                                } else {
                                    let offset = app.list_state.offset_mut();
                                    let max_offset = len.saturating_sub(1);
                                    let new_offset = (*offset as isize + scroll)
                                        .clamp(0, max_offset as isize)
                                        as usize;
                                    *offset = new_offset;
                                }
                            }
                            NormalModeAction::GotoTop => {
                                if !app.issues.is_empty() {
                                    app.list_state.select(Some(0));
                                }
                            }
                            NormalModeAction::GotoBottom => {
                                if !app.issues.is_empty() {
                                    app.list_state.select(Some(app.issues.len() - 1));
                                }
                            }
                            NormalModeAction::EnterInput => {
                                app.input_mode = InputMode::Insert;
                            }
                            NormalModeAction::None => {}
                        }
                    }
                    InputMode::Insert => {
                        match crate::ui::input::handle_editing_mode_key(&key, &mut app.input) {
                            EditingModeAction::Submit => {
                                if !app.input.trim().is_empty() {
                                    app.issues.push(Issue::new(
                                        app.input.trim().to_string(),
                                        "".to_string(),
                                    ));
                                    // Select the newly added issue
                                    app.list_state.select(Some(app.issues.len() - 1));
                                    app.input.clear();
                                }
                                app.input_mode = InputMode::Normal;
                                app.input_state.cursor = 0;
                            }
                            EditingModeAction::Cancel => {
                                app.input_mode = InputMode::Normal;
                                app.input_state.cursor = 0;
                            }
                            EditingModeAction::Edited => {
                                // Always update cursor to end of input after edit
                                app.input_state.cursor = app.input.len();
                            }
                            EditingModeAction::None => {}
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
