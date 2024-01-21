use crate::{
    app::{AppState, Mode},
    editor::update_editor,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use time::{Date, Duration};
use tui_textarea::{Input, Key};

pub trait UpdateEvent {
    fn update(&self, app: &mut AppState);
}
impl UpdateEvent for KeyEvent {
    fn update(&self, app: &mut AppState) {
        match app.mode {
            Mode::CALENDAR => self.process_calendar_input(app),
            Mode::EDITOR => self.process_editor_input(app),
            Mode::SORT => self.process_sort_input(app),
        }
    }
}
pub trait EventProcessorExtension {
    fn process_calendar_input(&self, app: &mut AppState);
    fn process_editor_input(&self, app: &mut AppState);
    fn process_sort_input(&self, app: &mut AppState);
    fn convert_to_editor_input(&self) -> Option<Input>;
}
impl EventProcessorExtension for KeyEvent {
    fn process_calendar_input(&self, app: &mut AppState) {
        match self.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if self.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                app.quit()
            }
            KeyCode::Right => {
                if self.modifiers.contains(KeyModifiers::CONTROL) {
                    app.selected_date = app
                        .selected_date
                        .checked_add(Duration::WEEK * 4)
                        .unwrap_or(Date::MIN)
                } else {
                    app.selected_date = app.selected_date.next_day().unwrap_or(Date::MIN)
                }
            }
            KeyCode::Up => {
                if self.modifiers.contains(KeyModifiers::CONTROL) {
                    app.selected_date = app
                        .selected_date
                        .checked_sub(Duration::WEEK * 17)
                        .unwrap_or(Date::MIN)
                } else {
                    app.selected_date = app
                        .selected_date
                        .checked_sub(Duration::WEEK)
                        .unwrap_or(Date::MIN)
                }
            }
            KeyCode::Left => {
                if self.modifiers.contains(KeyModifiers::CONTROL) {
                    app.selected_date = app
                        .selected_date
                        .checked_sub(Duration::WEEK * 4)
                        .unwrap_or(Date::MIN)
                } else {
                    app.selected_date = app.selected_date.previous_day().unwrap_or(Date::MIN)
                }
            }
            KeyCode::Down => {
                if self.modifiers.contains(KeyModifiers::CONTROL) {
                    app.selected_date = app
                        .selected_date
                        .checked_add(Duration::WEEK * 17)
                        .unwrap_or(Date::MIN)
                } else {
                    app.selected_date = app
                        .selected_date
                        .checked_add(Duration::WEEK)
                        .unwrap_or(Date::MIN)
                }
            }
            KeyCode::Enter => {
                app.mode = Mode::EDITOR;
                slog::info!(app.logger, "Mode Changed"; "mode" => %app.mode);
            }
            _ => {}
        }
    }
    fn process_editor_input(&self, app: &mut AppState) {
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            match self.code {
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    app.save();
                    app.mode = Mode::CALENDAR;
                    slog::info!(app.logger, "Mode Changed"; "mode" => %app.mode);
                }
                KeyCode::Char('c') | KeyCode::Char('C') => app.quit(),
                _ => {}
            }
        } else {
            if self.code == KeyCode::Esc {
                app.save();
                app.mode = Mode::CALENDAR;
                slog::info!(app.logger, "Mode Changed"; "mode" => %app.mode);
            }
            let editor_input = self.convert_to_editor_input();
            if let Some(input) = editor_input {
                update_editor(app, input);
            }
        }
    }

    fn process_sort_input(&self, _app: &mut AppState) {
        todo!()
    }

    fn convert_to_editor_input(&self) -> Option<Input> {
        Some(Input {
            key: match self.code {
                KeyCode::Backspace => Key::Backspace,
                KeyCode::Char(c) => Key::Char(c),
                KeyCode::Delete => Key::Delete,
                KeyCode::Down => Key::Down,
                KeyCode::End => Key::End,
                KeyCode::Enter => Key::Enter,
                KeyCode::F(f) => Key::F(f),
                KeyCode::Home => Key::Home,
                KeyCode::Left => Key::Left,
                KeyCode::PageDown => Key::PageDown,
                KeyCode::PageUp => Key::PageUp,
                KeyCode::Right => Key::Right,
                KeyCode::Tab => Key::Tab,
                KeyCode::Up => Key::Up,
                KeyCode::Null => Key::Null,
                _ => return None, // Ignore other keys
            },
            ctrl: self.modifiers.contains(KeyModifiers::CONTROL),
            alt: self.modifiers.contains(KeyModifiers::ALT),
            shift: self.modifiers.contains(KeyModifiers::SHIFT),
        })
    }
}
impl UpdateEvent for MouseEvent {
    fn update(&self, _app: &mut AppState) {
        match self.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // let (_x, _y): (i16, i16) = (self.column as i16, self.row as i16);
                // // map clicked coords to a calendar date
                // // let clicked_date = map_to_date(app, x, y);
                // // update AppState with selected date
                // app.selected_date = OffsetDateTime::now_utc().date();
                // app.mode = Mode::EDITOR;
            }
            _ => {}
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::app::{AppState, Mode};
    use crate::update::UpdateEvent;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use time::OffsetDateTime;
    use tui_textarea::TextArea;
    pub fn get_last_character(text_area: &TextArea) -> Option<char> {
        text_area.lines().last()?.chars().last()
    }
    #[test]
    fn test_key_event_update() {
        let mut app = AppState::new();

        // Test all key codes and modifiers in CALENDAR mode
        app.mode = Mode::CALENDAR;
        for (key_code, modifier) in vec![
            (KeyCode::Esc, KeyModifiers::NONE),
            (KeyCode::Char('q'), KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::NONE),
            (KeyCode::Char('C'), KeyModifiers::NONE),
            (KeyCode::Right, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::CONTROL),
            (KeyCode::Char('C'), KeyModifiers::CONTROL),
        ] {
            let key_event = KeyEvent::new(key_code, modifier);
            key_event.update(&mut app);
            // Add assertions here based on expected behavior
            match (key_code, modifier) {
                (KeyCode::Esc, _) | (KeyCode::Char('q'), _) => assert!(app.quit_flag),
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('C'), KeyModifiers::CONTROL) => assert!(app.quit_flag),
                (KeyCode::Right, _) | (KeyCode::Up, _) => {
                    assert!(app.selected_date > OffsetDateTime::now_local().unwrap().date())
                }
                (KeyCode::Left, _) | (KeyCode::Down, _) => {
                    assert!(app.selected_date < OffsetDateTime::now_local().unwrap().date())
                }
                (KeyCode::Enter, _) => assert_eq!(app.mode, Mode::EDITOR),
                _ => {}
            }
        }

