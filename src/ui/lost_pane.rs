use ratatui::prelude::*;

use super::generic_popup::*;
use crate::app::App;

pub fn render_lost_pane(app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        ratatui::prelude::Line::from(""),
        ratatui::prelude::Line::from("You've lost!"),
        ratatui::prelude::Line::from(""),
        ratatui::prelude::Line::from(format!("Score: {}", app.score)),
        ratatui::prelude::Line::from(""),
        ratatui::prelude::Line::from("Press 'q' to exit, 'r' to restart"),
        ratatui::prelude::Line::from(""),
    ];
    render_popup(app, frame, text, area).unwrap_or_else(|_| {
        let text = vec![
            ratatui::prelude::Line::from("You've lost!"),
            ratatui::prelude::Line::from(format!("Score: {}", app.score)),
            ratatui::prelude::Line::from("'q'uit or 'r'estart"),
        ];
        let _ = render_popup(app, frame, text, area);
    });
}
