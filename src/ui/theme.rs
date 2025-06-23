use ratatui::style::{Color, Modifier, Style};

pub const THEME: Theme = Theme::new();

pub struct Theme {
    pub list_highlight: Style,
    pub list_highlight_inactive: Style,
    pub input: Style,
    pub input_placeholder: Style,
    pub footer_normal: Style,
    pub footer_insert: Style,
    pub details_title: Style,

    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub yellow: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub black: Color,
    pub gray: Color,
    pub dark_gray: Color,
}

impl Theme {
    pub const fn new() -> Self {
        Self {
            list_highlight: Style::new().bg(Color::Black).add_modifier(Modifier::BOLD),
            list_highlight_inactive: Style::new().bg(Color::Black).add_modifier(Modifier::DIM),
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

            red: Color::Red,
            green: Color::Green,
            blue: Color::Blue,
            yellow: Color::Yellow,
            magenta: Color::Magenta,
            cyan: Color::Cyan,
            white: Color::White,
            black: Color::Black,
            gray: Color::Gray,
            dark_gray: Color::DarkGray,
        }
    }
}
