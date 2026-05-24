# Overall Plan

Based on the Texas Instruments Analog Engineer's Pocket Reference pdf, I'd like a TUI in Rust that gives a user an interface for accessing and using the formula the pdf states. As the user selects the formula, a box on the right shows the formula and the user can enter in values for the variables and watch the output change accordingly.

## Technology to use

- Rust
- crates:
    - ratatui
    - clippy (for testing Rust program)
    - fmt (for formating)
    - clap --features derive

## Design and look

The overall interface should include:

- outline boxes
    - one on left with the analog reference chapters (on the left)
        - covers 1/3 the TUI
    - one on the right with the preview of the formulas
        - cover 2/3 of TUI
- nested components

### Actions and interactions

- when a chapter topic is selected the right outline box shows the next level of the subchapter
- when a subchapter is selected the subchapter is displayed in the left box, and the right box has the formula
- then the variables can be selected by the user and changed
    - scientific notation allowed for entering values (like 1e3 for 1,000 for example)
    - for appropriate variables the user can type in the unit format (like 1 k for 1,000, or 1u for 1e-6)
- have the motions be Vim compatable (j is down, k is up, h is left, l is right, etc.)

See the openapi-tui repo in the resources for reference in how to implement these features.
