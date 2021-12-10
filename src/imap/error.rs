pub enum ImapError {
    InvalidState,
}

pub type Result<T> = std::result::Result<T, ImapError>;
