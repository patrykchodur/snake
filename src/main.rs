use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use snake::app::{App, AppResult};
use snake::event::{Event, EventHandler};
use snake::handler::handle_key_events;
use snake::tui::Tui;
use std::io;

fn main() -> AppResult<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let map_size = (20, 20);
    // Create an application.
    let mut app = App::from_size(map_size);

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
            Event::Tick => app.tick()?,
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
