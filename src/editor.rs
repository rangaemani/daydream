use crate::app::{self, AppState};
use kiro_editor::Editor;
use ratatui::prelude::*;
use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::fs::File;
use std::io;
use std::rc::Rc;

// TODO implement editor logic for creating/reading/writing to/tagging journal entries
// features:
// - implement text editor (notepad)
// - implement file handling system (naming (titling), searching, saving, etc.)
// - implement tagging system (user generated tags)
// controls:
// - keyboard input
// - arrow key directional navigation
// - `Ctrl-S` save and quit

pub fn save_file(app: &mut AppState, file: File) {
    unimplemented!()
}

pub fn draw_editor(app: &mut AppState, frame: &mut Frame) {
    let app_area = frame.size();

    let editor_area = Rect {
        x: app_area.x
            + if app_area.height / 9 == 0 {
                1
            } else {
                app_area.height / 9
            },
        y: app_area.y
            + if app_area.height / 7 == 0 {
                1
            } else {
                app_area.height / 7
            },
        height: if app_area.height > 1 {
            app_area.height - 1
        } else {
            1
        },
        width: if app_area.width > 1 {
            app_area.width - 1
        } else {
            1
        },
    };

    let mut editor = Editor::new(app_area.height, app_area.width);
    editor.insert_str("Sample Journal Text");
    editor.run();

    todo!()
}

fn create_file(app: &mut AppState) {
    //     let dir = Path::new("/entries/");
    //     let entries = find_entry(dir, date);
    // for entry in entries {
    //    println!("{}", entry);
    // }
    //     if(format!("{:?}.md", app.selected_date)){}
}
