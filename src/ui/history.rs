use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::App;
use crate::input::format_eng;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .history
        .iter()
        .map(|entry| {
            let sym = format!("{}:", entry.variable);
            let val = format!("{} {}", format_eng(entry.value), entry.unit);
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {:<12}", sym), Style::default().fg(Color::Cyan)),
                Span::styled(val, Style::default().fg(Color::White)),
            ]))
        })
        .collect();

    let title = if app.history.is_empty() {
        " History — empty "
    } else {
        " History  (y: copy • H/Esc: close) "
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    let mut state = ListState::default();
    if !app.history.is_empty() {
        state.select(Some(app.history_cursor));
    }

    f.render_stateful_widget(list, area, &mut state);
}
