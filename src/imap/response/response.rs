use crate::imap::{tag::Tag, tokens::Token};

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

impl From<&str> for ImapResponse {
    fn from(s: &str) -> Self {
        match s {
            "OK" => ImapResponse::Ok,
            "NO" => ImapResponse::No,
            "BAD" => ImapResponse::Bad,
            "PREAUTH" => ImapResponse::Preauth,
            "BYE" => ImapResponse::Bye,
            "ENABLED" => ImapResponse::Enabled,
            "CAPABILITY" => ImapResponse::Capability,
            "LIST" => ImapResponse::List,
            "NAMESPACE" => ImapResponse::Namespace,
            "STATUS" => ImapResponse::Status,
            "ESEARCH" => ImapResponse::Esearch,
            "FLAGS" => ImapResponse::Flags,
            n if n.parse::<u32>().is_ok() => ImapResponse::Size(n.parse().unwrap()),
            _ => ImapResponse::Continuation,
        }
    }
}
