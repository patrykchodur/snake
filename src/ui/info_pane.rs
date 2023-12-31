use ratatui::{prelude::*, widgets::Paragraph};

use crate::app::App;

pub fn render_info_panel(app: &App, frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new(format!("Score: {} points", app.score)), area);
}
