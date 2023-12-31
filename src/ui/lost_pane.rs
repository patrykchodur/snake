use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use super::ui_utils::*;
use crate::app::App;

pub fn render_lost_pane(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let text = vec![
        ratatui::prelude::Line::from(""),
        ratatui::prelude::Line::from("You've lost!"),
        ratatui::prelude::Line::from(""),
        ratatui::prelude::Line::from(format!("Score: {}", app.score)),
        ratatui::prelude::Line::from(""),
        ratatui::prelude::Line::from("Press 'q' to exit, 'r' to restart"),
    ];
    let area = centered_rect(38, 9, area);
    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph.block(block), area);
}
