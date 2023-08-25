use crate::grid::{Cell, Grid};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    Down,
    Up,
    Left,
    Right,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
}

pub enum Event {
    Key(Key),
}

#[derive(Debug)]
pub struct World {}

pub struct Input {
    pub event: Option<Event>,
    pub mouse_x: usize,
    pub mouse_y: usize,
    pub fps: f32,
}

impl World {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, input: Input) -> Grid<Cell> {
        let mut grid = Grid::new(50, 80, Cell::new());

        let mut view = grid.view();

        let mut sub_view_1 = view.sub_view(10, 10, 3, 8);

        sub_view_1.fill_char('@');
        let mut sub_view_2 = view.sub_view(20, 5, 3, 13);
        sub_view_2.fill_char('B');
        let mut sub_view_3 = sub_view_2.sub_view(0, 0, 2, 2);
        sub_view_3.fill_char('E');

        let mut sub = view.sub_view(30, 30, 15, 15);
        let mut inner = sub.block();
        inner.fill_char('i');

        view.print(
            2,
            1,
            &format!("Mouse tile position: {}, {}", input.mouse_x, input.mouse_y),
        );
        view.print(3, 1, &format!("FPS: {}", input.fps));

        let mut split_view = view.sub_view(40, 50, 9, 29);
        let (mut new, del) = split_view.split_block(10);

        new.fill_char('l');
        split_view
            .sub_view(del.0, del.1, del.2, del.3)
            .fill_char('r');

        if let Some(Event::Key(key)) = input.event {
            view.print(25, 50, &format!("Key code: {:?}", key));
        }

        grid
    }
}
