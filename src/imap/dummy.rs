use super::{
    client::State,
    command::Command,
    error::ImapResult,
    response::{Response, Scan, Scanner},
};
use std::borrow::Cow;

pub struct Dummy;

impl Command for Dummy {
    fn send(&self) -> Cow<'static, str> {
        Cow::Borrowed("")
    }
    fn check(&self, _state: &State) -> ImapResult<()> {
        Ok(())
    }
}

impl Scan for Dummy {
    fn scan(s: &str, scanner: Scanner) -> Self {
        Self
    }
}

impl Response for Dummy {
    fn is_err(&self) -> ImapResult<()> {
        Ok(())
    }
}
