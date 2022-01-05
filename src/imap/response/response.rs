use crate::imap::tokens::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum ImapResponse {
    Continuation,
    // Generic
    Ok,
    No,
    Bad,
    Preauth,
    Bye,
    // ServerStatus
    Enabled,
    Capability,
    // MailboxStatus
    List,
    Namespace,
    Status,
    Esearch,
    Flags,
    // MailboxSize
    Size(usize),
}

impl From<&Token> for ImapResponse {
    fn from(t: &Token) -> Self {
        match t {
            Token::Ok => ImapResponse::Ok,
            Token::No => ImapResponse::No,
            Token::Bad => ImapResponse::Bad,
            Token::PreAuth => ImapResponse::Preauth,
            Token::Bye => ImapResponse::Bye,
            Token::Enabled => ImapResponse::Enabled,
            Token::Capability => ImapResponse::Capability,
            Token::List => ImapResponse::List,
            Token::Namespace => ImapResponse::Namespace,
            Token::Status => ImapResponse::Status,
            Token::ESearch => ImapResponse::Esearch,
            Token::Flags => ImapResponse::Flags,
            Token::Other(n) => ImapResponse::Size(n.parse().unwrap()),
            _ => ImapResponse::Continuation,
        }
    }
}
