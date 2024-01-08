use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, panic};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::AppState, event::EventHandler, ui};

// struct representing the terminal user interface
// responsible for terminal setup, initializes interface & handles drawing
pub struct Tui {
    // terminal interface
    terminal: CrosstermTerminal,
    // terminal events handler
    pub events: EventHandler,
}

impl Tui {
    // constructor
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    // initialize terminal interface
    pub fn configure(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // define custom panic hook to prevent terminal corruption
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to restore terminal. recommend killing process");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }
    // resets terminal interface parameters
    // used by panic hooks to revert terminal properties
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    // exits interface
    // disables raw mode + reverts terminal properties
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    // draws terminal interface -> renders calendar widgets
    pub fn draw_calendar(&mut self, app: &mut AppState) -> Result<()> {
        self.terminal
            .draw(|frame| ui::render_journal_calendar(app, frame))?;
        Ok(())
    }

    // draws terminal interface -> renders editor
    pub fn draw_editor(&mut self, app: &mut AppState) -> Result<()> {
        self.terminal
            .draw(|frame| ui::render_journal_entry(app, frame))?;
        Ok(())
    }
}
