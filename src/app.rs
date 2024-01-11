use crate::{calendar::CalendarInfo, editor::save_file};

use slog::Logger;
use slog::{o, Drain};
use slog_async::Async;
use slog_term::{FullFormat, PlainSyncDecorator};

use std::fs::{self, OpenOptions};
use std::sync::Arc;
use std::{path::PathBuf, str::FromStr};
use time::{Date, OffsetDateTime};
use tui_textarea::TextArea;

/// Application state holding various runtime properties.
pub struct AppState<'a> {
    pub mode: Mode,
    pub selected_date: Date,
    pub quit_flag: bool,
    pub initialized: bool,
    pub holiday_info: Option<CalendarInfo>,
    pub editor: TextArea<'a>,
    pub entries_dir: PathBuf,
    pub logger: Arc<Logger>,
}

/// Enum representing the different modes of operation within the application.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    CALENDAR,
    EDITOR,
    SORT,
}

impl AppState<'_> {
    /// Constructs a new `AppState` with default values.
    pub fn new() -> Self {
        let mut app = AppState {
            mode: Mode::CALENDAR,
            selected_date: OffsetDateTime::now_local().unwrap().date(),
            quit_flag: false,
            initialized: false,
            holiday_info: None,
            editor: TextArea::default(),
            entries_dir: PathBuf::from_str("entries").unwrap(),
            logger: Arc::new(Logger::root(slog::Discard, slog::o!())), // Placeholder logger
        };
        app.initialize_logging();
        app
    }

    /// Marks the application as initialized.
    pub fn initialized(&mut self) {
        self.initialized = true;
    }

    pub fn initialize_logging(&mut self) {
        let log_path = "app.log";
        // try to remove the existing log file
        match fs::remove_file(log_path) {
            Err(_) => {} // Ignore errors if the file does not exist
            _ => {}      // Continue if the file was successfully removed
        };
        // logfile
        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_path)
        {
            Ok(logfile) => {
                // instantiate logging
                let decorator = PlainSyncDecorator::new(logfile);
                let drain = FullFormat::new(decorator).build().fuse();
                let drain = Async::new(drain).build().fuse();
                let logger = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
                self.logger = Arc::new(logger);
            }
            Err(e) => slog::error!(self.logger, "Failed to open log file: {}", e),
        };
    }

    /// Resets the application state to default values.
    pub fn reset(&mut self) {
        self.mode = Mode::CALENDAR;
        self.selected_date = OffsetDateTime::now_local().unwrap().date();
        self.quit_flag = false;
        self.initialized = false;
        self.editor = TextArea::default();
    }

    /// Saves the current state of the editor to a file and resets the application state.
    pub fn save(&mut self) {
        save_file(self);
        self.reset();
    }

    /// Sets the quit flag to true, indicating that the application should exit.
    pub fn quit(&mut self) {
        self.quit_flag = true;
    }
}
