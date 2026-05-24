pub mod nav;
pub mod formula;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::{App, Mode};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // Reserve bottom bar for search when in Search mode (or always show hint)
    let (main_area, bar_area) = split_main_and_bar(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
        .split(main_area);

    nav::render(f, app, chunks[0]);
    formula::render(f, app, chunks[1]);
    render_search_bar(f, app, bar_area);
}

fn split_main_and_bar(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);
    (chunks[0], chunks[1])
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    use ratatui::{
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    };

    let is_searching = app.mode == Mode::Search;

    let label = if is_searching {
        Span::styled("/ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("Press / to search", Style::default().fg(Color::DarkGray))
    };

    let content = if is_searching {
        let query = Span::styled(
            app.search_query.as_str(),
            Style::default().fg(Color::White),
        );
        let cursor = Span::styled("█", Style::default().fg(Color::Yellow));
        Line::from(vec![label, query, cursor])
    } else {
        Line::from(vec![label])
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(if is_searching {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let para = Paragraph::new(content).block(block);
    f.render_widget(para, area);
}
