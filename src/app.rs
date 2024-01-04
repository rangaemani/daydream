use crate::calendar::DatePosition;
use time::{Date, Month, OffsetDateTime};
// App state
pub struct AppState {
    pub selected_date: Date,
    pub quit_flag: bool,
    pub date_pos_map: Vec<DatePosition>,
}

impl AppState {
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
            date_pos_map: Vec::new(),
        }
    }
    // handles terminal tick events
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.quit_flag = true;
    }
}
