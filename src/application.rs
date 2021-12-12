use crate::{client::EmailClient, terminal::Terminal, ui};
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use std::io;

pub struct Application {
    terminal: Terminal,
    pub title: &'static str,
    spans: Vec<String>,
    email_client: EmailClient,
}

impl<'a> Application {
    pub fn new(title: &'static str) -> Self {
        // TODO: using google
        let mut email_client = EmailClient::new();
        // TODO: move out of application creation once configuration is finished
        if let Err(_e) = email_client.connect("imap.gmail.com:993", "smtp.gmail.com:587") {
            // TODO: Own terminal and draw error messages
            // todo!()
            ()
        }
        Self {
            terminal: Terminal::new(),
            title,
            spans: Vec::new(),
            email_client,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.0.draw(|f| ui::draw(f, &self.spans))?;

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
