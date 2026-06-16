mod app;
mod formulas;
mod input;
mod ui;

use std::io;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;

#[derive(Parser)]
#[command(
    name = "analog_engineer_assistant",
    about = "TI Analog Engineer's Pocket Reference — interactive TUI"
)]
struct Cli {
    /// Open the Conversions topic
    #[arg(long)]
    conversions: bool,
    /// Open the The Basics topic
    #[arg(long)]
    basics: bool,
    /// Open the Amplifiers topic
    #[arg(long)]
    amplifiers: bool,
    /// Open the PCB and Wire topic
    #[arg(long)]
    pcb: bool,
    /// Open the Sensor topic
    #[arg(long)]
    sensor: bool,
    /// Open the Digital topic
    #[arg(long)]
    digital: bool,
    /// Open the ADC topic
    #[arg(long)]
    adc: bool,
    /// Open the DAC topic
    #[arg(long)]
    dac: bool,
    /// Open the Multiplexer topic
    #[arg(long)]
    multiplexer: bool,
    /// Open the RF topic
    #[arg(long)]
    rf: bool,
}

impl Cli {
    fn jump_topic(&self) -> Option<usize> {
        if self.conversions {
            Some(0)
        } else if self.basics {
            Some(1)
        } else if self.amplifiers {
            Some(2)
        } else if self.pcb {
            Some(3)
        } else if self.sensor {
            Some(4)
        } else if self.digital {
            Some(5)
        } else if self.adc {
            Some(6)
        } else if self.dac {
            Some(7)
        } else if self.multiplexer {
            Some(8)
        } else if self.rf {
            Some(9)
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut app = App::new(cli.jump_topic());

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

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if app.should_quit(key.code) {
                return Ok(());
            }
            app.handle_key(key);
        }
    }
}
