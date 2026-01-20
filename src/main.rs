mod app;
mod learning;
mod ui;
mod vim;

use app::App;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use ui::EventHandler;

#[derive(Parser)]
#[command(name = "vex")]
#[command(about = "VEX - Vim Movement Trainer", long_about = None)]
struct Cli {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();

    let mut terminal = setup_terminal()?;

    let mut app = App::new();
    let event_handler = EventHandler::new();

    let res = run_app(&mut terminal, &mut app, &event_handler);

    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    event_handler: &EventHandler,
) -> io::Result<()> {
    while app.is_running() {
        terminal.draw(|f| ui::render_ui(f, app))?;

        if let Some(event) = event_handler.poll_event(std::time::Duration::from_millis(100))? {
            app.handle_event(event);
        }
    }

    Ok(())
}
