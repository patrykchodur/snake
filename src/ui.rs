use ratatui::prelude::*;

use crate::app::App;

mod board_pane;
mod generic_popup;
mod info_pane;
mod lost_pane;
mod ui_utils;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(app.map_size.1 as u16 * 1),
            Constraint::Max(8),
        ])
        .split(frame.size());
    board_pane::render_game_board(app, frame, layout[0]);
    info_pane::render_info_panel(app, frame, layout[1]);
    if !app.is_alive {
        lost_pane::render_lost_pane(app, frame, layout[0]);
    }
}
