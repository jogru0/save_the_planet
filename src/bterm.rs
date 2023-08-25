use std::collections::HashSet;

use bracket_terminal::prelude::{
    log, main_loop, BError, BEvent, BTerm, BTermBuilder, GameState, VirtualKeyCode,
    BACKEND_INTERNAL, INPUT,
};

use crate::{
    grid::Grid,
    world::{Event, Input, Key, World},
};

struct BTermState {
    pressed_keys: HashSet<Key>,
    world: World,
}

impl BTermState {
    pub fn new() -> Self {
        Self {
            pressed_keys: Default::default(),
            world: World::new(),
        }
    }
}

impl TryFrom<VirtualKeyCode> for Key {
    fn try_from(key: VirtualKeyCode) -> Result<Key, ()> {
        match key {
            VirtualKeyCode::A => Ok(Key::A),
            VirtualKeyCode::B => Ok(Key::B),
            VirtualKeyCode::C => Ok(Key::C),
            VirtualKeyCode::D => Ok(Key::D),
            VirtualKeyCode::E => Ok(Key::E),
            VirtualKeyCode::F => Ok(Key::F),
            VirtualKeyCode::G => Ok(Key::G),
            VirtualKeyCode::H => Ok(Key::H),
            VirtualKeyCode::I => Ok(Key::I),
            VirtualKeyCode::J => Ok(Key::J),
            VirtualKeyCode::K => Ok(Key::K),
            VirtualKeyCode::L => Ok(Key::L),
            VirtualKeyCode::M => Ok(Key::M),
            VirtualKeyCode::N => Ok(Key::N),
            VirtualKeyCode::O => Ok(Key::O),
            VirtualKeyCode::P => Ok(Key::P),
            VirtualKeyCode::Q => Ok(Key::Q),
            VirtualKeyCode::R => Ok(Key::R),
            VirtualKeyCode::S => Ok(Key::S),
            VirtualKeyCode::T => Ok(Key::T),
            VirtualKeyCode::U => Ok(Key::U),
            VirtualKeyCode::V => Ok(Key::V),
            VirtualKeyCode::W => Ok(Key::W),
            VirtualKeyCode::X => Ok(Key::X),
            VirtualKeyCode::Y => Ok(Key::Y),
            VirtualKeyCode::Z => Ok(Key::Z),

            VirtualKeyCode::Left => Ok(Key::Left),
            VirtualKeyCode::Up => Ok(Key::Up),
            VirtualKeyCode::Right => Ok(Key::Right),
            VirtualKeyCode::Down => Ok(Key::Down),

            VirtualKeyCode::Key0 => Ok(Key::Number0),
            VirtualKeyCode::Key1 => Ok(Key::Number1),
            VirtualKeyCode::Key2 => Ok(Key::Number2),
            VirtualKeyCode::Key3 => Ok(Key::Number3),
            VirtualKeyCode::Key4 => Ok(Key::Number4),
            VirtualKeyCode::Key5 => Ok(Key::Number5),
            VirtualKeyCode::Key6 => Ok(Key::Number6),
            VirtualKeyCode::Key7 => Ok(Key::Number7),
            VirtualKeyCode::Key8 => Ok(Key::Number8),
            VirtualKeyCode::Key9 => Ok(Key::Number9),

            _ => {
                log(format!("unrecognized key {key:?}"));
                Err(())
            }
        }
    }

    type Error = ();
}

fn draw_grid(ctx: &mut BTerm, mut grid: Grid<char>) {
    let mut lock = BACKEND_INTERNAL.lock();
    let console = lock.consoles[ctx.active_console].console.as_mut();

    let grid = grid.view();

    assert_eq!(
        console.get_char_size(),
        (grid.width() as u32, grid.height() as u32)
    );

    console.cls();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            console.print(x as i32, y as i32, grid[y][x].to_string().as_str())
        }
    }
}

fn receive_input(ctx: &mut BTerm, state: &mut BTermState) -> Input {
    let mut input = INPUT.lock();
    let mouse_tile = input.mouse_tile(0);

    let mut new_key = None;

    while matches!(new_key, None) {
        match input.pop() {
            Some(BEvent::KeyboardInput {
                key: vkey,
                scan_code: _,
                pressed,
            }) => {
                if let Ok(key) = Key::try_from(vkey) {
                    if pressed {
                        if state.pressed_keys.insert(key) {
                            new_key = Some(key);
                        }
                    } else {
                        state.pressed_keys.remove(&key);
                        //Maybe it was pressed from the beginning.
                        //assert!(was_present, "{key:?}")
                    }
                }
            }
            Some(BEvent::CloseRequested) => ctx.quit(),
            None => break,
            _ => {}
        }
    }

    Input {
        event: new_key.map(Event::Key),
        mouse_x: mouse_tile.x as usize,
        mouse_y: mouse_tile.y as usize,
        fps: ctx.fps,
    }
}

impl GameState for BTermState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let input = receive_input(ctx, self);

        let grid = self.world.update(input);

        draw_grid(ctx, grid);
    }
}

pub fn main() -> BError {
    let context = BTermBuilder::new()
        .with_dimensions(80, 50)
        .with_tile_dimensions(10, 16)
        .with_title("Hello Minimal Bracket World")
        .with_font("terminal_10x16.png", 10, 16)
        .with_simple_console(80, 50, "terminal_10x16.png")
        .with_advanced_input(true)
        .build()?;

    let gs: BTermState = BTermState::new();
    main_loop(context, gs)
}
