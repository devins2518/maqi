use crate::imap::error::Result as IResult;

use std::fmt::{self, Display};
use std::str::{self, FromStr};

use super::error::ImapError;
use super::scanner::Scanner;
use super::tokens::Token;

#[derive(Debug, PartialEq, Eq)]
enum Tag {
    Real(TagRepr),
    ServerContinuation,
    ClientContinuation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TagRepr {
    alpha: char,
    numeric: u16,
}

impl TagRepr {
    pub fn new() -> Self {
        Self {
            alpha: 'A',
            numeric: 0001,
        }
    }
    pub fn inc(&mut self) {
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

pub enum Command<'a> {
    // Any state
    Capability,
    Noop,
    Logout,
    // Not Auth state
    StartTLS,
    Authenticate,
    //    user    pass
    Login(&'a str, &'a str),
    // Auth state
    Enable,
    Select,
    Examine,
    Create,
    Delete,
    Rename,
    Subscribe,
    Unsubscribe,
    List,
    Namespace,
    Status,
    Append,
    Idle,
    // Select state
    Close,
    Unselect,
    Expunge,
    Search,
    Fetch,
    Store,
    Copy,
    Move,
    UID,
}

impl<'a> Command<'a> {
    pub fn check(&self, state: &State) -> IResult<()> {
        match (self, state) {
            (Command::Capability | Command::Noop | Command::Logout, _) => Ok(()),
            (
                Command::StartTLS | Command::Authenticate | Command::Login(_, _),
                State::NotAuthenticated,
            ) => Ok(()),
            (
                Command::Enable
                | Command::Select
                | Command::Examine
                | Command::Create
                | Command::Delete
                | Command::Rename
                | Command::Subscribe
                | Command::Unsubscribe
                | Command::List
                | Command::Namespace
                | Command::Status
                | Command::Append
                | Command::Idle,
                State::Authenticated,
            ) => Ok(()),
            (
                Command::Close
                | Command::Unselect
                | Command::Expunge
                | Command::Search
                | Command::Fetch
                | Command::Store
                | Command::Copy
                | Command::Move
                | Command::UID,
                State::Selected,
            ) => Ok(()),
            _ => Err(ImapError::InvalidState),
        }
    }
}

impl<'a> Display for Command<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: possible to remove allocations
        let s = match self {
            Command::Capability => "CAPABILITY".to_string(),
            Command::Noop => "NOOP".to_string(),
            Command::Logout => "LOGOUT".to_string(),
            Command::StartTLS => "STARTTLS".to_string(),
            Command::Authenticate => "AUTHENTICATE".to_string(),
            Command::Login(user, pass) => format!("LOGIN {} {}", user, pass),
            Command::Enable => "ENABLE".to_string(),
            Command::Select => "SELECT".to_string(),
            Command::Examine => "EXAMINE".to_string(),
            Command::Create => "CREATE".to_string(),
            Command::Delete => "DELETE".to_string(),
            Command::Rename => "RENAME".to_string(),
            Command::Subscribe => "SUBSCRIBE".to_string(),
            Command::Unsubscribe => "UNSUBSCRIBE".to_string(),
            Command::List => "LIST".to_string(),
            Command::Namespace => "NAMESPACE".to_string(),
            Command::Status => "STATUS".to_string(),
            Command::Append => "APPEND".to_string(),
            Command::Idle => "IDLE".to_string(),
            Command::Close => "CLOSE".to_string(),
            Command::Unselect => "UNSELECT".to_string(),
            Command::Expunge => "EXPUNGE".to_string(),
            Command::Search => "SEARCH".to_string(),
            Command::Fetch => "FETCH".to_string(),
            Command::Store => "STORE".to_string(),
            Command::Copy => "COPY".to_string(),
            Command::Move => "MOVE".to_string(),
            Command::UID => "UID".to_string(),
        };
        f.write_str(&s)
    }
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

#[derive(Debug, PartialEq, Eq)]
pub struct ServerResponse {
    tag: Tag,
    response: ImapResponse,
    msg: Option<Vec<Token>>,
}

impl ServerResponse {
    pub fn is_continuation(&self) -> bool {
        self.tag == Tag::ServerContinuation
    }
}

impl<T> From<T> for ServerResponse
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let mut scanner = Scanner::new(s.as_ref());
        scanner.scan_tokens();
        let mut iter = scanner.tokens.into_iter().peekable();
        let tag;
        let response;
        tag = match iter.next().unwrap() {
            Token::STAR => Tag::ServerContinuation,
            Token::PLUS => Tag::ClientContinuation,
            Token::Other(t) => Tag::Real(TagRepr::from(t)),
            _ => unreachable!(),
        };
        let _ = iter.next(); // Skip space after response
        response = if tag == Tag::ServerContinuation {
            ImapResponse::Continuation
        } else {
            match iter.next().unwrap() {
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
                _ => unreachable!(),
            }
        };
        let _ = iter.next();
        let msg = if let Some(_) = iter.peek() {
            Some(iter.collect())
        } else {
            None
        };

