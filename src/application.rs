use crate::{
    client::EmailClient,
    terminal::Terminal,
    ui::{self, UI},
};
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use log::info;
use std::io;

pub struct Application {
    terminal: Terminal,
    pub title: &'static str,
    spans: Vec<String>,
    email_client: EmailClient,
    ui: UI,
}

impl<'a> Application {
    pub fn new(title: &'static str) -> Self {
        let terminal = Terminal::new();
        let rect = terminal.size().unwrap();
        // TODO: using google
        // TODO: move out of application creation once configuration is finished
        /* if let Err(_e) = email_client.connect("imap.gmail.com:993", "smtp.gmail.com:587") {
        /     // TODO: Own terminal and draw error messages
        /     // todo!()
        /     ()
         } */
        Self {
            terminal,
            title,
            spans: Vec::new(),
            email_client: EmailClient::new(),
            ui: UI::new(&rect),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| self.ui.draw(f, &self.spans))?;

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
                    Key(KeyEvent {
                        code: KeyCode::Char('l'),
                        ..
                    }) => self.login()?,
                    _ => {}
                }
            };
        }
        Ok(())
    }

    fn login(&mut self) -> io::Result<()> {
        let mut user = String::new();
        let mut pass = String::new();
        self.terminal.draw(|f| {
            user = self.ui.prompt(f, "Please enter username: ");
            pass = self.ui.prompt(f, "Please enter password: ");
        })?;
        unimplemented!()
    }
}
