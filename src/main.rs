use crossterm::{
    event::{self, EnableMouseCapture, Event::Key, KeyCode, KeyEvent},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

mod application;

const APP_NAME: &str = "Maqi";

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let app = application::Application::new(APP_NAME, terminal);
    loop {
        if let Ok(e) = event::read() {
            match e {
                Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break,
                _ => {}
            }
        };
    }
    Ok(())
}
