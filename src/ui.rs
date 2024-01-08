use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::AppState;
use crate::calendar::draw_calendar;
use crate::editor::draw_editor;

pub fn render_journal_entry(app: &mut AppState, frame: &mut Frame) {
    draw_editor(app, frame);
    frame.render_widget(
        Paragraph::new(format!(
            "
                Write/Edit an entry. Save and Exit with `Ctrl-S`. To Exit without Saving use `Ctrl-C`
            "
        ))
        .block(
            Block::default()
                .title("『daydream』- editor")
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        ),
        frame.size(),
    );
    todo!()
}

pub fn render_journal_calendar(app: &mut AppState, frame: &mut Frame) {
    draw_calendar(app, frame);
    frame.render_widget(
        Paragraph::new(format!(
            "
            Press `Esc`, `Ctrl-C`, or `q` to halt program. ﾉ(._.`ﾉ)\n
            Select a day using the arrow keys.\n
            (ヘ･_･)ヘ Use the Enter `↵` key to select an entry to edit.\n            
        "
        ))
        .block(
            Block::default()
                .title("『daydream』")
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Rgb(200, 200, 255)))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
