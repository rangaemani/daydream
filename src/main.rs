use crate::update::UpdateEvent;
use anyhow::Result;
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

fn main() -> Result<()> {
    // create application state instance
    let mut app = AppState::new();

    // initialize terminal user interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(32);
    let mut tui = Tui::new(terminal, events);
    tui.configure()?;

    // start the main logic loop
    while !app.quit_flag {
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => {
                match app.mode {
                    app::Mode::CALENDAR => {
                        key_event.update(&mut app);
                        tui.draw_calendar(&mut app)?;
                    } //🌕
                    app::Mode::EDITOR => {
                        key_event.update(&mut app);
                        tui.draw_editor(&mut app)?;
                    } //🌗
                    app::Mode::SORT => todo!(), //🌑
                }
            }
            Event::Mouse(mouse_event) => {
                mouse_event.update(&mut app);
                match app.mode {
                    app::Mode::CALENDAR => tui.draw_calendar(&mut app)?,
                    app::Mode::EDITOR => tui.draw_editor(&mut app)?,
                    app::Mode::SORT => todo!(),
                }
            }
            Event::Resize(_, _) => tui.draw_calendar(&mut app)?,
        };
    }
    // exit ui
    tui.exit()?;
    Ok(())
}
