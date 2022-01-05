use super::{error::ImapResult, tokens::Token};
mod flags;
mod response;

pub trait Response {
    fn convert(tokens: &[Token]) -> Self;
    fn is_err(&self) -> ImapResult<()>;
}

mod authenticated;
mod not_authenticated;

pub use authenticated::*;
pub use not_authenticated::*;
