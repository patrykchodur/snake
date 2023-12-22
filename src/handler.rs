use crate::app::{App, AppResult, SnakeDirection};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit()?;
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()?;
            }
        }
        // Snake direction handlers
        KeyCode::Right => {
            app.change_direction(SnakeDirection::Right)?;
        }
        KeyCode::Left => {
            app.change_direction(SnakeDirection::Left)?;
        }
        KeyCode::Up => {
            app.change_direction(SnakeDirection::Up)?;
        }
        KeyCode::Down => {
            app.change_direction(SnakeDirection::Down)?;
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
