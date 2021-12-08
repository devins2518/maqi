use super::imap::ImapClient;
use std::io;

pub struct EmailClient {
    imap: ImapClient,
    // TODO
    smtp: (),
}

impl EmailClient {
    /// (IMAP, SMTP)
    pub fn new(imap: &str, _smtp: &str) -> Result<Self, io::Error> {
        Ok(Self {
            imap: ImapClient::new(imap)?,
            smtp: (),
        })
    }
    pub fn init(&mut self) {
        self.imap.init(todo!(), todo!());
        // SMTP initialization
        todo!();
    }
}
