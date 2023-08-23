use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    render_main_navigation_2(app, frame, frame.size())
}

pub fn render_flyer<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let ratio = app.buyers as f64 / 1000.0;
    let show_progress = ratio >= 0.2;

    let p = if show_progress {
        if ratio <= 0.93 {
            70
        } else {
            let additional = ratio - 0.93;
            let sub = additional * 1000.;
            let sub = sub as u16;
            70_u16.saturating_sub(sub)
        }
    } else {
        100
    };

    if show_progress {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(p), Constraint::Percentage(100 - p)].as_ref())
            .direction(Direction::Vertical)
            .split(area);
        render_text(app, frame, chunks[0]);
        render_progress(app, frame, chunks[1])
    } else {
        render_text(app, frame, area)
    }
}

pub fn render_main_navigation_2<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(21), Constraint::Min(0)].as_ref())
        .split(area);

    let mut cards = vec!["Flyer"];
    if app.cards.achievements.is_some() {
        cards.push("Achievements");
    }

    let mut state = ListState::default();
    state.select(cards.iter().position(|it| it == &app.cards.selected));

    let list = List::new(
        cards
            .iter()
            .map(|c| ListItem::new(vec![Line::from(Span::raw(*c))]))
            .collect_vec(),
    )
    .block(Block::default().borders(Borders::ALL).title("List"))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[0], &mut state);

    match app.cards.selected.as_str() {
        "Flyer" => render_flyer(app, frame, chunks[1]),
        "Achievements" => {}
        _ => unreachable!(),
    }
}

pub fn render_text<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let counter = if app.counter > 0 {
        format!("Counter: {}", app.counter)
    } else {
        "Press `Enter` to increment the counter.".to_owned()
    };

    let auto_clicker_buy = if app.buyers > 0 {
        format!("Buyer: {}", app.buyers)
    } else if app.counter >= 20 {
        "Press `a` to buy an auto buyer for 20.".to_owned()
    } else {
        "".to_owned()
    };

    let winning = if app.buyers >= 100 {
        "Reach 1000 Buyer to win."
    } else {
        ""
    };

    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                {}\n\
                {}\n\
                {}",
            counter, auto_clicker_buy, winning
        ))
        .block(
            Block::default()
                .title("Template")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        area,
    );
}

pub fn render_progress<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let ratio = app.buyers as f64 / 1000.0;

    let label = format!("{:.2}%", ratio * 100.);
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio((ratio / 93. * 100.).min(1.));

    frame.render_widget(gauge, area);
}
