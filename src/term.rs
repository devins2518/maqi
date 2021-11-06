use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{stdout, Stdout};
use tuirealm::tui::{backend::CrosstermBackend, Terminal as TuiTerminal};

pub struct Terminal {
    pub terminal: TuiTerminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Self {
        let _ = enable_raw_mode();
        // Create terminal
        let mut stdout = stdout();
        assert!(execute!(stdout, EnterAlternateScreen).is_ok());
        Self {
            terminal: TuiTerminal::new(CrosstermBackend::new(stdout)).unwrap(),
        }
    }

    pub fn leave_alternate_screen(&mut self) {
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }

    pub fn clear_screen(&mut self) {
        let _ = self.terminal.clear();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Re-enable terminal stuff
        self.leave_alternate_screen();
        let _ = disable_raw_mode();
    }
}
