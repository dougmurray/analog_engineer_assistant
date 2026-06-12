use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{App, Mode};
use crate::input::{format_eng, parse_value};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    match app.mode {
        Mode::TopicList => render_topic_preview(f, app, area),
        Mode::Search => render_search_preview(f, app, area),
        _ => render_formula_calc(f, app, area),
    }
}

fn render_topic_preview(f: &mut Frame, app: &App, area: Rect) {
    let ch = app.current_topic();
    let formula_names: Vec<Line> = ch
        .formulas
        .iter()
        .map(|fo| {
            Line::from(vec![
                Span::raw("  • "),
                Span::styled(fo.name, Style::default().fg(Color::Cyan)),
            ])
        })
        .collect();

    let block = Block::default()
        .title(format!(" {} — formulas ", ch.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let para = Paragraph::new(formula_names)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn render_search_preview(f: &mut Frame, app: &App, area: Rect) {
    // Show the formula for the currently highlighted search result
    if let Some(result) = app.search_results.get(app.search_cursor) {
        let ch = &app.topics[result.topic_idx];
        if let Some(formula) = ch.formulas.get(result.formula_idx)
            && let Some(variant) = formula.variants.first()
        {
            render_variant_info(f, app, area, variant, formula.name, ch.name);
            return;
        }
    }
    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(block, area);
}

fn render_variant_info(
    f: &mut Frame,
    _app: &App,
    area: Rect,
    variant: &crate::formulas::SolveVariant,
    formula_name: &str,
    topic_name: &str,
) {
    let mut lines = vec![
        Line::from(vec![
            Span::styled(topic_name, Style::default().fg(Color::DarkGray)),
            Span::raw("  ›  "),
            Span::styled(
                formula_name,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            variant.expression,
            Style::default().fg(Color::Cyan),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "Variables:",
            Style::default().fg(Color::Yellow),
        )]),
    ];
    for vd in variant.inputs {
        lines.push(Line::from(vec![
            Span::raw(format!("  {}  ", vd.symbol)),
            Span::styled(vd.name, Style::default().fg(Color::White)),
            Span::styled(
                format!("  [{}]", vd.unit),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn render_formula_calc(f: &mut Frame, app: &App, area: Rect) {
    let ch = app.current_topic();
    let formula = match ch.formulas.get(app.formula_cursor) {
        Some(f) => f,
        None => return,
    };
    let variant = match formula.variants.get(app.variant_idx) {
        Some(v) => v,
        None => return,
    };

    // Split area: top for expression + result, bottom for variable inputs
    let mut top_height = if formula.note.is_some() { 9 } else { 8 };
    if variant.output_unit == "codes" {
        top_height += 1;
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(top_height), Constraint::Min(0)])
        .split(area);

    render_expression_block(f, app, chunks[0], formula.name, variant);
    render_inputs_block(f, app, chunks[1], formula.name, variant);
}

fn render_expression_block(
    f: &mut Frame,
    app: &App,
    area: Rect,
    name: &str,
    variant: &crate::formulas::SolveVariant,
) {
    let n_variants = app.current_topic().formulas[app.formula_cursor]
        .variants
        .len();

    let mut lines = vec![
        Line::from(vec![Span::styled(
            name,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            variant.expression,
            Style::default().fg(Color::Cyan),
        )]),
    ];

    if let Some(note) = app.current_topic().formulas[app.formula_cursor].note {
        lines.push(Line::from(vec![Span::styled(
            note,
            Style::default().fg(Color::DarkGray),
        )]));
    }

    lines.push(Line::raw(""));

    // Result line
    let result_str = if let Some(val) = app.compute_result() {
        if val.is_finite() {
            format!("  =  {}  {}", format_eng(val), variant.output_unit)
        } else {
            "  =  (invalid input)".to_string()
        }
    } else {
        "  =  —".to_string()
    };

    lines.push(Line::from(vec![Span::styled(
        result_str,
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )]));

    // Hexadecimal representation of ADC output codes
    if variant.output_unit == "codes"
        && let Some(val) = app.compute_result()
        && val.is_finite()
        && val >= 0.0
    {
        lines.push(Line::from(vec![Span::styled(
            format!("  =  0x{:X}", val as u64),
            Style::default().fg(Color::Green),
        )]));
    }

    // Solve-for tab hint
    if n_variants > 1 {
        lines.push(Line::raw(""));
        let tabs: Vec<Span> = app.current_topic().formulas[app.formula_cursor]
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if i == app.variant_idx {
                    Span::styled(
                        format!("[{}] ", v.solves_for),
                        Style::default().fg(Color::Black).bg(Color::Cyan),
                    )
                } else {
                    Span::styled(
                        format!("[{}] ", v.solves_for),
                        Style::default().fg(Color::DarkGray),
                    )
                }
            })
            .collect();
        let mut tab_line = vec![Span::styled(
            "  Solve for: ",
            Style::default().fg(Color::DarkGray),
        )];
        tab_line.extend(tabs);
        tab_line.push(Span::styled(
            "  (Tab)",
            Style::default().fg(Color::DarkGray),
        ));
        lines.push(Line::from(tab_line));
    }

    let block = Block::default()
        .title(format!(" {} ", variant.solves_for))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn render_inputs_block(
    f: &mut Frame,
    app: &App,
    area: Rect,
    formula_name: &str,
    variant: &crate::formulas::SolveVariant,
) {
    let is_editing = app.mode == Mode::InputEdit;
    let show_bode = matches!(
        formula_name,
        "RC Filter Corner Frequency" | "LC Filter Corner Frequency"
    );
    let corner_freq = if show_bode {
        app.corner_frequency()
    } else {
        None
    };
    let show_divider = formula_name == "Voltage Divider";

    let mut lines: Vec<Line> = vec![Line::from(vec![Span::styled(
        "  Variables  (j/k to move, Enter/i to edit)",
        Style::default().fg(Color::DarkGray),
    )])];

    lines.push(Line::raw(""));

    for (i, vd) in variant.inputs.iter().enumerate() {
        let selected = i == app.input_cursor;
        let editing = selected && is_editing;

        let value_str = if editing {
            format!("{}█", app.edit_buffer)
        } else {
            let raw = app.input_values.get(i).cloned().unwrap_or_default();
            // Show parsed value in engineering notation alongside raw entry
            if let Some(parsed) = parse_value(&raw) {
                if format_eng(parsed) == raw {
                    raw
                } else {
                    format!("{}  ({})", raw, format_eng(parsed))
                }
            } else {
                format!("{} ✗", raw)
            }
        };

        let (sym_style, val_style, row_bg) = if editing {
            (
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Black).bg(Color::Yellow),
                Color::Yellow,
            )
        } else if selected {
            (
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Black).bg(Color::Green),
                Color::Green,
            )
        } else {
            (
                Style::default().fg(Color::Cyan),
                Style::default().fg(Color::White),
                Color::Reset,
            )
        };

        let _ = row_bg; // used indirectly via styles

        lines.push(Line::from(vec![
            Span::styled(format!("  {:>6}  ", vd.symbol), sym_style),
            Span::styled(format!("{:<20}", vd.name), val_style),
            Span::styled(format!("  {:>12}  ", value_str), val_style),
            Span::styled(vd.unit.to_string(), Style::default().fg(Color::DarkGray)),
        ]));
    }

    let block = Block::default()
        .title(" Inputs ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    if let Some(f_c) = corner_freq {
        let inputs_height = (lines.len() as u16) + 2;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(inputs_height), Constraint::Min(0)])
            .split(area);

        let para = Paragraph::new(lines).block(block);
        f.render_widget(para, chunks[0]);

        if chunks[1].height > 2 {
            crate::ui::bode::render(f, chunks[1], f_c);
        }
    } else if show_divider {
        let inputs_height = (lines.len() as u16) + 2;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(inputs_height), Constraint::Min(0)])
            .split(area);

        let para = Paragraph::new(lines).block(block);
        f.render_widget(para, chunks[0]);

        if chunks[1].height > 2 {
            crate::ui::divider::render(
                f,
                chunks[1],
                app.variable_value("V_in"),
                app.variable_value("R1"),
                app.variable_value("R2"),
                app.variable_value("V_out"),
            );
        }
    } else {
        let para = Paragraph::new(lines).block(block);
        f.render_widget(para, area);
    }
}
