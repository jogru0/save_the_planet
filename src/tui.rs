use std::io;

use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};

use crate::grid::{Cell, Grid};

use self::{
    app::{AppResult, TuiState},
    event::EventHandler,
    handler::receive_input,
    tui_impl::Tui,
};

// pub fn draw(ctx: &mut T, app: &mut TuiState) -> AppResult<()> {
//     self.terminal.draw(|frame| ui::render(app, frame))?;
//     Ok(())
// }

fn draw_grid<B: Backend>(ctx: &mut Terminal<B>, grid: Grid<Cell>) -> AppResult<()> {
    ctx.draw(|frame| ui::render(grid, frame))?;
    Ok(())

    // ctx.console.cls();

    // for y in 0..grid.height() {
    //     for x in 0..grid.width() {
    //         console.print(x as i32, y as i32, grid[y][x].to_string().as_str())
    //     }
    // }
}

// fn receive_input(ctx: &mut BTerm, state: &mut BTermState) -> Input {
//     let mut input = INPUT.lock();
//     let mouse_tile = input.mouse_tile(0);

//     let mut new_key = None;

//     while matches!(new_key, None) {
//         match input.pop() {
//             Some(BEvent::KeyboardInput {
//                 key: vkey,
//                 scan_code: _,
//                 pressed,
//             }) => {
//                 if let Some(key) = Key::from(vkey) {
//                     if pressed {
//                         if state.pressed_keys.insert(key) {
//                             new_key = Some(key);
//                         }
//                     } else {
//                         let was_present = state.pressed_keys.remove(&key);
//                         assert!(was_present)
//                     }
//                 }
//             }
//             Some(BEvent::CloseRequested) => ctx.quit(),
//             None => break,
//             _ => {}
//         }
//     }

//     Input {
//         event: new_key.map(Event::Key),
//         mouse_x: mouse_tile.x as usize,
//         mouse_y: mouse_tile.y as usize,
//         fps: ctx.fps,
//     }
// }

// impl GameState for BTermState {
//     fn tick(&mut self, ctx: &mut BTerm) {
//         let input = receive_input(ctx, self);

//         let grid = self.world.update(input);

//         draw_grid(ctx, grid);
//     }
// }

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui_impl;

/// Event handler.
pub mod handler;

pub fn main() -> AppResult<()> {
    // Create an application.
    let mut app = TuiState::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Handle events.
        let input = receive_input(&tui, &mut app)?;

        let grid = app.simulation.update(&input);

        draw_grid(&mut tui.terminal, grid)?;
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
