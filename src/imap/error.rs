use openssl::{error::ErrorStack, ssl::HandshakeError};
use std::{borrow::Cow, fmt::Display, io, net::TcpStream};

#[derive(Debug)]
pub enum ImapError {
    InvalidState,
    No,
    Bad,
    Preauth,
    AuthenticationFailed,
    SslError(ErrorStack),
    HandshakeError(HandshakeError<TcpStream>),
    IOError(io::Error),
}

impl Display for ImapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidState => Cow::Borrowed("Invalid state"),
            Self::No => Cow::Borrowed("No"),
            Self::Bad => Cow::Borrowed("Bad"),
            Self::Preauth => Cow::Borrowed("Preauth"),
            Self::AuthenticationFailed => Cow::Borrowed("Authentication failed"),
            Self::HandshakeError(e) => Cow::Owned(format!("Handshake error: {}", e)),
            Self::SslError(e) => Cow::Owned(format!("SSL Error: {}", e)),
            Self::IOError(e) => Cow::Owned(format!("IO Error: {}", e)),
        };
        f.write_str(&s)
    }
}

impl From<HandshakeError<TcpStream>> for ImapError {
    fn from(e: HandshakeError<TcpStream>) -> Self {
        Self::HandshakeError(e)
    }
}

impl From<ErrorStack> for ImapError {
    fn from(e: ErrorStack) -> Self {
        Self::SslError(e)
    }
}

impl From<io::Error> for ImapError {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}

pub type ImapResult<T> = std::result::Result<T, ImapError>;
