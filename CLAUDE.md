# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust TUI (terminal user interface) for analog electrical engineers, modeled after the Texas Instruments Analog Engineer's Pocket Reference. Users browse chapters, select formulas, enter variable values (with SI prefix support), and see computed results update live.

Binary name: `analog_engineer_assistant`

## Commands

```bash
# Run in development
cargo run

# Run with release optimizations
cargo run --release

# Jump directly to a chapter on launch
cargo run -- --adc
cargo run -- --amplifiers   # other flags: --conversions, --basics, --pcb, --sensor, --digital, --dac, --multiplexer

# Build release binary
cargo build --release   # outputs to target/release/analog_engineer_assistant

# Run tests
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

## Architecture

The app is split into four modules:

- **`app.rs`** — Central state machine (`App` struct). Owns the `Mode` enum (`ChapterList → FormulaList → FormulaView → InputEdit`, plus `Search`), cursor positions, input buffers, and fuzzy search state. All key-event routing lives here.
- **`formulas/`** — Pure data. Each file (e.g. `amplifiers.rs`, `adc.rs`) exports a `formulas() -> Vec<FormulaEntry>` function. `FormulaEntry` holds a name and a slice of `SolveVariant`s; each variant has a `compute: fn(&[f64]) -> f64` closure plus metadata (`inputs`, `expression`, `output_unit`). `mod.rs` assembles all chapters via `all_chapters()`.
- **`ui/`** — Rendering only. `mod.rs` does the top-level layout (1/3 nav | 2/3 formula panel | bottom search bar). `nav.rs` renders the chapter/formula list. `formula.rs` renders the active formula, input fields, and live result.
- **`input.rs`** — Two pure functions: `parse_value` (SI-prefix-aware parser: `"4.7u"` → `4.7e-6`) and `format_eng` (formats `f64` back to engineering notation). Has unit tests.
- **`main.rs`** — CLI parsing via `clap` derive, crossterm terminal setup/teardown, and the event loop.

### Navigation flow

```
ChapterList  →(Enter/l)→  FormulaList  →(Enter/l)→  FormulaView  →(Enter/i)→  InputEdit
             ←(Esc/h/g)←               ←(Esc/h/g)←               ←(Esc/Enter)←
```

`/` opens fuzzy `Search` from any non-`InputEdit` mode. Tab cycles `SolveVariant`s within a formula. `q` quits from `ChapterList` or `FormulaList`.

### Adding a new formula

1. Open the relevant file in `src/formulas/` (e.g. `amplifiers.rs`).
2. Push a new `FormulaEntry` into the `vec![]` returned by `formulas()`.
3. Each `SolveVariant` needs: `solves_for`, `expression` (display string), `inputs` (`&[VarDef]`), `output_unit`, and `compute` (a `fn(&[f64]) -> f64` — inputs arrive in the same order as the `VarDef` slice).

### Adding a new chapter

1. Create `src/formulas/<chapter>.rs` following the pattern of any existing file.
2. Declare it in `src/formulas/mod.rs` (`pub mod <chapter>;`).
3. Add a `Chapter` entry in `all_chapters()`.
4. Add a `--<chapter>` CLI flag in `main.rs` and map it in `Cli::jump_chapter`.

## Key dependencies

| Crate | Role |
|---|---|
| `ratatui` | TUI rendering |
| `crossterm` | Terminal backend & raw-mode events |
| `clap` (derive) | CLI argument parsing |
| `fuzzy-matcher` | Skim-algorithm fuzzy search |
