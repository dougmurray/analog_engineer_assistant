mod app;
mod formulas;
mod input;
mod ui;

use std::io;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

#[derive(Parser)]
#[command(name = "analog-ref", about = "TI Analog Engineer's Pocket Reference — interactive TUI")]
struct Cli {
    /// Open the Conversions chapter
    #[arg(long)] conversions: bool,
    /// Open the The Basics chapter
    #[arg(long)] basics: bool,
    /// Open the Amplifiers chapter
    #[arg(long)] amplifiers: bool,
    /// Open the PCB and Wire chapter
    #[arg(long)] pcb: bool,
    /// Open the Sensor chapter
    #[arg(long)] sensor: bool,
    /// Open the Digital chapter
    #[arg(long)] digital: bool,
    /// Open the ADC chapter
    #[arg(long)] adc: bool,
    /// Open the DAC chapter
    #[arg(long)] dac: bool,
    /// Open the Multiplexer chapter
    #[arg(long)] multiplexer: bool,
}

impl Cli {
    fn jump_chapter(&self) -> Option<usize> {
        if self.conversions { Some(0) }
        else if self.basics  { Some(1) }
        else if self.amplifiers { Some(2) }
        else if self.pcb     { Some(3) }
        else if self.sensor  { Some(4) }
        else if self.digital { Some(5) }
        else if self.adc     { Some(6) }
        else if self.dac     { Some(7) }
        else if self.multiplexer { Some(8) }
        else { None }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut app = App::new(cli.jump_chapter());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if app.should_quit(key.code) {
                return Ok(());
            }
            app.handle_key(key.code);
        }
    }
}
