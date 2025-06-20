use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct Issue {
    title: String,
    description: String,
}

struct App {
    issues: Vec<Issue>,
    selected: usize,
    input_mode: InputMode,
    input: String,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App {
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
    };

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            ui(f, &app);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.selected + 1 < app.issues.len() {
                                app.selected += 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.selected > 0 {
                                app.selected -= 1;
                            }
                        }
                        KeyCode::Char('i') => {
                            app.input_mode = InputMode::Editing;
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            if !app.input.trim().is_empty() {
                                app.issues.push(Issue {
                                    title: app.input.trim().to_string(),
                                    description: "No description.".to_string(),
                                });
                                app.input.clear();
                            }
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Delete previous word
                            let trimmed = app.input.trim_end_matches(|c: char| c.is_whitespace());
                            if let Some(pos) = trimmed.rfind(|c: char| c.is_whitespace()) {
                                app.input.truncate(pos + 1);
                                app.input = app.input.to_string();
                            } else {
                                app.input.clear();
                            }
                        }
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Clear the input line
                            app.input.clear();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        _ => {}
                    },
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    // Split horizontally: left (issue list + input), right (sidebar/details)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.area());

    // Left side: split vertically into issue list (top) and input (bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(main_chunks[0]);

    // Issue List (top left)
    let items: Vec<ListItem> = app
        .issues
        .iter()
        .map(|i| ListItem::new(i.title.clone()))
        .collect();
    let issues = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(
        issues,
        left_chunks[0],
        &mut list_state(app.selected, app.issues.len()),
    );

    // New Issue Input (bottom left)
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::TOP)
                .title("New Issue (i)"),
        );
    f.render_widget(input, left_chunks[1]);

    // Details Sidebar (right)
    let details = if let Some(issue) = app.issues.get(app.selected) {
        vec![
            Line::from(vec![Span::styled(
                &issue.title,
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(issue.description.clone()),
        ]
    } else {
        vec![Line::from("No issue selected")]
    };
    let details = Paragraph::new(details).block(Block::default().borders(Borders::LEFT));
    f.render_widget(details, main_chunks[1]);

    // Show cursor in input mode
    if app.input_mode == InputMode::Editing {
        let x = left_chunks[1].x + app.input.len() as u16;
        let y = left_chunks[1].y + 1;
        f.set_cursor_position((x, y));
    }
}

fn list_state(selected: usize, len: usize) -> ratatui::widgets::ListState {
    let mut state = ratatui::widgets::ListState::default();
    if len > 0 {
        state.select(Some(selected));
    }
    state
}
