use super::{
    client::State, command::Command, error::ImapResult, response::Response, tokens::Token,
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

impl Response for Dummy {
    fn convert<'a>(_tokens: &[Token]) -> Self {
        Dummy
    }
    fn is_err(&self) -> ImapResult<()> {
        Ok(())
    }
}
