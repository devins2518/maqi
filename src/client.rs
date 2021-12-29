use crate::{
    imap::{error::ImapError, ImapClient, ListPayload},
    utils::Provider,
};
use std::{error, fmt::Display, io};

// Stores connections to IMAP and SMTP servers separately.
pub struct EmailClient {
    imap: Vec<ImapClient>,
    // TODO
    // smtp
    index: usize,
}

impl EmailClient {
    pub fn new() -> Self {
        Self {
            imap: Vec::new(),
            index: 0,
        }
    }

    pub fn new_mailbox(&mut self, provider: Provider) -> Result<(), Error> {
        let (imap, _smtp) = provider.get_addresses();
        let mut split = imap.split(':').take(2);
        let client = ImapClient::new(split.next().unwrap(), split.next().unwrap())?;
        self.imap.push(client);
        self.inc();
        // TODO: SMTP
        Ok(())
    }

    pub fn pop(&mut self) {
        self.imap.pop();
        // TODO: SMTP
        self.dec();
    }

    pub fn authenticate(&mut self) {
        unimplemented!()
    }

    pub fn login(&mut self, user: &str, pass: &str) -> Result<(), Error> {
        self.imap[self.index - 1].login(user, pass)?;
        Ok(())
    }

    pub fn mailboxes(&mut self) -> Result<Vec<String>, Error> {
        Ok(self.imap[self.index - 1].list(ListPayload::simple("", "*"))?)
    }

    fn inc(&mut self) {
        self.index += 1;
    }

    fn dec(&mut self) {
        self.index -= 1;
    }

    fn set(&mut self, n: usize) {
        self.index = n;
    }
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ImapError(ImapError),
    // TODO: SmtpError(SmtpError)
}

impl From<ImapError> for Error {
    fn from(e: ImapError) -> Self {
        Self::ImapError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = match self {
            Error::IO(e) => format!("IO Error: {}", e),
            Error::ImapError(e) => format!("IMAP Error: {}", e),
        };
        f.write_str(&e)
    }
}

impl error::Error for Error {}
