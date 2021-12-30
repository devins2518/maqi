use super::Command;
use crate::imap::{
    client::State,
    error::{ImapError, ImapResult},
};
use std::borrow::Cow;

pub struct Login<'str> {
    user: &'str str,
    pass: &'str str,
}

impl<'str> Login<'str> {
    pub fn new(user: &'str str, pass: &'str str) -> Self {
        Self { user, pass }
    }
}

impl<'str> Command for Login<'str> {
    fn send(&self) -> Cow<'static, str> {
        Cow::Owned(format!("LOGIN {} {}", self.user, self.pass))
    }
    fn check(&self, state: &State) -> ImapResult<()> {
        match state {
            State::NotAuthenticated => Ok(()),
            _ => Err(ImapError::InvalidState),
        }
    }
}
