pub mod nav;
pub mod formula;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, Mode};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    let (main_area, search_area, cmd_area) = split_areas(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
        .split(main_area);

    nav::render(f, app, chunks[0]);
    formula::render(f, app, chunks[1]);
    render_search_bar(f, app, search_area);
    render_command_bar(f, app, cmd_area);
}

fn split_areas(area: Rect) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3), Constraint::Length(1)])
        .split(area);
    (chunks[0], chunks[1], chunks[2])
}

fn render_command_bar(f: &mut Frame, app: &App, area: Rect) {
    let key = |k: &'static str| Span::styled(k, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    let desc = |d: &'static str| Span::styled(format!(" {}  ", d), Style::default().fg(Color::DarkGray));
    let sep = || Span::styled(" ", Style::default());

    let spans: Vec<Span> = match app.mode {
        Mode::ChapterList => vec![
            key("j/k"), desc("Move"),
            sep(),
            key("l/Enter"), desc("Open"),
            sep(),
            key("/"), desc("Search"),
            sep(),
            key("q"), desc("Quit"),
        ],
        Mode::FormulaList => vec![
            key("j/k"), desc("Move"),
            sep(),
            key("l/Enter"), desc("Open"),
            sep(),
            key("h/Esc"), desc("Back"),
            sep(),
            key("/"), desc("Search"),
            sep(),
            key("q"), desc("Quit"),
        ],
        Mode::FormulaView => vec![
            key("j/k"), desc("Select Input"),
            sep(),
            key("i/Enter"), desc("Edit"),
            sep(),
            key("Tab"), desc("Cycle Variant"),
            sep(),
            key("h/Esc"), desc("Back"),
            sep(),
            key("/"), desc("Search"),
        ],
        Mode::InputEdit => vec![
            key("Enter"), desc("Confirm"),
            sep(),
            key("Esc"), desc("Cancel"),
        ],
        Mode::Search => vec![
            key("j/k"), desc("Move"),
            sep(),
            key("Enter"), desc("Jump to Formula"),
            sep(),
            key("Esc"), desc("Cancel"),
        ],
    };

    let para = Paragraph::new(Line::from(spans));
    f.render_widget(para, area);
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
