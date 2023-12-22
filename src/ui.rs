use log::debug;
use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{canvas::*, Block, Borders},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    /* Original rendering of frame
    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
        .block(
            Block::default()
                .title("Template")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        frame.size(),
    )
    */

    // Unfortunately we have to set the app size here,
    // because that's the only place we can know the true size
    let screen_size = (
        frame.size().width as usize - 2,
        (frame.size().height as usize - 2) * 2,
    );
    app.set_screen_size(screen_size)
        .expect("Couldn't set the screen_size in render function");

    // calculate the snake points
    let snake_points: Vec<(f64, f64)> = app
        .snake_points
        .iter()
        .map(|point| (point.0 as f64, point.1 as f64))
        .collect();

    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Snake"))
            .marker(Marker::HalfBlock)
            .x_bounds([0.0, screen_size.0 as f64])
            .y_bounds([0.0, screen_size.1 as f64])
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &snake_points,
                    color: Color::Red,
                });
                /*
                ctx.draw(&Line {
                    x1: 10.0,
                    y1: 10.0,
                    x2: 40.0,
                    y2: 40.0,
                    color: Color::Red,
                });
                */
            }),
        frame.size(),
    );
    debug!("Frame size: {:?}", frame.size());
}
