use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::AppState;
use crate::calendar::draw;

pub fn render(app: &mut AppState, frame: &mut Frame) {
    draw(app, frame);
    frame.render_widget(
        Paragraph::new(format!(
            "
            Press `Esc`, `Ctrl-C`, or `q` to halt program.\n
            Select a day to browse/edit an entry.\n            
        "
        ))
        .block(
            Block::default()
                .title("in-terminal journal")
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
