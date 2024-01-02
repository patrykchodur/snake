use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use super::ui_utils::*;
use crate::app::{App, AppResult};

pub fn render_popup(
    _app: &App,
    frame: &mut Frame,
    text: Vec<ratatui::prelude::Line>,
    area: Rect,
) -> AppResult<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let height = text.len() as u16 + 2;
    let width = 4 + match text.iter().max_by_key(|s| s.width()) {
        Some(i) => i.width() as u16,
        None => 0,
    };
    let area = centered_rect(width, height, area)?;

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph.block(block), area);

    Ok(())
}
