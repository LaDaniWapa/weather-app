use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen};
use std::io::{stdout};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;
use crate::errors::{AppError, Result};

mod weather;
mod errors;
mod app;

#[tokio::main]
async fn main() -> Result<()> {
    //setup
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    //main loop
    let mut app = App::new();
    app.run(&mut terminal).await?;

    // cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}