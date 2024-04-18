use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

use curl::easy::Easy;
use scraper::{Html, Selector};
use std::io::{stdout, Result, Stdout};
// mod app;
// mod ui;
const FPS: u64 = 60;
const BASE_SITE: &'static str = "https://mangareader.to";

fn main() -> Result<()> {
    let mut buf = Vec::new();
    {
        let mut easy = Easy::new();
        easy.url(format!("{BASE_SITE}/search?keyword=one+piece").as_str())
            .unwrap();
        let mut easy = easy.transfer();
        easy.write_function(|data| {
            let fragment = Html::parse_fragment(&String::from_utf8(data.to_vec()).unwrap());
            let selector = Selector::parse("h3").unwrap();
            buf.extend(fragment.select(&selector).map(|elem| elem.inner_html()));
            Ok(data.len())
        })
        .unwrap();
        easy.perform().unwrap();
    };
    let mut terminal = init()?;
    let wdgt = Paragraph::new(buf.into_iter().fold(String::new(), |mut acc, cur| {
        acc += &cur;
        acc
    }))
    .green();
    loop {
        terminal.draw(|frame| {
            frame.render_widget(&wdgt, frame.size());
        })?;
        if is_exit_pressed()? {
            break;
        }
    }
    restore()
}

fn init() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
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
