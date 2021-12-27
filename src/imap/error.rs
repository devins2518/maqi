use openssl::{error::ErrorStack, ssl::HandshakeError};
use std::{fmt::Display, io, net::TcpStream};

#[derive(Debug)]
pub enum ImapError {
    InvalidState,
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
            Self::InvalidState => format!("Invalid state"),
            Self::Bad => format!("Bad"),
            Self::Preauth => format!("Preauth"),
            Self::AuthenticationFailed => format!("Authentication failed"),
            Self::HandshakeError(e) => format!("Handshake error: {}", e),
            Self::SslError(e) => format!("SSL Error: {}", e),
            Self::IOError(e) => format!("IO Error: {}", e),
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
