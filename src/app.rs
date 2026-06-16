use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

use crate::formulas::{Topic, all_topics};
use crate::input::{format_eng, parse_value};

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    TopicList,
    FormulaList,
    FormulaView,
    InputEdit,
    Search,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub topic_idx: usize,
    pub formula_idx: usize,
    pub score: i64,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub variable: String,
    pub value: f64,
    pub unit: String,
}

pub struct App {
    pub topics: Vec<Topic>,
    pub mode: Mode,

    // Navigation
    pub topic_cursor: usize,
    pub formula_cursor: usize,
    pub variant_idx: usize,  // which "solve for" variant is active
    pub input_cursor: usize, // which variable field is focused

    // Per-formula input values (indexed by variant input slot)
    pub input_values: Vec<String>,
    pub edit_buffer: String,
    pub edit_is_fresh: bool,

    // Search
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_cursor: usize,

    // Calculation history popup
    pub history: Vec<HistoryEntry>,
    pub history_cursor: usize,
    pub show_history: bool,

    // Previous mode to restore on Esc from search
    prev_topic: usize,
    prev_formula: usize,
    prev_variant: usize,
}

impl App {
    pub fn new(jump_topic: Option<usize>) -> Self {
        let topics = all_topics();
        let topic_cursor = jump_topic.unwrap_or(0).min(topics.len().saturating_sub(1));
        let mode = if jump_topic.is_some() {
            Mode::FormulaList
        } else {
            Mode::TopicList
        };
        let mut app = Self {
            topics,
            mode,
            topic_cursor,
            formula_cursor: 0,
            variant_idx: 0,
            input_cursor: 0,
            input_values: vec![],
            edit_buffer: String::new(),
            edit_is_fresh: false,
            search_query: String::new(),
            search_results: vec![],
            search_cursor: 0,
            history: vec![],
            history_cursor: 0,
            show_history: false,
            prev_topic: 0,
            prev_formula: 0,
            prev_variant: 0,
        };
        app.reset_inputs();
        app
    }

    pub fn current_topic(&self) -> &Topic {
        &self.topics[self.topic_cursor]
    }

    pub fn current_variant(&self) -> Option<&crate::formulas::SolveVariant> {
        let ch = self.current_topic();
        if self.formula_cursor >= ch.formulas.len() {
            return None;
        }
        let f = &ch.formulas[self.formula_cursor];
        f.variants.get(self.variant_idx)
    }

    fn reset_inputs(&mut self) {
        if let Some(v) = self.current_variant() {
            self.input_values = v.inputs.iter().map(|vd| format_eng(vd.default)).collect();
        } else {
            self.input_values = vec![];
        }
        self.input_cursor = 0;
    }

    pub fn compute_result(&self) -> Option<f64> {
        let variant = self.current_variant()?;
        let vals: Vec<f64> = self
            .input_values
            .iter()
            .map(|s| parse_value(s).unwrap_or(f64::NAN))
            .collect();
        if vals.len() != variant.inputs.len() {
            return None;
        }
        Some((variant.compute)(&vals))
    }

    /// Resolve the filter corner frequency for the current variant, whether
    /// it's the solved-for result (`f_c`) or one of the inputs.
    pub fn corner_frequency(&self) -> Option<f64> {
        let variant = self.current_variant()?;
        if variant.solves_for == "f_c" {
            return self.compute_result().filter(|v| v.is_finite() && *v > 0.0);
        }
        let idx = variant.inputs.iter().position(|vd| vd.symbol == "f_c")?;
        parse_value(self.input_values.get(idx)?).filter(|v| v.is_finite() && *v > 0.0)
    }

    /// Resolve the value of a named variable for the current variant,
    /// whether it's the solved-for result or one of the inputs.
    pub fn variable_value(&self, symbol: &str) -> Option<f64> {
        let variant = self.current_variant()?;
        if variant.solves_for == symbol {
            return self.compute_result().filter(|v| v.is_finite());
        }
        let idx = variant.inputs.iter().position(|vd| vd.symbol == symbol)?;
        parse_value(self.input_values.get(idx)?)
    }

    // ── Key handlers ─────────────────────────────────────────────────────────

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::{KeyCode::*, KeyModifiers};
        let code = key.code;

        // H (Shift+h) toggles history overlay from any non-edit mode
        if code == Char('H') && self.mode != Mode::InputEdit {
            self.show_history = !self.show_history;
            if self.show_history {
                self.history_cursor = self.history.len().saturating_sub(1);
            }
            return;
        }

