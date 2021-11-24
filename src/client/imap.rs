use std::{
    fmt::{self, Display},
    net::TcpStream,
};

/// TODO:
/// From RFC 9051:
/// [ ] Manipulation of remote mailboxes
/// [ ] Local mailbox synchronization
/// [ ] Create, delete, rename mailboxes
/// [ ] Check for new messages
/// [ ] Remove messages permanently
/// [ ] Set and clear message flags
/// [ ] Parsing per RFC 5322, 2045, 2231
/// [ ] Selective fetching of message attrs, texts, and portions

pub struct ImapStream {
    // TCP Stream
    stream: TcpStream,
    // Tag
    tag: Tag,
}

struct Tag {
    alpha: char,
    numeric: u16,
}

impl Tag {
    fn new() -> Self {
        Self {
            alpha: 'A',
            numeric: 0001,
        }
    }
    fn inc(&mut self) {
        if self.numeric == 9999 {
            // TODO: increase letter
            self.alpha = if self.alpha == 'Z' {
                'A'
            } else {
                std::char::from_u32(self.alpha as u32 + 1).unwrap_or(self.alpha)
            };
            self.numeric = 1;
        } else {
            self.numeric += 1;
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{:>04}", self.alpha, self.numeric))
    }
}

const COMMAND_STR: [&str; 28] = [
    "CAPABILITY",
    "NOOP",
    "LOGOUT",
    "STARTTLS",
    "AUTHENTICATE",
    "LOGIN",
    "ENABLE",
    "SELECT",
    "EXAMINE",
    "CREATE",
    "DELETE",
    "RENAME",
    "SUBSCRIBE",
    "UNSUBSCRIBE",
    "LIST",
    "NAMESPACE",
    "STATUS",
    "APPEND",
    "IDLE",
    "CLOSE",
    "UNSELECT",
    "EXPUNGE",
    "SEARCH",
    "FETCH",
    "STORE",
    "COPY",
    "MOVE",
    "UID",
];

enum Command {
    // Any state
    CAPABILITY,
    NOOP,
    LOGOUT,
    // Not Auth state
    STARTTLS,
    AUTHENTICATE,
    LOGIN,
    // Auth state
    ENABLE,
    SELECT,
    EXAMINE,
    CREATE,
    DELETE,
    RENAME,
    SUBSCRIBE,
    UNSUBSCRIBE,
    LIST,
    NAMESPACE,
    STATUS,
    APPEND,
    IDLE,
    // Select state
    CLOSE,
    UNSELECT,
    EXPUNGE,
    SEARCH,
    FETCH,
    STORE,
    COPY,
    MOVE,
    UID,
}

const RESPONSE_STR: [&str; 38] = [
    "OK",
    "NO",
    "BAD",
    "PREAUTH",
    "BYE",
    "ALERT",
    "ALREADYEXISTS",
    "APPENDUID",
    "AUTHENTICATIONFAILED",
    "AUTHORIZATIONFAILED",
    "BADCHARSET",
    "CANNOT",
    "CAPABILITY",
    "CLIENTBUG",
    "CLOSED",
    "CONTACTADMIN",
    "COPYUID",
    "CORRUPTION",
    "EXPIRATION",
    "EXPUNGEISSUED",
    "HASCHILDREN",
    "INUSE",
    "LIMIT",
    "NONEXISTENT",
    "NOPERM",
    "OVERQUOTA",
    "PARSE",
    "PERMANENTFLAGS",
    "PRIVACYREQUIRED",
    "READ-ONLY",
    "READ-WRITE",
    "SERVERBUG",
    "TRYCREATE",
    "UIDNEXT",
    "UIDNOTSTICKY",
    "UIDVAILIDITY",
    "UNAVAILABLE",
    "UNKNOWN-CTE",
];

enum Response {
    Ok(Option<String>),
    No(Option<String>),
    Bad(Option<String>),
    Preauth(Option<String>),
    Bye(Option<String>),
    Alert,
    AlreadyExists,
    AppendUid,
    AuthenticationFailed,
    AuthorizationFailed,
    BadCharset,
    Cannot,
    Capability,
    ClientBug,
    Closed,
    ContactAdmin,
    CopyUid,
    Corruption,
    Expiration,
    ExpungeIssued,
    HasChildren,
    InUse,
    Limit,
    NonExistent,
    NoPerm,
    // TODO: implement RFC 2087 for quota capability
    OverQuota,
    Parse,
    PermanentFlags,
    PrivacyRequired,
    ReadOnly,
    ReadWrite,
    ServerBug,
    TryUreate,
    UidNext,
    UidNotSticky,
    UidValidity,
    Unavailable,
    UnknownCte,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag() {
        let mut t = Tag::new();
        // Increase number
        for i in 1..=9999 {
            assert_eq!(format!("A{:>04}", i), format!("{}", t));
            t.inc();
        }

        // Increase letter
        assert_eq!(String::from("B0001"), format!("{}", t));

        // Wraparound
        for _ in 0..(25 * 9999) {
            t.inc();
        }
        assert_eq!(String::from("A0001"), format!("{}", t));
    }
}
