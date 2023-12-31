use ratatui::{prelude::*, widgets::Paragraph};

use crate::app::App;

pub fn render_info_panel(app: &App, frame: &mut Frame, area: Rect) {
    let logo = "
 ______         __        ____          __      
/_  __/_ ______/ /  ___  / __/__  ___ _/ /_____ 
 / / / // / __/ _ \\/ _ \\_\\ \\/ _ \\/ _ `/  '_/ -_)
/_/  \\_,_/_/ /_.__/\\___/___/_//_/\\_,_/_/\\_\\\\__/ 
";
    frame.render_widget(Paragraph::new(logo).alignment(Alignment::Center), area);
    frame.render_widget(Paragraph::new(format!("Score: {} points", app.score)).alignment(Alignment::Center), area);
}
