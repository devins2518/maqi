use crate::imap::{error::ImapError, ImapClient};
use std::io;

pub struct EmailClient {
    imap: ImapClient,
    // TODO
    smtp: (),
}

impl EmailClient {
    /// (IMAP, SMTP)
    pub fn new(imap: &str, _smtp: &str) -> Result<Self, io::Error> {
        let mut split = imap.split(':').take(2);
        Ok(Self {
            imap: ImapClient::new(split.next().unwrap(), split.next().unwrap())?,
            smtp: (),
        })
    }
    pub fn init(&mut self) -> Result<(), Error> {
        let user = std::env::var("EMAIL_USER").unwrap_or(String::from(""));
        let pass = std::env::var("EMAIL_PASS").unwrap_or(String::from(""));
        if let Err(e) = self.imap.init(&user, &pass) {
            return Err(Error::ImapError(e));
        }
        // TODO
        // SMTP initialization
        Ok(())
    }
}

pub enum Error {
    ImapError(ImapError),
    // SmtpError(SmtpError)
}
