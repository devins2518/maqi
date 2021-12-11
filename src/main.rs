use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

mod application;
mod client;
mod imap;
mod terminal;
mod ui;
mod utils;

const APP_NAME: &str = "Maqi";

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = application::Application::new(APP_NAME);
    if let Err(e) = app.run() {
        eprintln!("Error received when running maqi!\n{}", e);
        std::process::exit(1);
    }

    Ok(())
}
