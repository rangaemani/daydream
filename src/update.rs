use crate::{
    app::{AppState, Mode},
    calendar::map_to_date,
};
use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
};
use time::Date;

pub trait UpdateEvent {
    fn update(&self, app: &mut AppState);
}

// respond to input events differently based on which mode is currently engaged
impl UpdateEvent for KeyEvent {
    fn update(&self, app: &mut AppState) {
        match app.mode {
            Mode::CALENDAR => {
                match self.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if self.modifiers == KeyModifiers::CONTROL {
                            app.quit()
                        }
                    }
                    KeyCode::Right | KeyCode::Up => {
                        app.selected_date = app.selected_date.next_day().unwrap_or(Date::MIN);
                    }
                    KeyCode::Left | KeyCode::Down => {
                        app.selected_date = app.selected_date.previous_day().unwrap_or(Date::MIN);
                    }
                    KeyCode::Enter => {
                        app.mode = Mode::EDITOR;
                    }
                    _ => {}
                };
            }
            Mode::EDITOR => match self.code {
                KeyCode::Char('S') | KeyCode::Char('s') => {
                    if self.modifiers == KeyModifiers::CONTROL {
                        app.save();
                        app.mode = Mode::CALENDAR;
                    }
                }
                KeyCode::Char('C') | KeyCode::Char('c') => {
                    if self.modifiers == KeyModifiers::CONTROL {
                        app.quit()
                    }
                }
                KeyCode::Left => {
                    unimplemented!()
                }
                KeyCode::Right => {
                    unimplemented!()
                }
                KeyCode::Up => {
                    unimplemented!()
                }
                KeyCode::Down => {
                    unimplemented!()
                }
                _ => unimplemented!(),
            },
            Mode::SORT => {
                todo!()
            }
        }
    }
}

impl UpdateEvent for MouseEvent {
    fn update(&self, app: &mut AppState) {
        match self.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let (x, y): (i16, i16) = (self.column as i16, self.row as i16);
                // map clicked coords to a calendar date
                let clicked_date = map_to_date(app, x, y).unwrap();
                // update AppState with selected date
                app.selected_date = clicked_date;
                app.mode = Mode::EDITOR;
            }
            _ => {}
        }
    }
}
