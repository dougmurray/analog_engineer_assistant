use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::formulas::{all_chapters, Chapter};
use crate::input::{format_eng, parse_value};

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    ChapterList,
    FormulaList,
    FormulaView,
    InputEdit,
    Search,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub chapter_idx: usize,
    pub formula_idx: usize,
    pub score: i64,
    pub label: String,
}

pub struct App {
    pub chapters: Vec<Chapter>,
    pub mode: Mode,

    // Navigation
    pub chapter_cursor: usize,
    pub formula_cursor: usize,
    pub variant_idx: usize,   // which "solve for" variant is active
    pub input_cursor: usize,  // which variable field is focused

    // Per-formula input values (indexed by variant input slot)
    pub input_values: Vec<String>,
    pub edit_buffer: String,
    pub edit_is_fresh: bool,

    // Search
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_cursor: usize,

    // Previous mode to restore on Esc from search
    prev_chapter: usize,
    prev_formula: usize,
    prev_variant: usize,
}

impl App {
    pub fn new(jump_chapter: Option<usize>) -> Self {
        let chapters = all_chapters();
        let chapter_cursor = jump_chapter.unwrap_or(0).min(chapters.len().saturating_sub(1));
        let mode = if jump_chapter.is_some() {
            Mode::FormulaList
        } else {
            Mode::ChapterList
        };
        let mut app = Self {
            chapters,
            mode,
            chapter_cursor,
            formula_cursor: 0,
            variant_idx: 0,
            input_cursor: 0,
            input_values: vec![],
            edit_buffer: String::new(),
            edit_is_fresh: false,
            search_query: String::new(),
            search_results: vec![],
            search_cursor: 0,
            prev_chapter: 0,
            prev_formula: 0,
            prev_variant: 0,
        };
        app.reset_inputs();
        app
    }

    pub fn current_chapter(&self) -> &Chapter {
        &self.chapters[self.chapter_cursor]
    }

    pub fn current_variant(&self) -> Option<&crate::formulas::SolveVariant> {
        let ch = self.current_chapter();
        if self.formula_cursor >= ch.formulas.len() {
            return None;
        }
        let f = &ch.formulas[self.formula_cursor];
        f.variants.get(self.variant_idx)
    }

    fn reset_inputs(&mut self) {
        if let Some(v) = self.current_variant() {
            self.input_values = v
                .inputs
                .iter()
                .map(|vd| format_eng(vd.default))
                .collect();
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

    // ── Key handlers ─────────────────────────────────────────────────────────

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;

        // "/" always opens search from any non-edit mode
        if key == Char('/') && self.mode != Mode::InputEdit {
            self.open_search();
            return;
        }

        match self.mode.clone() {
            Mode::ChapterList  => self.handle_chapter_list(key),
            Mode::FormulaList  => self.handle_formula_list(key),
            Mode::FormulaView  => self.handle_formula_view(key),
            Mode::InputEdit    => self.handle_input_edit(key),
            Mode::Search       => self.handle_search(key),
        }
    }

    fn handle_chapter_list(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        match key {
            Char('j') | Down  => self.chapter_cursor = (self.chapter_cursor + 1).min(self.chapters.len() - 1),
            Char('k') | Up    => self.chapter_cursor = self.chapter_cursor.saturating_sub(1),
            Char('l') | Enter => {
                self.formula_cursor = 0;
                self.variant_idx    = 0;
                self.reset_inputs();
                self.mode = Mode::FormulaList;
            }
            Char('q') => self.mode = Mode::ChapterList, // handled by main loop
            _ => {}
        }
    }

    fn handle_formula_list(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        let n = self.current_chapter().formulas.len();
        match key {
            Char('j') | Down  => self.formula_cursor = (self.formula_cursor + 1).min(n.saturating_sub(1)),
            Char('k') | Up    => self.formula_cursor = self.formula_cursor.saturating_sub(1),
            Char('l') | Enter => {
                self.variant_idx  = 0;
                self.input_cursor = 0;
                self.reset_inputs();
                self.mode = Mode::FormulaView;
            }
            Char('h') | Char('g') | Esc | Backspace => self.mode = Mode::ChapterList,
            _ => {}
        }
    }

    fn handle_formula_view(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode::*;
        match key {
            Tab => {
                if let Some(f) = self.current_chapter().formulas.get(self.formula_cursor) {
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
        self.prev_chapter = self.chapter_cursor;
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

        for (ci, ch) in self.chapters.iter().enumerate() {
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
                        chapter_idx: ci,
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
                self.chapter_cursor = self.prev_chapter;
                self.formula_cursor = self.prev_formula;
                self.variant_idx    = self.prev_variant;
                self.search_query.clear();
                if self.prev_formula < self.chapters[self.prev_chapter].formulas.len() {
                    self.reset_inputs();
                    self.mode = Mode::FormulaView;
                } else {
                    self.mode = Mode::ChapterList;
                }
            }
            Enter => {
                if let Some(r) = self.search_results.get(self.search_cursor).cloned() {
                    self.chapter_cursor = r.chapter_idx;
                    self.formula_cursor = r.formula_idx;
                    self.variant_idx    = 0;
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
            && (self.mode == Mode::ChapterList || self.mode == Mode::FormulaList)
    }
}

