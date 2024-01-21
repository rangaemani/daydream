use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::AppState;
use crate::calendar::draw_calendar;
use crate::editor::draw_editor;

pub fn render_journal_entry(app: &mut AppState, frame: &mut Frame) {
    let frame_size = frame.size();

    let constraints = vec![
        Constraint::Percentage(12), // Adjust this value to allocate more space for the title
        Constraint::Percentage(85),
    ];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame_size);

    let title_area = layout[0];
    let _editor_area = layout[1];

    draw_editor(app, frame);

    // Render the title
    frame.render_widget(
        Paragraph::new(
            "write/edit an entry 
        save and exit with `Ctrl-S` or `Esc`
        to exit without saving use `Ctrl-C`",
        )
        .block(
            Block::default()
                .title(format!("『daydream』- editor"))
                .border_style(Style::new().light_green())
                .borders(Borders::all())
                .border_type(BorderType::QuadrantOutside)
                .borders(Borders::all())
                .title_alignment(Alignment::Left),
        )
        .alignment(Alignment::Center),
        title_area,
    );
}

pub fn render_journal_calendar(app: &mut AppState, frame: &mut Frame) {
    let frame_size = frame.size();

    let constraints = vec![
        Constraint::Percentage(15), // Adjust this value to allocate more space for the titl
        Constraint::Percentage(85), // Adjust this value to allocate more space for the titl
    ];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame_size);

    let title_area = layout[0];
    // let calendar_area = layout[1];

    draw_calendar(app, frame);

    // Render the title
    frame.render_widget(
        Paragraph::new("Press `Esc`, `Ctrl-C`, or `q` to halt program. ﾉ(._.ﾉ)\nSelect a day using the arrow keys.\n(/ ･〰･)／ Use the `↵` key to select an entry to edit.")
            .block(Block::default().title("『daydream』").border_style(Style::new().light_green()).borders(Borders::all()).border_type(BorderType::QuadrantOutside))
            .alignment(Alignment::Center),
        title_area,
    );
}
