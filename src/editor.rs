use crate::app::AppState;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::prelude::Frame;
use std::fs::*;
use std::io::{self, BufRead};
use std::io::{BufReader, Write};
use std::path::Path;
use tui_textarea::TextArea;

// TODO implement editor logic for creating/reading/writing to/tagging journal entries
// features:
// - implement text editor (notepad)
// - implement file handling system (naming (titling), searching, saving, etc.)
// - implement tagging system (user generated tags)
// controls:
// - keyboard input
// - arrow key directional navigation
// - `Ctrl-S` save and quit

// pub fn save_file(app: &mut AppState, file: File) {
//     unimplemented!()
// }

pub fn draw_editor(app: &mut AppState, frame: &mut Frame) {
    let editor_area = Rect {
        x: frame.size().x
            + if frame.size().height / 9 == 0 {
                1
            } else {
                frame.size().height / 9
            },
        y: frame.size().y
            + if frame.size().height / 7 == 0 {
                1
            } else {
                frame.size().height / 7
            },
        height: if frame.size().height > 1 {
            frame.size().height - 1
        } else {
            1
        },
        width: if frame.size().width > 1 {
            frame.size().width - 1
        } else {
            1
        },
    };
    if !app.initialized {
        initialize_editor(app, frame, editor_area);
    } else {
        frame.render_widget(app.editor.widget(), editor_area);
    }

    // let _current_entry_file = match create_entry_file(app) {
    //     Some(file) => initialize_editor(app, frame),
    //     None => return,
    // };
    todo!()
}

pub fn update_editor(app: &mut AppState, input_key: KeyCode) {
    app.editor.input(Event::Key(input_key.into()));
}

// runs the first time the editor is opened, populates editor with file contents
// @params AppState, Frame // to access date, rendering frame
// @return void
pub fn initialize_editor<'a>(app: &'a mut AppState, frame: &mut Frame, editor_area: Rect) {
    app.initialized();

    let file = match create_entry_file(app) {
        Some(file) => file,
        None => {
            eprintln!("Failed to create or open file");
            return;
        }
    };
    let reader = BufReader::new(file);
    app.editor = TextArea::new(
        reader
            .lines()
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap(),
    );
    let widget = app.editor.widget();
    frame.render_widget(widget, editor_area);
}
// creates file for selected date or opens one if it already exists
// @params AppState // to access date
// @return Option<File>
fn create_entry_file<'a>(app: &mut AppState) -> Option<std::fs::File> {
    let path = format!("/entries/{}_entry.md", app.selected_date);
    if Path::new(&path).exists() {
        let file = std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .write(true)
            .truncate(true)
            .open(path)
            .ok()?;
        return Some(file);
    } else {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .ok()?;

        file.write_all(format!("# Daily Entry - 『{}』\n", app.selected_date).as_bytes())
            .ok()?;
        file.flush().ok()?;
        return Some(file);
    }
}
