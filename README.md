# Analog Engineer Assistant

**Analog Engineer Assistant** is an interactive terminal UI (TUI) that lets you look up common board and system level analog electrical engineer formulas. You can enter values with SI prefix shorthand and see results update in real time — without leaving the command line.

Covering multiple topics, every formula supports multiple *solve variants*: choose which variable to solve for and the app rearranges the equation accordingly. A full calculation history is kept for the session, and selected results can be copied to your system clipboad in engineering notation. 

## Features

- SI-prefix input — type `4.7u` for 4.7 µF, `10k` for 10 kΩ, `3.3M` for 3.3 MΩ
- Engineering-notation output — results displayed as `1.23 mV`, `47.0 kHz`, etc.
- Fuzzy search across all formula names with `/`
- Bode plots (ASCII) for RC and LC filters
- ASCII circuit diagrams for selected formulas (e.g., voltage divider)
- Per-session calculation history with variable copy support
- Direct topic launch via CLI flags for rapid access

## Installation

### Prerequisites

- Rust toolchain - install via [rustup.rs](https://rustup.rs/)
- A terminal emulator with 256-color and UTF-8 support (iTerm2, Ghostty, Alacritty, etc.)

### Build from source

1. Clone the repository  

    ```
    git clone https://github.com/<your-username>/analog_engineer_assistant.git
    cd analog_engineer_assistant
    ```
2. Build a release binary  

    ```
    cargo build --release
    ```

3. Optional: copy the binary into your PATH  

    ```
    cp target/release/analog_engineer_assistant /usr/local/bin/
    ```

> [!NOTE]
> The binary name is `analog_engineer_assistant`. A shorter alias `analog-ref` is also produced by the build. 

## Running
You can run it either by:  

```
cargo run                   # debug build, start at Topic List  
cargo run --release         # optimized build
```

Or by the installed binary:  

```
analog_engineer_assistant
```

### Jump directly to a topic
Skip the topic-list navigation and open a specific section on launch:  

```
analog_engineer_assistant --adc  
analog_engineer_assistant --amplifiers  
analog_engineer_assistant --basics  
analog_engineer_assistant --conversions  
analog_engineer_assistant --dac  
analog_engineer_assistant --digital  
analog_engineer_assistant --multiplexer  
analog_engineer_assistant --pcb  
analog_engineer_assistant --rf  
analog_engineer_assistant --sensor  
```

## License

Dual-license under MIT or the [UNLICENSE](https://unlicense.org/).
