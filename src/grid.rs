use std::ops::{Index, IndexMut, Range};

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl Color {
    pub const CYAN: Color = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const WHITE: Color = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const RED: Color = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const ORANGE: Color = Self {
        r: 1.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };

    pub const GREY: Color = Self {
        r: 0.1,
        g: 0.1,
        b: 0.2,
        a: 1.0,
    };

    pub const YELLOW: Color = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const BLACK: Color = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub character: char,
    pub foreground: Color,
    pub background: Color,
}
impl Cell {
    pub(crate) fn new() -> Self {
        Self {
            character: ' ',
            foreground: Color::WHITE,
            background: Color::BLACK,
        }
    }
}

impl<T> Grid<T> {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn new(height: usize, width: usize, t: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![t; height * width],
            width,
        }
    }

    pub fn view(&mut self) -> MutGridView<T> {
        let width = self.width;
        let height = self.height();
        MutGridView {
            reference: self,
            start_line_id: 0,
            start_char_id: 0,
            height,
            width,
            //parent_view: None,
        }
    }
}

pub struct MutGridView<'a, T> {
    reference: &'a mut Grid<T>,
    start_line_id: usize,
    start_char_id: usize,
    height: usize,
    width: usize,
    // parent_view: Option<&'a mut MutGridView<'a, T>>,
}

impl<'a> MutGridView<'a, Cell> {
    pub fn print(&mut self, line_id: usize, mut char_id: usize, string: &str) {
        for char in string.chars() {
            self[line_id][char_id].character = char;
            char_id += 1;
        }
    }

    pub fn print_overflowing(&mut self, mut line_id: usize, string: &str) {
        assert_ne!(self.width, 0);
        let mut char_id = 0;
        for char in string.chars() {
            self[line_id][char_id].character = char;
            char_id += 1;
            if char_id == self.width {
                char_id = 0;
                line_id += 1;
            }
        }
    }

    #[allow(dead_code)]
    pub fn fill_char(&mut self, c: char) {
        for line_id in 0..self.height {
            for char_id in 0..self.width {
                let reference_index = self.reference_index(line_id, char_id);
                self.reference.data[reference_index].character = c;
            }
        }
    }

    pub fn split_block(
        &mut self,
        left_width: usize,
    ) -> (MutGridView<'_, Cell>, (usize, usize, usize, usize)) {
        self.block();
        let sep_id = left_width + 1;

        for l in 1..self.height - 1 {
            self[l][sep_id].character = '│';
        }

        let inner_height = self.height - 2;
        let inner_width = self.width - 3;

        self[0][sep_id].character = '┬';
        self[inner_height + 1][sep_id].character = '┴';

        (
            self.sub_view(1, 1, inner_height, left_width),
            (1, sep_id + 1, inner_height, inner_width - left_width),
        )
    }

    pub fn block(&mut self) -> MutGridView<'_, Cell> {
        assert!(2 <= self.width);
        assert!(2 <= self.height);

        let last_line = self.height - 1;
        let last_char = self.width - 1;

        for l in 1..last_line {
            self[l][0].character = '│';
            self[l][last_char].character = '│';
        }

        for c in 1..last_char {
            self[0][c].character = '─';
            self[last_line][c].character = '─';
        }

        self[0][0].character = '┌';
        self[0][last_char].character = '┐';
        self[last_line][0].character = '└';
        self[last_line][last_char].character = '┘';

        // self[0][0] = '╭';
        // self[0][last_char] = '╮';
        // self[last_line][0] = '╰';
        // self[last_line][last_char] = '╯';

        self.sub_view(1, 1, self.height - 2, self.width - 2)
    }

    pub fn _fill_background(&mut self, color: Color) {
        self.for_all(|cell| cell.background = color)
    }

    pub fn fill_foreground(&mut self, color: Color) {
        self.for_all(|cell| cell.foreground = color)
    }
}

impl<'a, T> MutGridView<'a, T> {
    fn for_all<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        for line_id in 0..self.height {
            for char_id in 0..self.width {
                f(&mut self[line_id][char_id])
            }
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn reference_index(&self, line_id: usize, char_id: usize) -> usize {
        let reference_line_id = self.start_line_id + line_id;
        let reference_char_id = self.start_char_id + char_id;
        reference_line_id * self.reference.width + reference_char_id
    }

    fn range_for_line(&self, line_id: usize) -> Range<usize> {
        self.reference_index(line_id, 0)..self.reference_index(line_id, self.width)
    }

    pub fn sub_view<'b>(
        &'b mut self,
        start_line_id: usize,
        start_char_id: usize,
        height: usize,
        width: usize,
    ) -> MutGridView<'b, T> {
        assert!(start_line_id + height <= self.height);
        assert!(start_char_id + width <= self.width);

        MutGridView::<'b, T> {
            reference: self.reference,
            start_line_id: self.start_line_id + start_line_id,
            start_char_id: self.start_char_id + start_char_id,
            height,
            width,
        }
    }
}

impl<'a, T> Index<usize> for MutGridView<'a, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.reference.data[self.range_for_line(index)]
    }
}

impl<'a, T> IndexMut<usize> for MutGridView<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let range = self.range_for_line(index);
        &mut self.reference.data[range]
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, Grid};

    #[test]
    fn grid_view() {
        let mut grid = Grid::new(30, 20, ' ');

        let mut view = grid.view();

        view[3][2] = 'd';
        view[3][4] = '3';
        view[23][2] = 'g';

        for line_id in 0..30 {
            for char_id in 0..20 {
                print!("{}", view[line_id][char_id]);
            }
            println!();
        }
    }

    #[test]
    fn fill() {
        let mut grid = Grid::new(30, 20, Cell::new());

        let mut view = grid.view();

        let mut sub_view_1 = view.sub_view(10, 10, 3, 8);

        sub_view_1.fill_char('@');
        let mut sub_view_2 = view.sub_view(20, 5, 3, 13);
        sub_view_2.fill_char('B');
        let mut sub_view_3 = sub_view_2.sub_view(0, 0, 2, 2);
        sub_view_3.fill_char('E');

        for line_id in 0..30 {
            for char_id in 0..20 {
                print!("{}", view[line_id][char_id].character);
            }
            println!();
        }
    }
}
