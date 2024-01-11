use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, panic};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{
    app::{AppState, Mode},
    event::EventHandler,
    ui,
};
// generics used to maintain identical functionality with reference handle
trait Render {
    fn render(&mut self, app: &mut AppState) -> Result<(), Box<dyn std::error::Error>>;
}

impl Render for Tui {
    fn render(&mut self, app: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
        match app.mode {
            Mode::CALENDAR => {
                self.terminal
                    .draw(|frame| ui::render_journal_calendar(app, frame))?;
            }
            Mode::EDITOR => {
                self.terminal
                    .draw(|frame| ui::render_journal_entry(app, frame))?;
            }
            Mode::SORT => {
                // Assuming there's a render_journal_sort function
                self.terminal.draw(|_frame| unimplemented!())?;
            } // Handle other modes if necessary
        }
        Ok(())
    }
}

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
    pub fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    // exits interface
    // disables raw mode + reverts terminal properties
    pub fn exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    // draws terminal interface -> renders pertinent widgets
    pub fn draw(&mut self, app: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
        match self.render(app) {
            Ok(()) => return Ok(()),
            Err(_) => Ok(()),
        }
    }
    // frame auto resizes, this method is only for idiomatic purposes
    pub fn resize(&mut self, app: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
        match self.render(app) {
            Ok(()) => return Ok(()),
            Err(_) => Ok(()),
        }
    }

    // pub fn draw<Func: Fn(&mut AppState, &mut Frame, Rect)>(
    //     &mut self,
    //     app: &mut AppState,
    //     draw_function: Func,
    // ) -> Result<()>
    // where
    //     Func: Fn(),
    // {
    //     self.terminal
    //         .draw(|frame| draw_function(app, frame, Rect::default()))?;
    //     Ok(())
    // }
}
