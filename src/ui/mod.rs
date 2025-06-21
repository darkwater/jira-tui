pub mod input;
pub mod issue;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;
use crate::ui::input::InputMode;

/// Renders the entire UI, including the issue list, input, and (optionally) the sidebar.
pub fn render_ui(f: &mut Frame, app: &App) {
    // Split horizontally: left (issue list + input), right (sidebar/details)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(if app.sidebar_visible { 60 } else { 100 }),
            Constraint::Percentage(if app.sidebar_visible { 40 } else { 0 }),
        ])
        .split(f.area());

    // Left side: split vertically into issue list (top) and input (bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(main_chunks[0]);

    render_issue_list(f, app, left_chunks[0]);
    render_issue_input(f, app, left_chunks[1]);

    if app.sidebar_visible {
        render_sidebar(f, app, main_chunks[1]);
    }
}

/// Renders the issue list widget.
fn render_issue_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .issues
        .iter()
        .map(|i| ListItem::new(i.title.clone()))
        .collect();
    let issues = List::new(items)
        .block(Block::default().title("Issues"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(
        issues,
        area,
        &mut list_state(app.selected, app.issues.len()),
    );
}

/// Helper to create ListState for selection
fn list_state(selected: usize, len: usize) -> ratatui::widgets::ListState {
    let mut state = ratatui::widgets::ListState::default();
    if len > 0 {
        state.select(Some(selected));
    }
    state
}

/// Renders the new issue input widget.
fn render_issue_input(f: &mut Frame, app: &App, area: Rect) {
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
    f.render_widget(input, area);

    // Show cursor in input mode
    if app.input_mode == InputMode::Editing {
        let x = area.x + app.input.len() as u16;
        let y = area.y + 1;
        f.set_cursor_position((x, y));
    }
}

/// Renders the sidebar/details widget, if visible.
fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
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
    let details =
        Paragraph::new(details).block(Block::default().borders(Borders::LEFT).title("Details"));
    f.render_widget(details, area);
}
