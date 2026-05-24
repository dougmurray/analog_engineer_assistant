use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, Mode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    match app.mode {
        Mode::Search => render_search_results(f, app, area),
        Mode::FormulaList | Mode::FormulaView | Mode::InputEdit => render_formula_list(f, app, area),
        Mode::ChapterList => render_chapter_list(f, app, area),
    }
}

fn render_chapter_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .chapters
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let style = if i == app.chapter_cursor {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![Span::styled(format!("  {}", ch.name), style)]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.chapter_cursor));

    let block = Block::default()
        .title(" Chapters ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let list = List::new(items).block(block);
    f.render_stateful_widget(list, area, &mut state);
}

fn render_formula_list(f: &mut Frame, app: &App, area: Rect) {
    let ch = app.current_chapter();
    let items: Vec<ListItem> = ch
        .formulas
        .iter()
        .enumerate()
        .map(|(i, formula)| {
            let selected = i == app.formula_cursor;
            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![Span::styled(
                format!("  {}", formula.name),
                style,
            )]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.formula_cursor));

    let title = format!(" {} ", ch.name);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let list = List::new(items).block(block);
    f.render_stateful_widget(list, area, &mut state);
}

fn render_search_results(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let selected = i == app.search_cursor;
            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![Span::styled(
                format!("  {}", result.label),
                style,
            )]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.search_cursor));

    let count = app.search_results.len();
    let title = format!(" Results ({}) ", count);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let list = List::new(items).block(block);
    f.render_stateful_widget(list, area, &mut state);
}
