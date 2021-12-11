use std::io;

pub enum ImapError {
    InvalidState,
    IO(io::Error),
}

pub type Result<T> = std::result::Result<T, ImapError>;
