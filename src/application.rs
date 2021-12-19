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
            email_client: EmailClient::new(),
            ui: UI::new(&rect),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| self.ui.draw(f))?;

            if let Ok(e) = event::read() {
                match e {
                    Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    }) => break,
                    Key(KeyEvent {
                        code: KeyCode::Char('t'),
                        ..
                    }) => self.ui.spans.push(String::from("hey")),
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
        let user = self
            .ui
            .prompt(&mut self.terminal, "Please enter username: ");
        let pass = self
            .ui
            .prompt(&mut self.terminal, "Please enter password: ");
        info!("user {}", user);
        info!("pass {}", pass);
        Ok(())
    }
}
