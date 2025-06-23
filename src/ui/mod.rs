pub mod input;
pub mod issue;
pub mod issue_list;
pub mod theme;

use crate::app::App;
use crate::ui::{
    input::{InputMode, TextInputWidget},
    issue_list::render_issue_list,
    theme::THEME,
};
use itertools::Itertools;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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

    // Left side: split vertically into issue list (top), input (middle), and footer (bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),    // issue list
            Constraint::Length(2), // input
            Constraint::Length(1), // footer/hints
        ])
        .split(main_chunks[0]);

    render_issue_list(f, app, left_chunks[0]);
    render_issue_input(f, app, left_chunks[1]);
    render_footer(f, app, left_chunks[2]);

    if app.sidebar_visible {
        render_sidebar(f, app, main_chunks[1]);
    }
}

/// Renders the new issue input widget.
fn render_issue_input(f: &mut Frame, app: &mut App, area: Rect) {
    let area = area.inner(Margin::new(2, 0));

    let is_editing = app.input_mode == InputMode::Insert;
    let widget =
        TextInputWidget::new(&app.input, "New issue (i)", THEME.input, THEME.input_placeholder);

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
        let mut lines = vec![
            Line::from(vec![Span::styled(&issue.summary, THEME.details_title)]),
            Line::from(vec![
                Span::styled("ID: ", Style::default().add_modifier(ratatui::style::Modifier::BOLD)),
                Span::raw(&issue.id),
            ]),
        ];

        if let Some(ref t) = issue.issue_type {
            lines.push(Line::from(vec![
                Span::styled(
                    "Type: ",
                    Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(t),
            ]));
        }
        if let Some(ref s) = issue.status {
            lines.push(Line::from(vec![
                Span::styled(
                    "Status: ",
                    Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(s.as_str()),
            ]));
        }
        if let Some(ref p) = issue.priority {
            lines.push(Line::from(vec![
                Span::styled(
                    "Priority: ",
                    Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(p.as_str()),
            ]));
        }
        if let Some(points) = issue.story_points {
            lines.push(Line::from(vec![
                Span::styled(
                    "Story Points: ",
                    Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(points.to_string()),
            ]));
        }
        if let Some(ref epic) = issue.parent_epic {
            lines.push(Line::from(vec![
                Span::styled(
                    "Parent Epic: ",
                    Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(epic),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(issue.description.clone()));
        lines
    } else {
        vec![Line::from("No issue selected")]
    };
    let details =
        Paragraph::new(details).block(Block::default().borders(Borders::LEFT).title("Details"));
    f.render_widget(details, area);
}

/// Renders the footer with key hints at the bottom of the UI.
fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let (color, mode, key_hints) = match app.input_mode {
        InputMode::Normal => (
            THEME.footer_normal,
            "NORMAL",
            vec![("i", "new issue"), ("s", "sidebar"), ("q", "quit")],
        ),
        InputMode::Insert => (
            THEME.footer_insert,
            "INSERT",
            vec![("Enter", "submit"), ("Esc", "cancel"), ("^U", "clear")],
        ),
    };

    let inverted = Style { fg: color.bg, bg: color.fg, ..color };

    let mode_span = Span::styled(format!(" {mode} "), color);

    let key_hint_spans = key_hints.iter().map(|(key, label)| {
        vec![Span::styled(format!(" {key} "), color), Span::styled(format!(" {label} "), inverted)]
    });

    let spans = Itertools::intersperse(
        std::iter::once(vec![mode_span]).chain(key_hint_spans),
        vec![Span::raw("  ")],
    )
    .flatten()
    .collect::<Vec<_>>();

    let footer = Line::from(spans);

    let block = Block::default().borders(Borders::NONE);
    let para = Paragraph::new(footer).block(block);
    f.render_widget(para, area);
}
