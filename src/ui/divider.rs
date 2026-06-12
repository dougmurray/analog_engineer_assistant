use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::input::format_eng;

/// Render a simple voltage-divider schematic, labeled with the current
/// V_in, R1, R2 and V_out values where available.
pub fn render(
    f: &mut Frame,
    area: Rect,
    v_in: Option<f64>,
    r1: Option<f64>,
    r2: Option<f64>,
    v_out: Option<f64>,
) {
    let v_in_label = v_in.map_or("V_in".to_string(), |v| format!("{}V", format_eng(v)));
    let r1_label = r1.map_or("R1".to_string(), |v| format!("R1 = {}Ω", format_eng(v)));
    let r2_label = r2.map_or("R2".to_string(), |v| format!("R2 = {}Ω", format_eng(v)));
    let v_out_label = v_out.map_or("V_out".to_string(), |v| format!("V_out = {}V", format_eng(v)));

    let label_style = Style::default().fg(Color::White);
    let wire_style = Style::default().fg(Color::DarkGray);

    let lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("   ", wire_style),
            Span::styled(v_in_label, label_style),
        ]),
        Line::from(Span::styled("    │", wire_style)),
        Line::from(Span::styled("   ┌┴┐", wire_style)),
        Line::from(vec![
            Span::styled("   │ │  ", wire_style),
            Span::styled(r1_label, label_style),
        ]),
        Line::from(Span::styled("   └┬┘", wire_style)),
        Line::from(vec![
            Span::styled("    ├──── ", wire_style),
            Span::styled(v_out_label, label_style),
        ]),
        Line::from(Span::styled("   ┌┴┐", wire_style)),
        Line::from(vec![
            Span::styled("   │ │  ", wire_style),
            Span::styled(r2_label, label_style),
        ]),
        Line::from(Span::styled("   └┬┘", wire_style)),
        Line::from(Span::styled("    │", wire_style)),
        Line::from(Span::styled("   ───  GND", wire_style)),
    ];

    let block = Block::default()
        .title(" Schematic ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}
