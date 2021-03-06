use crate::{
    client::{EmailClient, Error},
    terminal::Terminal,
    ui::UI,
    utils::{Event, Provider},
};
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use log::{error, info, warn};
use std::io;

pub struct Application {
    pub title: &'static str,
    email_client: EmailClient,
    ui: UI,
    terminal: Terminal,
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        let terminal = Terminal::new();
        Self {
            title,
            email_client: EmailClient::new(),
            ui: UI::new(&terminal),
            terminal,
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
                    }) => self.ui.new_tab(String::from("hey")),
                    Key(KeyEvent {
                        code: KeyCode::Char('l'),
                        ..
                    }) => {
                        if let Err(e) = self.login() {
                            self.error(&e.to_string());
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn info(&mut self, msg: &str) {
        info!("Info logged: {}", msg);
        self.ui.info(msg);
    }

    fn warning(&mut self, msg: &str) {
        warn!("Warning logged: {}", msg);
        self.ui.warning(msg);
    }

    fn error(&mut self, msg: &str) {
        error!("Error logged: {}", msg);
        self.ui.error(msg);
    }

    // TODO: async this
    fn login(&mut self) -> Result<(), Error> {
        let user = self.prompt("Please enter username: ");
        let pass = self.prompt("Please enter password: ");
        self.email_client.new_mailbox(Provider::ICloud)?;
        if let Err(e) = self.email_client.login(&user, &pass) {
            self.email_client.pop();
            return Err(e);
        }
        // TODO: LIST is very unfinished
        // self.ui.set_mailbox_names(self.email_client.mailboxes()?);
        Ok(())
    }

    fn prompt(&mut self, msg: &str) -> String {
        let mut prompt = self.ui.prompt(msg);
        loop {
            self.terminal
                .draw(|f| {
                    self.ui.draw(f);
                    prompt.draw(f);
                })
                .unwrap();

            if let Event::Break = prompt.handle_event() {
                break;
            }
        }
        prompt.response
    }
}
