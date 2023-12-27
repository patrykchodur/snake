use crossterm::event::KeyCode;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use snake::app::{App, AppResult};
use snake::event::{Event, EventHandler};
use snake::tui::Tui;
use std::io;

fn main() -> AppResult<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // Create an application.
    let map_size = (20, 20);
    let mut app = App::from_size(map_size);
    let screen_size = app.map_size;
    let mut test_points = Vec::new();
    for x in 0..screen_size.0 {
        for y in 0..screen_size.1 {
            if x % 2 == y % 2 {
                let is_edge = x == 0 || y == 0 || x == screen_size.0 - 1 || y == screen_size.1 - 1;
                test_points.push((x, y, is_edge));
            }
        }
    }
    app.snake_points = test_points;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    app.running = false
                }
            }
            _ => (),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
