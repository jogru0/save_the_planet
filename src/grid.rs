use std::ops::{Index, IndexMut, Range};

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
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

impl<'a> MutGridView<'a, char> {
    pub fn print(&mut self, line_id: usize, mut char_id: usize, string: &str) {
        for char in string.chars() {
            self[line_id][char_id] = char;
            char_id += 1;
        }
    }

    pub fn block(&mut self) -> MutGridView<'_, char> {
        assert!(2 <= self.width);
        assert!(2 <= self.height);

        let last_line = self.height - 1;
        let last_char = self.width - 1;

        for l in 1..last_line {
            self[l][0] = '│';
            self[l][last_char] = '│';
        }

        for c in 1..last_char {
            self[0][c] = '─';
            self[last_line][c] = '─';
        }

        self[0][0] = '┌';
        self[0][last_char] = '┐';
        self[last_line][0] = '└';
        self[last_line][last_char] = '┘';

        // self[0][0] = '╭';
        // self[0][last_char] = '╮';
        // self[last_line][0] = '╰';
        // self[last_line][last_char] = '╯';

        self.sub_view(1, 1, self.height - 2, self.width - 2)
    }
}

impl<'a, T> MutGridView<'a, T> {
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
        MutGridView::<'b, T> {
            reference: self.reference,
            start_line_id: self.start_line_id + start_line_id,
            start_char_id: self.start_char_id + start_char_id,
            height,
            width,
        }
    }

    pub fn fill(&mut self, t: T)
    where
        T: Clone,
    {
        for line_id in 0..self.height {
            for char_id in 0..self.width {
                let reference_index = self.reference_index(line_id, char_id);
                self.reference.data[reference_index] = t.clone();
            }
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
    use super::Grid;

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
        let mut grid = Grid::new(30, 20, ' ');

        let mut view = grid.view();

        let mut sub_view_1 = view.sub_view(10, 10, 3, 8);

        sub_view_1.fill('@');
        let mut sub_view_2 = view.sub_view(20, 5, 3, 13);
        sub_view_2.fill('B');
        let mut sub_view_3 = sub_view_2.sub_view(0, 0, 2, 2);
        sub_view_3.fill('E');

        for line_id in 0..30 {
            for char_id in 0..20 {
                print!("{}", view[line_id][char_id]);
            }
            println!();
        }
    }
}
