use crate::update::UpdateEvent;
use anyhow::Result;
use app::AppState;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::collections::HashMap;
use tui::Tui;
use ui::render;
// use update::update;

use time::{Date, Duration, OffsetDateTime};

// application
pub mod app;

// terminal events handler
pub mod event;

// calendar widget
pub mod calendar;

// widget renderer
pub mod ui;

// terminal user interfaces
pub mod tui;

// application logic loop
pub mod update;

fn main() -> Result<()> {
    // create application state instance
    let mut app = AppState::new();

    // initialize terminal user interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(15);
    let mut tui = Tui::new(terminal, events);
    tui.configure()?;

    // start the main logic loop
    while !app.quit_flag {
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => {
                key_event.update(&mut app);
                tui.draw(&mut app)?;
            }
            Event::Mouse(mouse_event) => {
                mouse_event.update(&mut app);
                tui.draw(&mut app)?;
            }
            Event::Resize(_, _) => tui.draw(&mut app)?,
        };
    }
    // exit ui
    tui.exit()?;
    Ok(())
}
