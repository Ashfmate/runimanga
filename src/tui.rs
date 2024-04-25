use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

/// Type alias for stdout(), just so that if I want to change the output, I can do so in one place
type out = stdout();

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(out, EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(out))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(out, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
