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
    let screen_size = app.map_size;
    /* (
        frame.size().width as usize - 2,
        (frame.size().height as usize - 2) * 2,
    );
    app.set_screen_size(screen_size)
        .expect("Couldn't set the screen_size in render function");
                      */

    let ui_block_size = calculate_ui_block_size(screen_size, true);
    let area = centered_rect(ui_block_size.0, ui_block_size.1, frame.size());

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
                    color: Color::Green,
                });
                if !uneaten_fruit_points.is_empty() {
                    ctx.draw(&Points {
                        coords: &uneaten_fruit_points,
                        color: Color::Red,
                    });
                }
                if !eaten_fruit_points.is_empty() {
                    ctx.draw(&Points {
                        coords: &eaten_fruit_points,
                        color: Color::Yellow,
                    });
                }
            }),
        area,
    );

    if !app.is_alive {
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
        let area = centered_rect(38, 9, frame.size());
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph.block(block), area);
    }
}

pub fn render_screen_test(app: &mut App, frame: &mut Frame) {
    let screen_size = app.map_size;
    let mut test_points = Vec::new();
    for x in 0..screen_size.0 {
        for y in 0..screen_size.1 {
            if x % 2 == y % 2 {
                test_points.push((x as f64, y as f64));
            }
        }
    }
    let ui_block_size = calculate_ui_block_size(screen_size, true);
    let area = centered_rect(ui_block_size.0, ui_block_size.1, frame.size());
    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Snake"))
            .marker(Marker::HalfBlock)
            .x_bounds([-1.0, screen_size.0 as f64 - 1.0])
            .y_bounds([0.0, screen_size.1 as f64])
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &test_points,
                    color: Color::Red,
                });
            }),
        area,
    );
}

fn calculate_ui_block_size(map_size: (isize, isize), borders: bool) -> (u16, u16) {
    assert!(map_size.0 > 0 && map_size.1 > 0);
    let border_size = if borders { 2 } else { 0 };
    (
        map_size.0 as u16 + border_size,
        ((map_size.1 as u16 + 1) / 2) + border_size,
    )
}

/// helper function to create a centered rect using up fixed width inside rectangle `r`
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    assert!(width <= r.width && height <= r.height);
    let horizontal_margin = r.width - width;
    let vertical_margin = r.height - height;
    let left_horizontal_margin = horizontal_margin / 2;
    let left_vertical_margin = vertical_margin / 2;
    Rect {
        x: r.x + left_horizontal_margin,
        y: r.y + left_vertical_margin,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ui_block_size() {
        // Regular example: square map and borders
        let map_size = (50, 50);
        let block_size = (52, 27);
        assert_eq!(calculate_ui_block_size(map_size, true), block_size);

        // The height is not even - we need additional block
        // and one pixel will not be used
        let map_size = (50, 49);
        let block_size = (52, 27);
        assert_eq!(calculate_ui_block_size(map_size, true), block_size);

        // Example without borders
        let map_size = (50, 50);
        let block_size = (50, 25);
        assert_eq!(calculate_ui_block_size(map_size, false), block_size);

        // Without borders and uneven height
        let map_size = (50, 50);
        let block_size = (50, 25);
        assert_eq!(calculate_ui_block_size(map_size, false), block_size);
    }

    #[test]
    #[should_panic]
    fn calculate_ui_block_size_negative_x() {
        let map_size = (-50, 50);
        calculate_ui_block_size(map_size, false);
    }

    #[test]
    #[should_panic]
    fn calculate_ui_block_size_negative_y() {
        let map_size = (50, -50);
        calculate_ui_block_size(map_size, false);
    }

    #[test]
    fn check_centered_rect() {
        let my_block = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        let my_inner_block = Rect {
            x: 25,
            y: 25,
            height: 50,
            width: 50,
        };

        assert_eq!(centered_rect(50, 50, my_block), my_inner_block);
    }

    #[test]
    #[should_panic]
    fn centered_rect_invalid_width() {
        let my_block = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        centered_rect(150, 50, my_block);
    }

    #[test]
    #[should_panic]
    fn centered_rect_invalid_height() {
        let my_block = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        centered_rect(50, 150, my_block);
    }
}
