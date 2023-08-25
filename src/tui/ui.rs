use ratatui::{
    buffer::Cell as BufferCell,
    prelude::{Backend, Rect},
    style::{Color as BufferColor, Modifier},
    widgets::Widget,
    Frame,
};

use crate::grid::{Cell, Color, Grid};

impl From<Color> for BufferColor {
    fn from(value: Color) -> Self {
        fn cconv(f: f32) -> u8 {
            (f * u8::MAX as f32) as u8
        }
        Self::Rgb(cconv(value.r), cconv(value.g), cconv(value.b))
    }
}

impl Widget for Grid<Cell> {
    fn render(mut self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let grid = self.view();

        let area = area.intersection(Rect {
            x: 0,
            y: 0,
            width: grid.width() as u16,
            height: grid.height() as u16,
        });

        for line_id in area.top()..area.bottom() {
            for char_id in area.left()..area.right() {
                let cell = grid[line_id as usize][char_id as usize];

                *buf.get_mut(char_id, line_id) = BufferCell {
                    symbol: cell.character.into(),
                    fg: cell.foreground.into(),
                    bg: cell.background.into(),
                    underline_color: BufferColor::Reset,
                    modifier: Modifier::empty(),
                }
            }
        }
    }
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(grid: Grid<Cell>, frame: &mut Frame<'_, B>) {
    frame.render_widget(grid, frame.size());
}