        // Test all key codes and modifiers in EDITOR mode
        app.mode = Mode::EDITOR;
        for (key_code, modifier) in vec![
            (KeyCode::Char('S'), KeyModifiers::CONTROL),
            (KeyCode::Char('s'), KeyModifiers::CONTROL),
            (KeyCode::Char('C'), KeyModifiers::CONTROL),
            (KeyCode::Char('c'), KeyModifiers::CONTROL),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Right, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Delete, KeyModifiers::NONE),
            (KeyCode::Backspace, KeyModifiers::NONE),
            (KeyCode::Home, KeyModifiers::NONE),
            (KeyCode::End, KeyModifiers::NONE),
        ] {
            let key_event = KeyEvent::new(key_code, modifier);
            key_event.update(&mut app);
            // Add assertions here based on expected behavior
            match (key_code, modifier) {
                (KeyCode::Char('S'), KeyModifiers::CONTROL)
                | (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    assert_eq!(app.mode, Mode::CALENDAR)
                }
                (KeyCode::Char('C'), KeyModifiers::CONTROL)
                | (KeyCode::Char('c'), KeyModifiers::CONTROL) => assert!(app.quit_flag),
                (KeyCode::Char(_), _) => {
                    assert_eq!('a', get_last_character(&app.editor.clone()).unwrap_or('\0'))
                }
                _ => {}
            }
        }

        // Test all key codes and modifiers in SORT mode
        app.mode = Mode::SORT;
        for (key_code, modifier) in vec![/* Add all relevant key codes and modifiers here */] {
            let key_event = KeyEvent::new(key_code, modifier);
            key_event.update(&mut app);
            // Add assertions here based on expected behavior
        }
    }
}
