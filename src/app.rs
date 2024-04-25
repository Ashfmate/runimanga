use std::io::{Result, Stdout};

use crossterm::event;
use ratatui::{
    backend::CrosstermBackend,
    prelude::{Buffer, Rect},
    widgets::Widget,
    Frame, Terminal,
};

const FPS: u64 = 60;
const BASE_SITE: &'static str = "https://mangareader.to";

#[derive(Clone)]
enum AppStates {
    Searching,
    Viewing,
}

#[derive(Clone)]
pub struct App {
    is_exit: bool,
    state: AppStates,
}

impl App {
    pub fn new() -> Self {
        Self {
            is_exit: false,
            state: AppStates::Searching,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        while !self.is_exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget((*self).clone(), frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis((1 / FPS) as u64 * 1000))? {
            self.handle_key_input()?;
        }
        Ok(())
    }

    fn handle_key_input(&mut self) -> Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Esc => self.is_exit = false,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
