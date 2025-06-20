use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::error::Error;

mod app;
mod input;
mod issue;
mod jira;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load config and fetch issues from Jira
    let config = jira::JiraConfig::from_env().expect("Jira config from env");
    let search_results = jira::fetch_assigned_issues(&config, 50).await?;
    let issues = search_results
        .issues
        .unwrap_or_default()
        .into_iter()
        .map(|j| issue::Issue::from_jira(&j))
        .collect();

    let app = app::App::new(issues);

    let res = app::run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }
    Ok(())
}
