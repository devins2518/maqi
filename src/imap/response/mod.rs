use super::{error::ImapResult, tokens::Token};

pub trait Response {
    fn receive(tokens: &[Token]) -> Self;
    fn is_err(&self) -> ImapResult<()>;
}

mod authenticated;
mod not_authenticated;
mod result;

pub use authenticated::*;
pub use not_authenticated::*;
