use crate::app::AppState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Frame;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, ErrorKind, Write};
use std::str::FromStr;
use tui_textarea::Input;

/// Runs the first time the editor is opened, populates editor with file contents.
///
/// # Params
///
/// * `app` - Mutable reference to the application state.
/// * `frame` - Mutable reference to the current UI frame.
/// * `editor_area` - The area of the UI where the editor is displayed.
///
/// # Returns
///
/// Result<(), Box<dyn std::error::Error>>
pub fn initialize_editor<'a>(
    app: &'a mut AppState,
    frame: &mut Frame,
    editor_area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    app.initialized();

    let file = match get_entry_file_handle(app) {
        Ok(file) => file,
        Err(e) => {
            slog::error!(
                app.logger,
                "Failed to create or open file while initializing editor: {}",
                e
            );
            panic!();
            // return Err(Box::new(e));
        }
    };
    app.editor = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<_>>()?;
    app.editor.move_cursor(tui_textarea::CursorMove::Bottom);
    let widget = app.editor.widget();
    frame.render_widget(widget, editor_area);
    Ok(())
}

/// Accepts a key input and pushes it to the editor.
///
/// # Params
///
/// * `app` - Mutable reference to the application state.
/// * `input_key` - The input key to handle.
///
/// # Returns
///
/// void
pub fn update_editor(app: &mut AppState, input_key: Input) {
    app.editor.input(input_key);
}

/// Draws the editor interface on the terminal.
///
/// # Params
///
/// * `app` - Mutable reference to the application state.
/// * `frame` - Mutable reference to the current UI frame.
///
/// # Returns
///
/// void
pub fn draw_editor(app: &mut AppState, frame: &mut Frame) {
    let frame_size = frame.size();

    let constraints = vec![Constraint::Percentage(15), Constraint::Percentage(85)];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame_size);

    let editor_area = layout[1];

    if !app.initialized {
        let _result = match initialize_editor(app, frame, editor_area) {
            Ok(()) => Ok(()),
            Err(_e) => Err(()),
        };
    } else {
        let none_string = "standard day".to_string();
        let current_holiday = app
            .holiday_info
            .as_ref()
            .and_then(|holiday_info| holiday_info.holidays.get(&app.selected_date))
            .map(|holiday_name| (app.selected_date, holiday_name.clone()))
            .unwrap_or((app.selected_date, none_string));

        let _select_string: String = String::from_str("Selected Day ").unwrap();

        let editor_block = match &*current_holiday.1 {
            select_string if select_string == "Selected Day " => Block::default()
                .title(format!(
                    "『{}』☾ ﾟ｡⋆๑꒰⨳  ∟  ⨳๑ ꒱☁︎｡ﾟ｡。zｚℤＺ　",
                    app.selected_date
                ))
                .border_style(Style::default().fg(Color::Rgb(255, 225, 120)))
                .borders(Borders::all())
                .border_type(BorderType::Double),
            _ => Block::default()
                .title(format!("happy {}", current_holiday.1))
                .border_style(Style::default().fg(Color::Rgb(255, 255, 160)))
                .borders(Borders::all())
                .border_type(BorderType::Double),
        };
        app.editor.set_block(editor_block);
        frame.render_widget(app.editor.widget(), editor_area);
    }
}

/// Writes to a file the given text area's content.
///
/// # Params
///
/// * `path` - The file system path to write to.
/// * `textarea` - The text area containing lines of text to write.
///
/// # Returns
///
/// io::Result<()>
pub fn write_to_file(app: &mut AppState) -> io::Result<()> {
    let editor_content = app.editor.lines().concat();
    app.editor_text = editor_content.clone();
    let mut file = get_entry_file_handle(app)?;
    file.write_all(editor_content.as_bytes())?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

/// Creates a file for a selected date or opens one if it already exists.
///
/// # Params
///
/// * `app` - Mutable reference to the application state.
///
/// # Returns
///
/// io::Result<File>
fn get_entry_file_handle(app: &mut AppState) -> io::Result<File> {
    let path = app
        .entries_dir
        .join(format!("{}_entry.md", app.selected_date));
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_file() {
                slog::info!(app.logger, "Opening existing file"; "path" => ?path);
                OpenOptions::new().read(true).write(true).open(&path)
            } else {
                slog::error!(app.logger, "Path exists but is not a file"; "path" => ?path);
                Err(io::Error::new(
                    ErrorKind::Other,
                    "Path exists but is not a file",
                ))
            }
        }
        Err(_e) => {
            slog::info!(app.logger, "Creating new file"; "path" => ?path);
            let mut file = OpenOptions::new().create(true).write(true).open(&path)?;
            match file.write_all(format!("# Daily Entry - 『{}』\n", app.selected_date).as_bytes())
            {
                Ok(_) => {
                    slog::info!(app.logger, "Inserted daily entry heading"; "date" => %app.selected_date);
                    file.write_all(b"\n")?;
                    file.sync_all()?;
                    file.flush()?;
                    slog::info!(app.logger, "File sync completed"; "path" => ?path);
                    // // Additional check to confirm file contents
                    // let mut contents = String::new();
                    // file.read_to_string(&mut contents)?;
                    // slog::info!(app.logger, "File contents after write"; "contents" => %contents);
                }
                Err(e) => {
                    slog::error!(app.logger, "Failed to write entry heading to file"; "error" => %e, "path" => ?path);
                }
            }
            Ok(file)
        }
    }
}
