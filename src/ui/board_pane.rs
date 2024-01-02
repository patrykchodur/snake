use ratatui::{
    prelude::*,
    style::Color,
    symbols::Marker,
    widgets::{canvas::*, Block, BorderType, Borders},
};

use super::generic_popup::*;
use super::ui_utils::*;
use crate::app::App;

pub fn render_game_board(app: &App, frame: &mut Frame, area: Rect) {
    let screen_size = app.map_size;
    // Get the size of block required for displaying map
    // We expect the size to be non-negative. If it is, it should have been checked earlier
    let ui_block_size =
        calculate_ui_block_size(screen_size, true).expect("The map size is negative");
    let board_area = match centered_rect(ui_block_size.0, ui_block_size.1, area) {
        Ok(ba) => ba,
        Err(_er) => {
            // If we get an error here we can't really do anything else
            let _ = render_popup(app, frame, vec!["Please resize the screen".into()], area);
            return ();
        }
    };

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
        board_area,
    );
}
