use ratatui::{
    prelude::{Backend, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::grid::Grid;

/// Renders the user interface widgets.
pub fn render<B: Backend>(mut grid: Grid<char>, frame: &mut Frame<'_, B>) {
    let grid = grid.view();

    let mut text = String::new();
    for line_id in 0..grid.height() {
        for &char in &grid[line_id] {
            text.push(char);
        }
        text.push('\n');
    }

    let width = (grid.width() as u16).min(frame.size().width);
    let height = (grid.height() as u16).min(frame.size().height);

    let paragraph = Paragraph::new(text);
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        paragraph,
        Rect {
            x: 0,
            y: 0,
            width,
            height,
        },
    )
}
