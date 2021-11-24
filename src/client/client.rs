use super::imap::ImapStream;

pub struct EmailClient {
    imap_stream: ImapStream,
    // TODO
    smtp_stream: (),
}
