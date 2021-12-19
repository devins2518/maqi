#![allow(unused)]

use std::fmt::{self, Display};
use std::str::{self, FromStr};

use super::error::ImapError;
use super::parser::Token;
use super::scanner::Scanner;

#[derive(Debug, PartialEq, Eq)]
enum Tag {
    Real(TagRepr),
    Continuation,
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

pub enum Command {
    // Any state
    Capability,
    Noop,
    Logout,
    // Not Auth state
    StartTLS,
    Authenticate,
    //    user    pass
    Login(String, String),
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

impl Command {
    pub fn check(&self, state: &State) -> Option<ImapError> {
        match (self, state) {
            (Command::Capability | Command::Noop | Command::Logout, _) => None,
            (
                Command::StartTLS | Command::Authenticate | Command::Login(_, _),
                State::NotAuthenticated,
            ) => None,
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
            ) => None,
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
            ) => None,
            _ => Some(ImapError::InvalidState),
        }
    }
}

impl Display for Command {
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
enum ImapResult {
    ImapOk(ImapOk),
    ImapError(ImapErrorInternal),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ServerResponse {
    tag: Tag,
    result: ImapResult,
    response_code: Option<ResponseCode>,
    msg: Option<String>,
}

impl<T> From<T> for ServerResponse
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let mut scanner = Scanner::new(s.as_ref());
        scanner.scan_tokens();
        let mut iter = scanner.tokens.iter();
        let tag = match iter.next().unwrap() {
            Token::STAR => Tag::Continuation,
            Token::PLUS => Tag::Continuation,
            Token::Other(t) => Tag::Real(TagRepr::from(t)),
            _ => unreachable!(),
        };
        let result = todo!();
        let response_code = todo!();
        let msg = todo!();

        Self {
            tag,
            result,
            response_code,
            msg,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ImapOk {
    Ok,
    Preauth,
    Bye,
}

#[derive(Debug, PartialEq, Eq)]
enum ImapErrorInternal {
    No,
    Bad,
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
}