        // When history overlay is open, intercept navigation/copy/close keys
        if self.show_history {
            match code {
                Char('j') | Down => {
                    let n = self.history.len();
                    if n > 0 {
                        self.history_cursor = (self.history_cursor + 1).min(n - 1);
                    }
                }
                Char('k') | Up => {
                    self.history_cursor = self.history_cursor.saturating_sub(1);
                }
                Esc => self.show_history = false,
                Char('y') => self.copy_selected_history(),
                Char('c') if key.modifiers.contains(KeyModifiers::SUPER) => {
                    self.copy_selected_history();
                }
                _ => {}
            }
            return;
        }

        // "/" always opens search from any non-edit mode
        if code == Char('/') && self.mode != Mode::InputEdit {
            self.open_search();
            return;
        }

        match self.mode.clone() {
            Mode::TopicList => self.handle_topic_list(code),
            Mode::FormulaList => self.handle_formula_list(code),
            Mode::FormulaView => self.handle_formula_view(code),
            Mode::InputEdit => self.handle_input_edit(code),
            Mode::Search => self.handle_search(code),
        }
    }

    fn push_history(&mut self) {
        if let Some(val) = self.compute_result()
            && val.is_finite()
            && let Some(variant) = self.current_variant()
        {
            let entry = HistoryEntry {
                variable: variant.solves_for.to_string(),
                value: val,
                unit: variant.output_unit.to_string(),
            };
            self.history.push(entry);
            if self.history.len() > 100 {
                self.history.remove(0);
            }
        }
    }

    fn copy_selected_history(&self) {
        // Items are displayed newest-first, so index 0 is the last pushed entry
        if let Some(entry) = self.history.get(self.history_cursor) {
            copy_to_clipboard(&format_eng_sci(entry.value));
        }
    }

    fn handle_topic_list(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        match key {
            Char('j') | Down => {
                self.topic_cursor = (self.topic_cursor + 1).min(self.topics.len() - 1)
            }
            Char('k') | Up => self.topic_cursor = self.topic_cursor.saturating_sub(1),
            Char('l') | Enter => {
                self.formula_cursor = 0;
                self.variant_idx = 0;
                self.reset_inputs();
                self.mode = Mode::FormulaList;
            }
            Char('q') => self.mode = Mode::TopicList, // handled by main loop
            _ => {}
        }
    }

    fn handle_formula_list(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        let n = self.current_topic().formulas.len();
        match key {
            Char('j') | Down => {
                self.formula_cursor = (self.formula_cursor + 1).min(n.saturating_sub(1))
            }
            Char('k') | Up => self.formula_cursor = self.formula_cursor.saturating_sub(1),
            Char('l') | Enter => {
                self.variant_idx = 0;
                self.input_cursor = 0;
                self.reset_inputs();
                self.mode = Mode::FormulaView;
                self.push_history();
            }
            Char('h') | Char('g') | Esc | Backspace => self.mode = Mode::TopicList,
            _ => {}
        }
    }

    fn handle_formula_view(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        match key {
            Tab => {
                if let Some(f) = self.current_topic().formulas.get(self.formula_cursor) {
                    let n = f.variants.len();
                    self.variant_idx = (self.variant_idx + 1) % n;
                    self.reset_inputs();
                }
            }
            Char('j') | Down => {
                let n = self.input_values.len();
                if n > 0 {
                    self.input_cursor = (self.input_cursor + 1).min(n - 1);
                }
            }
            Char('k') | Up => {
                self.input_cursor = self.input_cursor.saturating_sub(1);
            }
            Char('i') | Enter => {
                if !self.input_values.is_empty() {
                    self.edit_buffer = self.input_values[self.input_cursor].clone();
                    self.edit_is_fresh = true;
                    self.mode = Mode::InputEdit;
                }
            }
            Char('h') | Char('g') | Esc | Backspace => self.mode = Mode::FormulaList,
            _ => {}
        }
    }

    fn handle_input_edit(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        match key {
            Enter => {
                if !self.edit_buffer.is_empty() {
                    self.input_values[self.input_cursor] = self.edit_buffer.clone();
                }
                self.edit_buffer.clear();
                self.edit_is_fresh = false;
                self.mode = Mode::FormulaView;
                self.push_history();
            }
            Esc => {
                self.edit_buffer.clear();
                self.edit_is_fresh = false;
                self.mode = Mode::FormulaView;
            }
            Backspace => {
                if self.edit_is_fresh {
                    self.edit_buffer.clear();
                    self.edit_is_fresh = false;
                } else {
                    self.edit_buffer.pop();
                }
            }
            Char(c) => {
                if self.edit_is_fresh {
                    self.edit_buffer.clear();
                    self.edit_is_fresh = false;
                }
                self.edit_buffer.push(c);
            }
            _ => {}
        }
    }

    // ── Search ────────────────────────────────────────────────────────────────

    fn open_search(&mut self) {
        self.prev_topic = self.topic_cursor;
        self.prev_formula = self.formula_cursor;
        self.prev_variant = self.variant_idx;
        self.search_query.clear();
        self.search_cursor = 0;
        self.update_search_results();
        self.mode = Mode::Search;
    }

    fn update_search_results(&mut self) {
        let matcher = SkimMatcherV2::default();
        let mut results: Vec<SearchResult> = vec![];

        for (ci, ch) in self.topics.iter().enumerate() {
            for (fi, formula) in ch.formulas.iter().enumerate() {
                let label = format!("{}  ›  {}", ch.name, formula.name);
                let haystack = label.to_lowercase();
                let score = if self.search_query.is_empty() {
                    0
                } else {
                    matcher
                        .fuzzy_match(&haystack, &self.search_query.to_lowercase())
                        .unwrap_or(-1)
                };
                if self.search_query.is_empty() || score >= 0 {
                    results.push(SearchResult {
                        topic_idx: ci,
                        formula_idx: fi,
                        score,
                        label,
                    });
                }
            }
        }

        if !self.search_query.is_empty() {
            results.sort_by(|a, b| b.score.cmp(&a.score));
        }
        self.search_results = results;
        self.search_cursor = 0;
    }

    fn handle_search(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        match key {
            Esc => {
                self.topic_cursor = self.prev_topic;
                self.formula_cursor = self.prev_formula;
                self.variant_idx = self.prev_variant;
                self.search_query.clear();
                if self.prev_formula < self.topics[self.prev_topic].formulas.len() {
                    self.reset_inputs();
                    self.mode = Mode::FormulaView;
                } else {
                    self.mode = Mode::TopicList;
                }
            }
            Enter => {
                if let Some(r) = self.search_results.get(self.search_cursor).cloned() {
                    self.topic_cursor = r.topic_idx;
                    self.formula_cursor = r.formula_idx;
                    self.variant_idx = 0;
                    self.reset_inputs();
                    self.search_query.clear();
                    self.mode = Mode::FormulaView;
                    self.push_history();
                }
            }
            Char('j') | Down => {
                let n = self.search_results.len();
                if n > 0 {
                    self.search_cursor = (self.search_cursor + 1).min(n - 1);
                }
            }
            Char('k') | Up => {
                self.search_cursor = self.search_cursor.saturating_sub(1);
            }
            Backspace => {
                self.search_query.pop();
                self.update_search_results();
            }
            Char(c) => {
                self.search_query.push(c);
                self.update_search_results();
            }
            _ => {}
        }
    }

    pub fn should_quit(&self, key: crossterm::event::KeyCode) -> bool {
        key == crossterm::event::KeyCode::Char('q')
            && (self.mode == Mode::TopicList || self.mode == Mode::FormulaList)
    }
}

fn copy_to_clipboard(text: &str) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    if let Ok(mut child) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() {
        if let Some(stdin) = child.stdin.take() {
            let _ = std::io::BufWriter::new(stdin).write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

/// Format a value in engineering notation: exponent is always a multiple of 3.
/// E.g. 0.723 → "723e-3", 4700 → "4.7e3", 0.00015 → "150e-6".
fn format_eng_sci(value: f64) -> String {
    if value == 0.0 {
        return "0e0".to_string();
    }
    if !value.is_finite() {
        return format!("{value}");
    }
    let exp = value.abs().log10().floor() as i32;
    // Snap exponent down to the nearest multiple of 3.
    // For negatives, Rust's truncating division needs the bias of -2 to floor correctly.
    let eng_exp = if exp >= 0 {
        (exp / 3) * 3
    } else {
        ((exp - 2) / 3) * 3
    };
    let mantissa = value / 10f64.powi(eng_exp);
    let s = format!("{:.4}", mantissa);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if eng_exp == 0 {
        s.to_string()
    } else {
        format!("{s}e{eng_exp}")
    }
}
