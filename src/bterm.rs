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

impl Key {
    fn from(key: VirtualKeyCode) -> Option<Self> {
        match key {
            VirtualKeyCode::A => Some(Key::A),
            VirtualKeyCode::B => Some(Key::B),
            VirtualKeyCode::C => Some(Key::C),
            VirtualKeyCode::D => Some(Key::D),
            VirtualKeyCode::E => Some(Key::E),
            VirtualKeyCode::F => Some(Key::F),
            VirtualKeyCode::G => Some(Key::G),
            VirtualKeyCode::H => Some(Key::H),
            VirtualKeyCode::I => Some(Key::I),
            VirtualKeyCode::J => Some(Key::J),
            VirtualKeyCode::K => Some(Key::K),
            VirtualKeyCode::L => Some(Key::L),
            VirtualKeyCode::M => Some(Key::M),
            VirtualKeyCode::N => Some(Key::N),
            VirtualKeyCode::O => Some(Key::O),
            VirtualKeyCode::P => Some(Key::P),
            VirtualKeyCode::Q => Some(Key::Q),
            VirtualKeyCode::R => Some(Key::R),
            VirtualKeyCode::S => Some(Key::S),
            VirtualKeyCode::T => Some(Key::T),
            VirtualKeyCode::U => Some(Key::U),
            VirtualKeyCode::V => Some(Key::V),
            VirtualKeyCode::W => Some(Key::W),
            VirtualKeyCode::X => Some(Key::X),
            VirtualKeyCode::Y => Some(Key::Y),
            VirtualKeyCode::Z => Some(Key::Z),

            VirtualKeyCode::Left => Some(Key::Left),
            VirtualKeyCode::Up => Some(Key::Up),
            VirtualKeyCode::Right => Some(Key::Right),
            VirtualKeyCode::Down => Some(Key::Down),

            VirtualKeyCode::Key0 => Some(Key::Number0),
            VirtualKeyCode::Key1 => Some(Key::Number1),
            VirtualKeyCode::Key2 => Some(Key::Number2),
            VirtualKeyCode::Key3 => Some(Key::Number3),
            VirtualKeyCode::Key4 => Some(Key::Number4),
            VirtualKeyCode::Key5 => Some(Key::Number5),
            VirtualKeyCode::Key6 => Some(Key::Number6),
            VirtualKeyCode::Key7 => Some(Key::Number7),
            VirtualKeyCode::Key8 => Some(Key::Number8),
            VirtualKeyCode::Key9 => Some(Key::Number9),

            _ => {
                log(format!("unrecognized key {key:?}"));
                None
            }
        }
    }
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
                if let Some(key) = Key::from(vkey) {
                    if pressed {
                        if state.pressed_keys.insert(key) {
                            new_key = Some(key);
                        }
                    } else {
                        let was_present = state.pressed_keys.remove(&key);
                        assert!(was_present)
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
