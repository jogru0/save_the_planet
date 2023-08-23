use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use save_the_planet::app::{App, AppResult};
use save_the_planet::event::{Event, EventHandler};
use save_the_planet::tui::Tui;
use std::io;
use std::time::{Duration, Instant};

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let mut last_frame = Instant::now();
    let mut delta = Duration::default();

    // Start the main loop.
    while app.running {
        let this_frame = Instant::now();
        delta += this_frame - last_frame;

        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => app.handle_key_events(key_event)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        if app.key == app.old_key {
            app.key = None
        } else if app.key.is_some() {
            app.old_key = app.key;
        }

        let seconds = delta.as_secs();
        delta -= Duration::from_secs(seconds);

        app.simulate(seconds);

        last_frame = this_frame;
    }

    // Exit the user interface.
    tui.exit()?;

    if app.buyers >= 1000 {
        println!("You win!");
    }

    Ok(())
}
