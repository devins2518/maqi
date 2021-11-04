use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::io::Stdout;
use tui::backend::CrosstermBackend;

pub struct Application {
    pub title: &'static str,
    term: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Application {
    pub fn new(title: &'static str, term: tui::Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self { title, term }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.term.show_cursor().unwrap();
    }
}
