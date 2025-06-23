use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub list_highlight: Style,
    pub list_highlight_inactive: Style,
    pub input: Style,
    pub input_placeholder: Style,
    pub footer_normal: Style,
    pub footer_insert: Style,
    pub details_title: Style,
}

impl Theme {
    pub const fn new() -> Self {
        Self {
            list_highlight: Style::new()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            list_highlight_inactive: Style::new()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::DIM),
            input: Style::new().fg(Color::Yellow),
            input_placeholder: Style::new().fg(Color::DarkGray),
            footer_normal: Style::new()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            footer_insert: Style::new()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            details_title: Style::new().add_modifier(Modifier::BOLD),
        }
    }
}

pub const THEME: Theme = Theme::new();
