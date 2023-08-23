use crate::app::{cards::Achievements, App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Enter => {
            app.increment_counter();
        }

        KeyCode::Down => app.cards.next(),
        KeyCode::Up => app.cards.previous(),

        // Other handlers you could add here.
        KeyCode::Char('a') => {
            app.add_buyer();
        }

        // Other handlers you could add here.
        KeyCode::Char('A') => {
            if app.cards.achievements.is_none() {
                app.cards.achievements = Some(Achievements::new())
            }
        }

        _ => {}
    }
    Ok(())
}
