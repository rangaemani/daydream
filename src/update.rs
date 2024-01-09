use crate::{
    app::{AppState, Mode},
    calendar::map_to_date,
    editor::update_editor,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
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
                        // initialize_editor(app);
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
                    let editor_input = KeyCode::Left;
                    update_editor(app, editor_input);
                }
                KeyCode::Right => {
                    let editor_input = KeyCode::Right;
                    update_editor(app, editor_input);
                }
                KeyCode::Up => {
                    let editor_input = KeyCode::Up;
                    update_editor(app, editor_input);
                }
                KeyCode::Down => {
                    let editor_input = KeyCode::Down;
                    update_editor(app, editor_input);
                }
                KeyCode::Char(c) => {
                    let editor_input = KeyCode::Char(c);
                    update_editor(app, editor_input);
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    let editor_input = KeyCode::Backspace;
                    update_editor(app, editor_input);
                }
                KeyCode::Home => {
                    let editor_input = KeyCode::Home;
                    update_editor(app, editor_input);
                }
                KeyCode::End => {
                    let editor_input = KeyCode::End;
                    update_editor(app, editor_input);
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
