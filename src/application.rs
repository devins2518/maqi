use crate::{client::EmailClient, ui};
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use std::io::{self, Stdout};
use tui::backend::CrosstermBackend;

pub struct Application {
    pub title: &'static str,
    pub spans: Vec<String>,
    email_client: EmailClient,
}

impl<'a> Application {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            spans: Vec::new(),
            // TODO: using google
            email_client: EmailClient::new("imap.gmail.com:993", "smtp.gmail.com:587").unwrap(),
        }
    }

    pub fn run(&mut self, term: &mut tui::Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            term.draw(|f| ui::draw(f, self))?;

            if let Ok(e) = event::read() {
                match e {
                    Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    }) => break,
                    Key(KeyEvent {
                        code: KeyCode::Char('t'),
                        ..
                    }) => self.spans.push(String::from("hey")),
                    _ => {}
                }
            };
        }
        Ok(())
    }
}
