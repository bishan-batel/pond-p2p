use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let title_block = Block::new()
        .borders(Borders::all())
        .style(Style::default())
        .border_type(BorderType::Rounded);

    let title = Paragraph::new(Text::styled(
        "Create new json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, frame.area());
}
