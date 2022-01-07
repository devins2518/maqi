use super::Command;
use crate::imap::{
    client::State,
    error::{ImapError, ImapResult},
};
use std::borrow::Cow;

pub struct List<'a> {
    pub selection_options: Option<ListSelectionOptions>,
    pub reference: &'a str,
    pub mailbox: &'a str,
    pub return_options: Option<ListReturnOptions>,
}

impl<'a> List<'a> {
    pub fn simple(reference: &'a str, mailbox: &'a str) -> Self {
        Self {
            selection_options: None,
            reference,
            mailbox,
            return_options: None,
        }
    }
}

impl<'a> Command for List<'a> {
    fn send(&self) -> Cow<'static, str> {
        Cow::Owned(format!("LIST \"{}\" \"{}\"", self.reference, self.mailbox))
    }
    fn check(&self, state: &State) -> ImapResult<()> {
        match state {
            State::Authenticated | State::Selected => Ok(()),
            _ => Err(ImapError::InvalidState),
        }
    }
}

pub enum ListSelectionOptions {
    Subscribed,
    Remote,
    RecursiveMatch,
}

pub enum ListReturnOptions {
    Subscribed,
    Children,
    Status,
}
