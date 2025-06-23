use crate::app::App;
use crate::ui::theme::THEME;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Cell, HighlightSpacing, Row, Table, TableState},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Id,
    Summary,
    Status,
    Priority,
}

#[derive(Debug, Clone, Copy)]
enum FieldWidth {
    Flexible { factor: u16, min: u16 },
    Fixed(u16),
}

impl Field {
    // Order in which fields are rendered in the row
    pub const RENDER_ORDER: &'static [Field] =
        &[Field::Id, Field::Priority, Field::Summary, Field::Status];

    // Priority order for hiding fields (first field is always shown)
    pub const PRIORITY: &'static [Field] =
        &[Field::Summary, Field::Status, Field::Id, Field::Priority];

    pub const fn width(self) -> FieldWidth {
        match self {
            Field::Id => FieldWidth::Fixed(8),
            Field::Summary => FieldWidth::Flexible { factor: 5, min: 20 },
            Field::Status => FieldWidth::Flexible { factor: 1, min: 5 },
            Field::Priority => FieldWidth::Fixed(1),
        }
    }

    pub fn cell(self, issue: &crate::ui::issue::Issue) -> Cell {
        match self {
            Field::Id => Cell::from(issue.id.clone()).style(Style::default().fg(Color::DarkGray)),
            Field::Summary => Cell::from(issue.summary.clone()),
            Field::Status => {
                let (text, color) = match issue.status.as_ref() {
                    Some(status) => (status.as_str(), status.color(&THEME)),
                    None => ("", THEME.gray),
                };
                Cell::from(text).style(Style::default().fg(color))
            }
            Field::Priority => {
                let (text, color) = match issue.priority.as_ref() {
                    Some(priority) => (priority.as_str(), priority.color(&THEME)),
                    None => ("", THEME.yellow),
                };
                Cell::from(text).style(Style::default().fg(color))
            }
        }
    }
}

pub fn render_issue_list(f: &mut Frame, app: &mut App, area: Rect) {
    let available_width = area.width;
    let mut used_width = 0u16;
    let mut shown_fields: Vec<Field> = vec![];

    // Always show the first field (by priority)
    let first = Field::PRIORITY[0];
    let min_first = match first.width() {
        FieldWidth::Flexible { min, .. } => min,
        FieldWidth::Fixed(w) => w,
    };
    used_width += min_first;
    shown_fields.push(first);

    // Try to add more fields as space allows (by priority)
    for field in Field::PRIORITY.iter().skip(1) {
        let min_w = match field.width() {
            FieldWidth::Flexible { min, .. } => min,
            FieldWidth::Fixed(w) => w,
        };
        if used_width + min_w + 2 <= available_width {
            used_width += min_w + 2;
            shown_fields.push(*field);
        }
    }

    // Compute total flexible factor for shown fields
    let total_flex: u16 = shown_fields
        .iter()
        .map(|field| match field.width() {
            FieldWidth::Flexible { factor, .. } => factor,
            FieldWidth::Fixed(_) => 0,
        })
        .sum();

    // Compute widths for each shown field (in render order)
    let mut constraints: Vec<ratatui::layout::Constraint> = vec![];
    let mut fixed_total: u16 = shown_fields
        .iter()
        .map(|field| match field.width() {
            FieldWidth::Fixed(w) => w,
            FieldWidth::Flexible { min, .. } => min,
        })
        .sum();

    // Add 2 spaces between columns for each column except the last
    let spacing_total = (shown_fields.len().saturating_sub(1) as u16) * 2;
    fixed_total += spacing_total;

    let remaining_width = available_width.saturating_sub(fixed_total);

    for field in Field::RENDER_ORDER
        .iter()
        .filter(|f| shown_fields.contains(f))
    {
        match field.width() {
            FieldWidth::Fixed(w) => constraints.push(ratatui::layout::Constraint::Length(w)),
            FieldWidth::Flexible { factor, min } => {
                let flex_width = if total_flex > 0 {
                    min + (remaining_width * factor / total_flex)
                } else {
                    min
                };
                constraints.push(ratatui::layout::Constraint::Min(flex_width));
            }
        }
    }

    // Build table rows
    let rows: Vec<Row> = app
        .issues
        .iter()
        .map(|issue| {
            let cells = Field::RENDER_ORDER
                .iter()
                .filter(|f| shown_fields.contains(f))
                .map(|&field| field.cell(issue))
                .collect::<Vec<_>>();
            Row::new(cells)
        })
        .collect();

    let highlight_style = if app.input_mode == crate::ui::input::InputMode::Insert {
        THEME.list_highlight_inactive
    } else {
        THEME.list_highlight
    };

    let mut table_state = TableState::default();
    table_state.select(app.list_state.selected());

    let table = Table::new(rows, constraints)
        .row_highlight_style(highlight_style)
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(table, area, &mut table_state);
}
