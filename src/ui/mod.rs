pub mod input;
pub mod issue;

use crate::app::App;
use crate::ui::input::{InputMode, TextInputWidget};
use ratatui::layout::Margin;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// Renders the entire UI, including the issue list, input, and (optionally) the sidebar.
pub fn render_ui(f: &mut Frame, app: &mut App) {
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
        .constraints([Constraint::Min(5), Constraint::Length(2)])
        .split(main_chunks[0]);

    render_issue_list(f, app, left_chunks[0]);
    render_issue_input(f, app, left_chunks[1]);

    if app.sidebar_visible {
        render_sidebar(f, app, main_chunks[1]);
    }
}

/// Renders the issue list widget.
fn render_issue_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .issues
        .iter()
        .map(|i| ListItem::new(i.title.clone()))
        .collect();

    let issues = List::new(items).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(issues, area, &mut app.list_state);
}

/// Renders the new issue input widget.
fn render_issue_input(f: &mut Frame, app: &mut App, area: Rect) {
    let area = area.inner(Margin::new(2, 0));

    let is_editing = app.input_mode == InputMode::Insert;
    let widget = TextInputWidget::new(
        &app.input,
        "New issue (i)",
        is_editing,
        Style::default().fg(Color::Yellow),
        Style::default().fg(Color::DarkGray),
    );

    f.render_stateful_widget(widget, area, &mut app.input_state);

    // Show cursor in input mode using stateful cursor position
    if is_editing {
        let x = area.x + app.input_state.cursor.min(area.width as usize - 1) as u16;
        let y = area.y;
        f.set_cursor_position((x, y));
    }
}

/// Renders the sidebar/details widget, if visible.
fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let selected = app.list_state.selected().unwrap_or(0);
    let details = if let Some(issue) = app.issues.get(selected) {
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
