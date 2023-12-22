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

    let mut eaten_fruit_points: Vec<(f64, f64)> = Vec::new();
    for point in &app.snake_points {
        if point.2 {
            eaten_fruit_points.push((point.0 as f64, point.1 as f64));
        }
    }

    let mut uneaten_fruit_points: Vec<(f64, f64)> = Vec::new();
    if let Some(uneaten_fruit) = app.uneaten_fruit {
        uneaten_fruit_points.push((uneaten_fruit.0 as f64, uneaten_fruit.1 as f64));
    }

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
                if !uneaten_fruit_points.is_empty() {
                    ctx.draw(&Points {
                        coords: &uneaten_fruit_points,
                        color: Color::Blue,
                    });
                }
                if !eaten_fruit_points.is_empty() {
                    ctx.draw(&Points {
                        coords: &eaten_fruit_points,
                        color: Color::Green,
                    });
                }
            }),
        frame.size(),
    );
}
