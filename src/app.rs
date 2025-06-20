use crossterm::event::{self};
use ratatui::Terminal;
use ratatui::backend::Backend;
use std::io;
use std::time::{Duration, Instant};

use crate::input::InputMode;
use crate::issue::Issue;

pub struct App {
    pub issues: Vec<Issue>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub input: String,
    pub sidebar_visible: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            issues: vec![
                Issue {
                    title: "First issue".to_string(),
                    description: "This is the first issue.".to_string(),
                },
                Issue {
                    title: "Second issue".to_string(),
                    description: "This is the second issue.".to_string(),
                },
            ],
            selected: 0,
            input_mode: InputMode::Normal,
            input: String::new(),
            sidebar_visible: true,
        }
    }
}

use crate::input::{EditingModeAction, NormalModeAction};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| crate::ui::render_ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match crate::input::handle_normal_mode_key(&key) {
                        NormalModeAction::Quit => return Ok(()),
                        NormalModeAction::SelectNext => {
                            if app.selected + 1 < app.issues.len() {
                                app.selected += 1;
                            }
                        }
                        NormalModeAction::SelectPrev => {
                            if app.selected > 0 {
                                app.selected -= 1;
                            }
                        }
                        NormalModeAction::EnterInput => {
                            app.input_mode = InputMode::Editing;
                        }
                        NormalModeAction::None => {}
                    },
                    InputMode::Editing => {
                        match crate::input::handle_editing_mode_key(&key, &mut app.input) {
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
                            EditingModeAction::Edited | EditingModeAction::None => {}
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
