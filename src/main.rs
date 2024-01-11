use crate::update::UpdateEvent;
use app::AppState;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};

use tui::Tui;
// use update::update;

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

// journal editor ui
pub mod editor;

// application logic loop
pub mod update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create application state instance
    let mut app = AppState::new();
    // initialize logging
    slog::info!(app.logger, "Application started"; "version" => env!("CARGO_PKG_VERSION"));
    // initialize terminal backend
    let backend = CrosstermBackend::new(std::io::stderr());
    // connect terminal user interface to backend
    let terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(e) => {
            eprintln!("Failed to create terminal: {}", e);
            return Err(e.into());
        }
    };
    // create eventhandler, set polling rate to 32hz
    let events = EventHandler::new(128);
    // create mutatble terminal object
    let mut tui = Tui::new(terminal, events);
    // configure terminal display/input
    tui.configure()?;
    // initial draw
    tui.draw(&mut app)?;
    // start the main logic loop
    while !app.quit_flag {
        // constant refresh
        tui.draw(&mut app)?;
        // process event queue
        match tui.events.next()? {
            // generic
            Event::Tick => {}
            // keyinput received
            Event::Key(key_event) => {
                key_event.update(&mut app);
                // check operating mode
                match app.mode {
                    app::Mode::CALENDAR => {
                        // draw widgets
                        tui.draw(&mut app)?; //🌕
                                             // process key
                    }
                    app::Mode::EDITOR => {
                        // draw widgets
                        tui.draw(&mut app)?; //🌗
                                             // process key
                    }
                    app::Mode::SORT => {
                        // draw widgets
                        tui.draw(&mut app)?; //🌑
                                             // process key
                    }
                }
            }
            // mouseinput received
            Event::Mouse(mouse_event) => {
                mouse_event.update(&mut app);
                match app.mode {
                    // draw widgets based on operating mode
                    app::Mode::CALENDAR => tui.draw(&mut app)?,
                    app::Mode::EDITOR => tui.draw(&mut app)?,
                    app::Mode::SORT => todo!(),
                }
            }
            Event::Resize(_width, _height) => tui.resize(&mut app)?,
        };
    }
    // exit ui
    tui.exit()?;
    Ok(())
}
