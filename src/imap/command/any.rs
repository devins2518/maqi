use std::borrow::Cow;

use crate::imap::{client::State, error::ImapResult};

use super::Command;

pub struct Logout;

impl Command for Logout {
    fn send(&self) -> Cow<'static, str> {
        Cow::Borrowed("LOGOUT")
    }
    fn check(&self, _state: &State) -> ImapResult<()> {
        Ok(())
    }
}
