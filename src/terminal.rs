use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    ops::{Deref, DerefMut},
};
use tui::backend::CrosstermBackend;
use tui::Terminal as TuiTerminal;

pub struct Terminal(pub TuiTerminal<CrosstermBackend<Stdout>>);

impl Terminal {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        Self(TuiTerminal::new(backend).unwrap())
    }
}

impl Deref for Terminal {
    type Target = TuiTerminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.0.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.0.show_cursor().unwrap();
    }
}
