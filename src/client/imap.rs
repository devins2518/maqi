use std::{
    fmt::{self, Display},
    io::{self, Write},
    net::TcpStream,
    str::{self, FromStr},
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
    stream: TcpStream,
    tag: TagRepr,
    state: State,
}

impl ImapStream {
    pub fn new(addr: String) -> Result<Self, io::Error> {
        Ok(Self {
            stream: TcpStream::connect(addr)?,
            tag: TagRepr::new(),
            state: State::NotAuthenticated,
        })
    }

    pub fn init(&mut self, _user: &str, _pass: &str) {
        todo!("TLS stuff");
    }

    pub fn send(&mut self, command: Command, body: &str) {
        let c_str = COMMAND_STR[command as usize];
        self.stream
            .write(format!("{} {} {}", self.tag, c_str, body).as_bytes());
        self.tag.inc()
    }
}

enum Tag {
    Tag(TagRepr),
    Continuation,
}

struct TagRepr {
    alpha: char,
    numeric: u16,
}

impl TagRepr {
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

impl<T> From<T> for TagRepr
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let s = s.as_ref();
        let c = s.as_bytes();
        Self {
            alpha: char::from(c[0]),
            numeric: u16::from_str(str::from_utf8(&c[1..]).unwrap()).unwrap(),
        }
    }
}

impl Display for TagRepr {
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

const RESPONSE_CODE_STR: [&str; 38] = [
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

enum ImapResult {
    ImapOk,
    ImapError,
}

struct ServerResponse {
    tag: Option<Tag>,
    result: ImapResult,
    response_code: Option<ResponseCode>,
    msg: Option<String>,
}

impl<T> From<T> for ServerResponse
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let s = s.as_ref();
        let mut splits = s.split(" ");
        let tag;
        let result = todo!();
        let response_code = todo!();
        let msg = todo!();
        match splits.next().unwrap() {
            "*" => {
                tag = None;
            }
            "+" => tag = Some(Tag::Continuation),
            s => tag = Some(Tag::Tag(TagRepr::from(s))),
        };

        Self {
            tag,
            result,
            response_code,
            msg,
        }
    }
}

enum ImapOk {
    Ok(Option<String>),
    Preauth(Option<String>),
    Bye(Option<String>),
}

enum ImapError {
    No(Option<String>),
    Bad(Option<String>),
}

enum ResponseCode {
    // TODO: hide until TLS auth is ensured
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

enum State {
    NotAuthenticated,
    Authenticated,
    Selected,
    Logout,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag() {
        let mut t = TagRepr::new();
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

    #[test]
    fn test_response_parsing() {
        let rs = [
            "* OK [CAPABILITY IMAP4rev2 STARTTLS AUTH=GSSAPI",
            "A01 OK STARTTLS complete",
            "* CAPABILITY IMAP4rev2 AUTH=GSSAPI AUTH=PLA",
            "* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft",
            "* LIST () \" / \" blurdybloop",
            "A932 OK [READ-ONLY] EXAMINE complet",
        ];
    }
}
