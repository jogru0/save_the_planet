use crossterm::event::KeyCode;
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::app::{
    cards::{Achievements, Card},
    App,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.cards.achievements.is_none() && app.check_key(KeyCode::Char('A')) {
        app.cards.achievements = Some(Achievements::new())
    }

    let area = frame.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Save the Planet")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    let inner = block.inner(area);
    frame.render_widget(block, area);
    render_main_navigation(app, frame, inner);

    frame.render_widget(
        Paragraph::new(format!("old: {:?}\nnew: {:?}", app.old_key, app.key))
            .block(Block::default().borders(Borders::all())),
        Rect {
            x: 30,
            y: 10,
            width: 22,
            height: 6,
        },
    )
}

pub fn render_flyer<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
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

pub fn render_main_navigation<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    if app.check_key(KeyCode::Up) {
        app.cards.previous();
    } else if app.check_key(KeyCode::Down) {
        app.cards.next();
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(21), Constraint::Min(0)].as_ref())
        .split(area);

    let mut cards = vec![Card::Flyer];
    if app.cards.achievements.is_some() {
        cards.push(Card::Achievements);
    }

    let mut state = ListState::default();
    state.select(cards.iter().position(|it| it == &app.cards.selected));

    let list = List::new(
        cards
            .iter()
            .map(|c| ListItem::new(vec![Line::from(Span::raw(c.text()))]))
            .collect_vec(),
    )
    .block(Block::default().borders(Borders::RIGHT))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[0], &mut state);

    match app.cards.selected {
        Card::Flyer => render_flyer(app, frame, chunks[1]),
        Card::Achievements => render_achievements(app, frame, chunks[1]),
    }
}

pub fn render_text<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    if app.check_key(KeyCode::Char('f')) {
        app.cards.flyer.available_flyers = Some(match app.cards.flyer.available_flyers {
            Some(x) => x + 1,
            None => 0,
        });
    }

    let flyer = &app.cards.flyer;

    let mut lines = Vec::new();

    let string = format!("Saved {} kg CO2e", flyer.saved_co2);
    lines.push(Line::from(string.bold()));

    let flyer_line = if let Some(available) = flyer.available_flyers {
        Line::from(format!("Flyer: {}", available))
    } else {
        Line::from("Press `f` to print a flyer.")
    };
    lines.push(flyer_line);

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

pub fn render_achievements<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let mut text = vec![Line::from("Unlocked Achievements".bold())];

    for achievement in &app.cards.achievements.as_ref().unwrap().unlocked {
        text.push(Line::from(achievement.text.clone()))
    }

    frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), area);
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
