use super::{client::State, error::ImapResult};
use std::borrow::Cow;

pub trait Command {
    fn send(&self) -> Cow<'static, str>;
    fn check(&self, state: &State) -> ImapResult<()>;
}

mod any;
mod authenticated;
mod not_authenticated;

pub use any::*;
pub use authenticated::*;
pub use not_authenticated::*;
