use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use curl::easy::Easy;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

const FPS: u32 = 60;
const BASE_SITE: &'static str = "https://mangareader.to";

fn main() -> Result<()> {
    let mut terminal = begin()?;

    loop {
        if is_exit_pressed()? {
            break;
        }
    }

    exit()
}

fn is_exit_pressed() -> Result<bool> {
    if event::poll(std::time::Duration::from_millis((1 / FPS) as u64 * 1000))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

fn begin() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    return Ok(terminal);
}

fn exit() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