        Self { tag, response, msg }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ImapResponse {
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

#[derive(Debug, PartialEq, Eq)]
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
    Completed,
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

pub enum State {
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
    fn test_server_responses() {
        let rs = [
            "* CAPABILITY XAPPLEPUSHSERVICE IMAP4 IMAP4rev1 SASL-IR AUTH=ATOKEN AUTH=PLAIN",
            "A001 OK Completed",
            "* BYE",
            "A002 OK",
            "A003 NO [AUTHENTICATIONFAILED] Authentication Failed",
            "A003 OK user drsingh2518 logged in",
            "* 23 EXISTS",
        ];
        let sr = [
            ServerResponse {
                tag: Tag::ServerContinuation,
                response: ImapResponse::Capability,
                msg: Some(vec![
                    Token::Other(String::from("XAPPLEPUSHSERVICE")),
                    Token::SP,
                    Token::Other(String::from("IMAP4")),
                    Token::SP,
                    Token::IMAP4Rev1,
                    Token::SP,
                    Token::Other(String::from("SASL")),
                    Token::HYPHEN,
                    Token::Other(String::from("IR")),
                    Token::SP,
                    Token::Auth,
                    Token::EQUAL,
                    Token::Other(String::from("ATOKEN")),
                    Token::SP,
                    Token::Auth,
                    Token::EQUAL,
                    Token::Other(String::from("PLAIN")),
                ]),
            },
            ServerResponse {
                tag: Tag::Real(TagRepr::from("A001")),
                response: ImapResponse::Ok,
                msg: Some(vec![Token::Other(String::from("Completed"))]),
            },
            ServerResponse {
                tag: Tag::ServerContinuation,
                response: ImapResponse::Bye,
                msg: None,
            },
            ServerResponse {
                tag: Tag::Real(TagRepr::from("A002")),
                response: ImapResponse::Ok,
                msg: None,
            },
            ServerResponse {
                tag: Tag::Real(TagRepr::from("A003")),
                response: ImapResponse::No,
                msg: Some(vec![
                    Token::LBRACKET,
                    Token::AuthenticationFailed,
                    Token::RBRACKET,
                    Token::SP,
                    Token::Other(String::from("Authentication")),
                    Token::SP,
                    Token::Other(String::from("Failed")),
                ]),
            },
            ServerResponse {
                tag: Tag::Real(TagRepr::from("A003")),
                response: ImapResponse::Ok,
                msg: Some(vec![
                    Token::Other(String::from("user")),
                    Token::SP,
                    Token::Other(String::from("drsingh2518")),
                    Token::SP,
                    Token::Other(String::from("logged")),
                    Token::SP,
                    Token::Other(String::from("in")),
                ]),
            },
            ServerResponse {
                tag: Tag::ServerContinuation,
                response: ImapResponse::Size(23),
                msg: Some(vec![Token::Exists]),
            },
        ];

        for (i, s) in rs.iter().enumerate() {
            let response = ServerResponse::from(s);
            assert_eq!(response, sr[i])
        }
    }
}
