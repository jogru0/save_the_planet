use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::Backend;

use crate::input::{Event as MyEvent, Input, Key};

use super::{
    app::{AppResult, TuiState},
    event::Event,
    tui_impl::Tui,
};

impl TryFrom<KeyCode> for Key {
    fn try_from(key: KeyCode) -> Result<Key, ()> {
        match key {
            KeyCode::Char('A') => Ok(Key::A),
            KeyCode::Char('B') => Ok(Key::B),
            KeyCode::Char('C') => Ok(Key::C),
            KeyCode::Char('D') => Ok(Key::D),
            KeyCode::Char('E') => Ok(Key::E),
            KeyCode::Char('F') => Ok(Key::F),
            KeyCode::Char('G') => Ok(Key::G),
            KeyCode::Char('H') => Ok(Key::H),
            KeyCode::Char('I') => Ok(Key::I),
            KeyCode::Char('J') => Ok(Key::J),
            KeyCode::Char('K') => Ok(Key::K),
            KeyCode::Char('L') => Ok(Key::L),
            KeyCode::Char('M') => Ok(Key::M),
            KeyCode::Char('N') => Ok(Key::N),
            KeyCode::Char('O') => Ok(Key::O),
            KeyCode::Char('P') => Ok(Key::P),
            KeyCode::Char('Q') => Ok(Key::Q),
            KeyCode::Char('R') => Ok(Key::R),
            KeyCode::Char('S') => Ok(Key::S),
            KeyCode::Char('T') => Ok(Key::T),
            KeyCode::Char('U') => Ok(Key::U),
            KeyCode::Char('V') => Ok(Key::V),
            KeyCode::Char('W') => Ok(Key::W),
            KeyCode::Char('X') => Ok(Key::X),
            KeyCode::Char('Y') => Ok(Key::Y),
            KeyCode::Char('Z') => Ok(Key::Z),

            KeyCode::Char('a') => Ok(Key::A),
            KeyCode::Char('b') => Ok(Key::B),
            KeyCode::Char('c') => Ok(Key::C),
            KeyCode::Char('d') => Ok(Key::D),
            KeyCode::Char('e') => Ok(Key::E),
            KeyCode::Char('f') => Ok(Key::F),
            KeyCode::Char('g') => Ok(Key::G),
            KeyCode::Char('h') => Ok(Key::H),
            KeyCode::Char('i') => Ok(Key::I),
            KeyCode::Char('j') => Ok(Key::J),
            KeyCode::Char('k') => Ok(Key::K),
            KeyCode::Char('l') => Ok(Key::L),
            KeyCode::Char('m') => Ok(Key::M),
            KeyCode::Char('n') => Ok(Key::N),
            KeyCode::Char('o') => Ok(Key::O),
            KeyCode::Char('p') => Ok(Key::P),
            KeyCode::Char('q') => Ok(Key::Q),
            KeyCode::Char('r') => Ok(Key::R),
            KeyCode::Char('s') => Ok(Key::S),
            KeyCode::Char('t') => Ok(Key::T),
            KeyCode::Char('u') => Ok(Key::U),
            KeyCode::Char('v') => Ok(Key::V),
            KeyCode::Char('w') => Ok(Key::W),
            KeyCode::Char('x') => Ok(Key::X),
            KeyCode::Char('y') => Ok(Key::Y),
            KeyCode::Char('z') => Ok(Key::Z),

            KeyCode::Left => Ok(Key::Left),
            KeyCode::Up => Ok(Key::Up),
            KeyCode::Right => Ok(Key::Right),
            KeyCode::Down => Ok(Key::Down),

            KeyCode::Char('0') => Ok(Key::Number0),
            KeyCode::Char('1') => Ok(Key::Number1),
            KeyCode::Char('2') => Ok(Key::Number2),
            KeyCode::Char('3') => Ok(Key::Number3),
            KeyCode::Char('4') => Ok(Key::Number4),
            KeyCode::Char('5') => Ok(Key::Number5),
            KeyCode::Char('6') => Ok(Key::Number6),
            KeyCode::Char('7') => Ok(Key::Number7),
            KeyCode::Char('8') => Ok(Key::Number8),
            KeyCode::Char('9') => Ok(Key::Number9),

            _ => {
                //log(format!("unrecognized key {key:?}"));
                Err(())
            }
        }
    }

    type Error = ();
}

pub fn receive_input<B: Backend>(ctx: &Tui<B>, state: &mut TuiState) -> AppResult<Input> {
    match ctx.events.next()? {
        Event::Tick => state.tick(),
        Event::Key(key_event) => return handle_key_events(key_event, state),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
    };

    Ok(Input {
        event: None,
        mouse_x: 0,
        mouse_y: 0,
        fps: 0.0,
    })
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut TuiState) -> AppResult<Input> {
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
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }

    let event = Key::try_from(key_event.code).ok().map(MyEvent::Key);

    Ok(Input {
        event,
        mouse_x: 0,
        mouse_y: 0,
        fps: 0.0,
    })
}
