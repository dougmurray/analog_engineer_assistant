use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the low-pass / high-pass Bode plot pair side by side into `area`,
/// for a single-pole filter with the given corner frequency.
pub fn render(f: &mut Frame, area: Rect, f_c: f64) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_plot(f, cols[0], f_c, "Low-Pass Response", false);
    render_plot(f, cols[1], f_c, "High-Pass Response", true);
}

fn render_plot(f: &mut Frame, area: Rect, f_c: f64, title: &str, highpass: bool) {
    // Leave room for y-axis labels/line, border, and x-axis line/label row.
    let width = area.width.saturating_sub(9).clamp(8, 40) as usize;
    let height = area.height.saturating_sub(4).clamp(3, 8) as usize;

    let grid = plot_grid(f_c, width, height, highpass);

    let mut lines: Vec<Line> = Vec::with_capacity(height + 2);
    for (row, cells) in grid.iter().enumerate() {
        let label = match row {
            0 => "  0dB".to_string(),
            r if r == height / 2 => "-20dB".to_string(),
            r if r == height - 1 => "-40dB".to_string(),
            _ => "     ".to_string(),
        };
        // Top row gets an arrowhead on the y-axis line.
        let axis_char = if row == 0 { '▲' } else { '│' };
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", label), Style::default().fg(Color::DarkGray)),
            Span::styled(axis_char.to_string(), Style::default().fg(Color::DarkGray)),
            Span::styled(cells.clone(), Style::default().fg(Color::Cyan)),
        ]));
    }

    // X-axis line with an arrowhead at the right end.
    let x_axis = if width > 0 {
        format!("└{}▶", "─".repeat(width.saturating_sub(1)))
    } else {
        "└".to_string()
    };
    lines.push(Line::from(Span::styled(
        format!("      {}", x_axis),
        Style::default().fg(Color::DarkGray),
    )));

    // Frequency axis labels under the plot.
    let f_min = f_c / 20.0;
    let f_max = f_c * 20.0;
    let left_label = fmt_freq(f_min);
    let right_label = fmt_freq(f_max);
    let gap = width
        .saturating_sub(left_label.len() + right_label.len())
        .max(1);
    let freq_label = format!(
        "       {}{:>gap$}",
        left_label,
        right_label,
        gap = gap + right_label.len()
    );
    lines.push(Line::from(Span::styled(
        freq_label,
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

/// Build a `height`-row by `width`-column ASCII grid tracing a single-pole
/// filter response. The x-axis is linear from `f_c/20` to `20*f_c`; the
/// y-axis spans 0 dB (top) to -40 dB (bottom).
fn plot_grid(f_c: f64, width: usize, height: usize, highpass: bool) -> Vec<String> {
    let f_min = f_c / 20.0;
    let f_max = f_c * 20.0;

    let mut grid = vec![vec![' '; width]; height];
    #[allow(clippy::needless_range_loop)]
    for col in 0..width {
        let frac = if width > 1 {
            col as f64 / (width - 1) as f64
        } else {
            0.0
        };
        let freq = f_min + (f_max - f_min) * frac;
        let ratio = freq / f_c;

        let h_db = if highpass {
            -20.0 * (1.0 + (1.0 / ratio).powi(2)).sqrt().log10()
        } else {
            -20.0 * (1.0 + ratio.powi(2)).sqrt().log10()
        };
        let h_db = h_db.clamp(-40.0, 0.0);

        let row = if height > 1 {
            ((-h_db / 40.0) * (height - 1) as f64).round() as usize
        } else {
            0
        };
        grid[row.min(height - 1)][col] = '*';
    }

    grid.into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

/// Format a frequency using Hz/kHz/MHz with one decimal place.
fn fmt_freq(hz: f64) -> String {
    let abs = hz.abs();
    if abs >= 1e6 {
        format!("{:.1}MHz", hz / 1e6)
    } else if abs >= 1e3 {
        format!("{:.1}kHz", hz / 1e3)
    } else {
        format!("{:.1}Hz", hz)
    }
}
