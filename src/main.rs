use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::error::Error;

mod app;
mod jira;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Clear(ClearType::All)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_jira_tui(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("{e}");
    }

    Ok(())
}

async fn run_jira_tui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn Error>> {
    let config = jira::JiraConfig::from_env()
        .map_err(|e| format!("Failed to load Jira config from environment: {e}"))?;
    let search_results = jira::fetch_assigned_issues(&config, 100).await?;
    let issues = search_results
        .issues
        .unwrap_or_default()
        .into_iter()
        .map(|j| ui::issue::Issue::from_jira(&j))
        .collect();

    let app = app::App::new(issues);
    app::run_app(terminal, app)?;

    Ok(())
}
