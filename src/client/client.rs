use super::imap::ImapStream;
use std::io;

pub struct EmailClient {
    imap_stream: ImapStream,
    // TODO
    smtp_stream: (),
}

impl EmailClient {
    pub fn new(imap: String, _smtp: String) -> Result<Self, io::Error> {
        Ok(Self {
            imap_stream: ImapStream::new(imap)?,
            smtp_stream: (),
        })
    }
    pub fn init(&mut self) {
        self.imap_stream.init(todo!(), todo!());
        // SMTP initialization
        todo!();
    }
}
