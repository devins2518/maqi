use openssl::error::ErrorStack;
use std::fmt::Display;

#[derive(Debug)]
pub enum ImapError {
    InvalidState,
    SslError(ErrorStack),
}

impl Display for ImapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidState => format!("Invalid state"),
            Self::SslError(e) => format!("SSL Error {}", e),
        };
        f.write_str(&s)
    }
}

pub type Result<T> = std::result::Result<T, ImapError>;
