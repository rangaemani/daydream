use crate::calendar::DatePosition;
use time::{Date, Month, OffsetDateTime};
use tui_textarea::TextArea;
// App state
pub struct AppState<'a> {
    pub selected_date: Date,
    pub quit_flag: bool,
    pub initialized: bool,
    pub date_pos_map: Vec<DatePosition>,
    pub mode: Mode,
    pub editor: TextArea<'a>,
}
#[derive(Clone, Copy)]
pub enum Mode {
    CALENDAR,
    EDITOR,
    SORT,
}

impl AppState<'_> {
    pub fn new() -> Self {
        AppState {
            selected_date: {
                OffsetDateTime::now_local()
                    .unwrap()
                    .date()
                    .replace_month(Month::January)
                    .unwrap()
                    .replace_day(1)
                    .unwrap()
            },
            quit_flag: false,
            initialized: false,
            date_pos_map: Vec::new(),
            mode: Mode::CALENDAR,
            editor: TextArea::default(),
        }
    }
    // handles terminal tick events
    pub fn tick(&self) {}

    pub fn save(&mut self) {
        todo!("editor save button")
    }

    pub fn initialized(&mut self) {
        self.initialized = true;
    }

    pub fn quit(&mut self) {
        self.quit_flag = true;
    }
}
