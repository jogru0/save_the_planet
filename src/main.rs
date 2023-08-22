use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use save_the_planet::app::{App, AppResult};
use save_the_planet::event::{Event, EventHandler};
use save_the_planet::handler::handle_key_events;
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
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        let seconds = delta.as_secs();
        delta -= Duration::from_secs(seconds);

        app.counter += app.buyers * seconds as u128;

        last_frame = this_frame;

        if app.buyers >= 1000 {
            app.running = false;
        }
    }

    // Exit the user interface.
    tui.exit()?;

    if app.buyers >= 1000 {
        println!("You win!");
    }

    Ok(())
}
