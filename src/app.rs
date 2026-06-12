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

    // ── Key handlers ─────────────────────────────────────────────────────────

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;

        // "/" always opens search from any non-edit mode
        if key == Char('/') && self.mode != Mode::InputEdit {
            self.open_search();
            return;
        }

        match self.mode.clone() {
            Mode::TopicList => self.handle_topic_list(key),
            Mode::FormulaList => self.handle_formula_list(key),
            Mode::FormulaView => self.handle_formula_view(key),
            Mode::InputEdit => self.handle_input_edit(key),
            Mode::Search => self.handle_search(key),
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
