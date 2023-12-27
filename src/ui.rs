use ratatui::{
    prelude::*,
    style::Color,
    symbols::Marker,
    widgets::{canvas::*, Block, BorderType, Borders, Clear, Paragraph},
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
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Snake")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .marker(Marker::HalfBlock)
            // Not sure about the reason for -1. Maybe it's a library bug or whatever
            // but it's related to borders
            .x_bounds([0.0 - 1.0, screen_size.0 as f64 - 1.0])
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

    if !app.is_alive {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let text = vec![
            ratatui::prelude::Line::from(""),
            ratatui::prelude::Line::from("You've lost!"),
            ratatui::prelude::Line::from(""),
            ratatui::prelude::Line::from("Press 'q' to exit"),
        ];
        let area = centered_rect(30, 7, frame.size());
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph.block(block), area);
    }
}

pub fn render_screen_test(_app: &mut App, frame: &mut Frame) {
    let screen_size = (
        frame.size().width as usize - 2,
        (frame.size().height as usize - 2) * 2,
    );

    let mut test_points = Vec::new();
    for x in 0..screen_size.0 {
        for y in 0..screen_size.1 {
            if x % 2 == y % 2 {
                test_points.push((x as f64, y as f64));
            }
        }
    }
    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Snake"))
            .marker(Marker::HalfBlock)
            .x_bounds([0.0 - 1.0, -1.0 + screen_size.0 as f64])
            .y_bounds([0.0, screen_size.1 as f64])
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &test_points,
                    color: Color::Red,
                });
            }),
        frame.size(),
    );
}

/// helper function to create a centered rect using up fixed width inside rectangle `r`
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let horizontal_margin = r.width - width;
    let vertical_margin = r.height - height;
    let left_horizontal_margin = horizontal_margin/2;
    let left_vertical_margin = vertical_margin/2;
    Rect {
        x: r.x + left_horizontal_margin,
        y: r.y + left_vertical_margin,
        width,
        height,
    }
}
