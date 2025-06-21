use crossterm::event::{self};
use ratatui::Terminal;
use ratatui::backend::Backend;
use std::io;
use std::time::{Duration, Instant};

use crate::ui::input::InputMode;
use crate::ui::issue::Issue;

pub struct App {
    pub issues: Vec<Issue>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub input: String,
    pub sidebar_visible: bool,
}

impl App {
    pub fn new(issues: Vec<Issue>) -> Self {
        Self {
            issues,
            selected: 0,
            input_mode: InputMode::Normal,
            input: String::new(),
            sidebar_visible: true,
        }
    }
}

use crate::ui::input::{EditingModeAction, NormalModeAction};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();
    let mut pending_count: Option<usize> = None;

    loop {
        terminal.draw(|f| crate::ui::render_ui(f, &app))?;

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
                                    app.selected = 0;
                                } else {
                                    let new_idx = app.selected as isize + offset;
                                    app.selected = new_idx.clamp(0, len as isize - 1) as usize;
                                }
                            }
                            NormalModeAction::GotoTop => {
                                app.selected = 0;
                            }
                            NormalModeAction::GotoBottom => {
                                if !app.issues.is_empty() {
                                    app.selected = app.issues.len() - 1;
                                }
                            }
                            NormalModeAction::EnterInput => {
                                app.input_mode = InputMode::Editing;
                            }
                            NormalModeAction::None => {}
                        }
                    }
                    InputMode::Editing => {
                        match crate::ui::input::handle_editing_mode_key(&key, &mut app.input) {
                            EditingModeAction::Submit => {
                                if !app.input.trim().is_empty() {
                                    app.issues.push(Issue::new(
                                        app.input.trim().to_string(),
                                        "No description.".to_string(),
                                    ));
                                    app.input.clear();
                                }
                                app.input_mode = InputMode::Normal;
                            }
                            EditingModeAction::Cancel => {
                                app.input_mode = InputMode::Normal;
                            }
                            EditingModeAction::Edited => {}
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
